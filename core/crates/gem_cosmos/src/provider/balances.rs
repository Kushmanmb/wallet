use async_trait::async_trait;
use chain_traits::{AccountBalances, ChainBalances};
use futures::try_join;
use num_bigint::BigUint;
use std::error::Error;

use gem_client::Client;
use primitives::{AssetBalance, AssetId};

use crate::{models::Balance, provider::balances_mapper, rpc::client::CosmosClient};

impl<C: Client> CosmosClient<C> {
    fn coin_balance(&self, balances: &[Balance]) -> Result<AssetBalance, Box<dyn Error + Sync + Send>> {
        let chain = self.get_chain().as_chain();
        let denom = chain.as_denom().ok_or("Chain does not have a denom")?;
        Ok(match balances.iter().find(|balance| balance.denom == denom) {
            Some(balance) => AssetBalance::new(chain.as_asset_id(), balance.amount.parse::<BigUint>().unwrap_or_default()),
            None => AssetBalance::new_zero_balance(chain.as_asset_id()),
        })
    }

    fn token_balances(&self, balances: &[Balance], token_ids: &[String]) -> Vec<AssetBalance> {
        token_ids
            .iter()
            .map(|token_id| {
                let asset_id = AssetId {
                    chain: self.get_chain().as_chain(),
                    token_id: Some(token_id.clone()),
                };
                match balances.iter().find(|balance| balance.denom == *token_id) {
                    Some(balance) => AssetBalance::new(asset_id, balance.amount.parse::<BigUint>().unwrap_or_default()),
                    None => AssetBalance::new_zero_balance(asset_id),
                }
            })
            .collect()
    }
}

#[async_trait]
impl<C: Client> ChainBalances for CosmosClient<C> {
    async fn get_account_balances(&self, address: String, coin: bool, token_ids: Vec<String>) -> Result<AccountBalances, Box<dyn Error + Sync + Send>> {
        let (balances, staking) = futures::join!(
            async {
                match coin || !token_ids.is_empty() {
                    true => self.get_balances(&address).await.map(Some),
                    false => Ok(None),
                }
            },
            async {
                match coin {
                    true => self.get_balance_staking(address.clone()).await,
                    false => Ok(None),
                }
            },
        );
        let balances = balances?.unwrap_or_default();
        Ok(AccountBalances {
            coin: coin.then(|| self.coin_balance(&balances)).transpose()?,
            staking: staking?,
            tokens: self.token_balances(&balances, &token_ids),
        })
    }

    async fn get_balance_coin(&self, address: String) -> Result<AssetBalance, Box<dyn Error + Sync + Send>> {
        self.coin_balance(&self.get_balances(&address).await?)
    }

    async fn get_balance_tokens(&self, address: String, token_ids: Vec<String>) -> Result<Vec<AssetBalance>, Box<dyn Error + Sync + Send>> {
        Ok(self.token_balances(&self.get_balances(&address).await?, &token_ids))
    }

    async fn get_balance_staking(&self, address: String) -> Result<Option<AssetBalance>, Box<dyn Error + Sync + Send>> {
        let cosmos_chain = self.get_chain();
        let chain = cosmos_chain.as_chain();
        if !chain.is_stake_supported() {
            return Ok(None);
        }
        let denom = chain.as_denom().ok_or("Chain does not have a denom")?;

        let (delegations, unbonding, rewards) = try_join!(self.get_delegations(&address), self.get_unbonding_delegations(&address), self.get_delegation_rewards(&address))?;

        Ok(Some(balances_mapper::map_balance_staking(delegations, unbonding, rewards, chain, denom)))
    }

    async fn get_balance_assets(&self, _address: String) -> Result<Vec<AssetBalance>, Box<dyn Error + Send + Sync>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gem_client::{ClientError, testkit::MockClient};
    use primitives::chain_cosmos::CosmosChain;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    fn staking_client(bank_requests: Arc<AtomicUsize>, rewards_status: Option<u16>) -> CosmosClient<MockClient> {
        let client = MockClient::new().with_get(move |path| {
            if path.starts_with("/cosmos/bank/v1beta1/balances/") {
                bank_requests.fetch_add(1, Ordering::Relaxed);
                return Ok(r#"{"balances":[{"denom":"uatom","amount":"42"},{"denom":"ibc/token","amount":"7"}],"pagination":{"next_key":null}}"#.as_bytes().to_vec());
            }
            if path.starts_with("/cosmos/staking/v1beta1/delegations/") {
                return Ok(include_bytes!("../../testdata/staking_delegations.json").to_vec());
            }
            if path.contains("/unbonding_delegations") {
                return Ok(include_bytes!("../../testdata/staking_unbonding_delegations.json").to_vec());
            }
            match rewards_status {
                Some(status) => Err(ClientError::Http { status, body: Vec::new() }),
                None => Ok(include_bytes!("../../testdata/staking_rewards.json").to_vec()),
            }
        });
        CosmosClient::new(CosmosChain::Cosmos, client)
    }

    #[tokio::test]
    async fn test_coin_and_tokens_share_one_bank_read() {
        let bank_requests = Arc::new(AtomicUsize::new(0));
        let client = staking_client(bank_requests.clone(), None);
        let token_ids = vec!["ibc/token".to_string()];

        let balances = client.get_account_balances("cosmos1".to_string(), true, token_ids.clone()).await.unwrap();

        assert_eq!(bank_requests.load(Ordering::Relaxed), 1);
        assert_eq!(balances.coin, Some(client.get_balance_coin("cosmos1".to_string()).await.unwrap()));
        assert_eq!(balances.tokens, client.get_balance_tokens("cosmos1".to_string(), token_ids).await.unwrap());
        assert_eq!(balances.staking, client.get_balance_staking("cosmos1".to_string()).await.unwrap());
        assert_eq!(balances.coin.unwrap().balance.available.to_string(), "42");
    }

    #[tokio::test]
    async fn test_a_failed_staking_read_fails_the_balances() {
        let client = staking_client(Arc::new(AtomicUsize::new(0)), Some(500));

        assert!(client.get_account_balances("cosmos1".to_string(), true, Vec::new()).await.is_err());
    }

    #[tokio::test]
    async fn test_a_denomination_that_disappeared_is_reported_as_zero() {
        let client = MockClient::new().with_get(|_| Ok(r#"{"balances":[{"denom":"still-here","amount":"7"}],"pagination":{"next_key":null}}"#.as_bytes().to_vec()));
        let client = CosmosClient::new(CosmosChain::Cosmos, client);

        let balances = client.get_balance_tokens("cosmos1".to_string(), vec!["still-here".to_string(), "spent".to_string()]).await.unwrap();

        assert_eq!(
            balances
                .iter()
                .map(|balance| (balance.asset_id.token_id.clone().unwrap_or_default(), balance.balance.available.to_string()))
                .collect::<Vec<_>>(),
            vec![("still-here".to_string(), "7".to_string()), ("spent".to_string(), "0".to_string())],
            "a requested denomination the node no longer lists is spent, not unchanged"
        );
    }
}

#[cfg(all(test, feature = "chain_integration_tests"))]
mod chain_integration_tests {
    use crate::provider::testkit::{TEST_ADDRESS, TEST_EMPTY_ADDRESS, create_cosmos_test_client};
    use chain_traits::ChainBalances;
    use num_bigint::BigUint;

    #[tokio::test]
    async fn test_cosmos_get_balance_coin() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client = create_cosmos_test_client();
        let address = TEST_ADDRESS.to_string();
        let balance = client.get_balance_coin(address).await?;

        println!("Balance: {:?} {}", balance.balance.available, balance.asset_id);

        assert!(balance.balance.available > BigUint::from(0u64));
        Ok(())
    }

    #[tokio::test]
    async fn test_cosmos_get_balance_coin_empty_address() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client = create_cosmos_test_client();
        let address = TEST_EMPTY_ADDRESS.to_string();
        let balance = client.get_balance_coin(address).await?;

        println!("Balance: {:?} {}", balance.balance.available, balance.asset_id);

        assert!(balance.balance.available == BigUint::from(0u64));
        Ok(())
    }

    #[tokio::test]
    async fn test_cosmos_get_balance_assets() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client = create_cosmos_test_client();
        let address = TEST_ADDRESS.to_string();
        let assets = client.get_balance_assets(address).await?;

        assert_eq!(assets.len(), 0);
        Ok(())
    }
}
