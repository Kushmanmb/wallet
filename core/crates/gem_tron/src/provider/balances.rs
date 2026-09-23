use async_trait::async_trait;
use chain_traits::{AccountBalances, ChainBalances};
use futures::future::join_all;
use std::error::Error;

use gem_client::Client;
use num_bigint::BigUint;
use primitives::{AssetBalance, AssetId, Chain, asset_balance::BalanceMetadata};

use crate::{
    address::TronAddress,
    models::TronAccount,
    provider::balances_mapper::{map_balance_staking, map_coin_balance, map_token_balance},
    rpc::{TronProvider, trongrid::mapper::TronGridMapper},
};

impl<C: Client> TronProvider<C> {
    async fn staking_balance(&self, account: &TronAccount) -> Result<AssetBalance, Box<dyn Error + Sync + Send>> {
        match &account.address {
            Some(address) => {
                let (reward, usage) = futures::try_join!(self.get_reward(address), self.get_account_usage(address))?;
                map_balance_staking(account, &reward, &usage)
            }
            None => Ok(AssetBalance::new_staking_with_metadata(
                AssetId::from_chain(Chain::Tron),
                BigUint::from(0u32),
                BigUint::from(0u32),
                BigUint::from(0u32),
                BalanceMetadata::default(),
            )),
        }
    }
}

#[async_trait]
impl<C: Client> ChainBalances for TronProvider<C> {
    async fn get_account_balances(&self, address: String, coin: bool, token_ids: Vec<String>) -> Result<AccountBalances, Box<dyn Error + Sync + Send>> {
        let (coin_balances, tokens) = futures::join!(
            async {
                match coin {
                    true => {
                        let account = self.get_account(&address).await?;
                        Ok::<_, Box<dyn Error + Sync + Send>>(Some((map_coin_balance(&account)?, self.staking_balance(&account).await?)))
                    }
                    false => Ok(None),
                }
            },
            async {
                match token_ids.is_empty() {
                    true => Ok(Vec::new()),
                    false => self.get_balance_tokens(address.clone(), token_ids).await,
                }
            },
        );
        let (coin, staking) = coin_balances?.unzip();
        Ok(AccountBalances { coin, staking, tokens: tokens? })
    }

    async fn get_balance_coin(&self, address: String) -> Result<AssetBalance, Box<dyn Error + Sync + Send>> {
        let account = self.get_account(&address).await?;
        map_coin_balance(&account)
    }

    async fn get_balance_tokens(&self, address: String, token_ids: Vec<String>) -> Result<Vec<AssetBalance>, Box<dyn Error + Sync + Send>> {
        let parameter = TronAddress::parse(&address)?.abi_address_parameter();

        let futures: Vec<_> = token_ids
            .into_iter()
            .map(|token_id| {
                let parameter = parameter.clone();
                async move {
                    let balance_hex = self.trigger_constant_contract(&token_id, "balanceOf(address)", &parameter).await?;
                    let asset_id = AssetId::from(self.get_chain(), Some(token_id));
                    map_token_balance(&balance_hex, asset_id)
                }
            })
            .collect();
        join_all(futures).await.into_iter().collect::<Result<Vec<_>, _>>()
    }

    async fn get_balance_staking(&self, address: String) -> Result<Option<AssetBalance>, Box<dyn Error + Sync + Send>> {
        let account = self.get_account(&address).await?;
        Ok(Some(self.staking_balance(&account).await?))
    }

    async fn get_balance_assets(&self, address: String) -> Result<Vec<AssetBalance>, Box<dyn Error + Send + Sync>> {
        Ok(self.get_indexer_accounts(&address).await?.into_iter().next().map(TronGridMapper::map_asset_balances).unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{TronAccountUsage, TronReward};
    use crate::rpc::client::TronClient;
    use gem_client::{ClientError, testkit::MockClient};
    use std::sync::{Arc, Mutex};

    const ADDRESS: &str = "TJoSEwEqt7cT3TUwmEoUYnYs5cZR3xSukM";

    fn provider(paths: Arc<Mutex<Vec<String>>>, reward_status: Option<u16>) -> TronProvider<MockClient> {
        let client = MockClient::new().with_post(move |path, _| {
            paths.lock().unwrap().push(path.to_string());
            match path {
                "/wallet/getaccount" => Ok(serde_json::to_vec(&TronAccount {
                    balance: Some(5_000_000),
                    ..TronAccount::mock(ADDRESS)
                })
                .unwrap()),
                "/wallet/getaccountresource" => Ok(serde_json::to_vec(&TronAccountUsage::mock(600, 0, 0)).unwrap()),
                "/wallet/getReward" => match reward_status {
                    Some(status) => Err(ClientError::Http { status, body: Vec::new() }),
                    None => Ok(serde_json::to_vec(&TronReward { reward: 7 }).unwrap()),
                },
                _ => Err(ClientError::Http { status: 404, body: Vec::new() }),
            }
        });
        TronProvider::new_rpc_only(TronClient::new(client))
    }

    #[tokio::test]
    async fn test_coin_and_staking_share_one_account_read() {
        let paths = Arc::new(Mutex::new(Vec::new()));
        let provider = provider(paths.clone(), None);

        let balances = provider.get_account_balances(ADDRESS.to_string(), true, Vec::new()).await.unwrap();

        assert_eq!(paths.lock().unwrap().iter().filter(|path| *path == "/wallet/getaccount").count(), 1);
        assert_eq!(balances.coin, Some(provider.get_balance_coin(ADDRESS.to_string()).await.unwrap()));
        assert_eq!(balances.staking, provider.get_balance_staking(ADDRESS.to_string()).await.unwrap());
        assert_eq!(balances.coin.unwrap().balance.available.to_string(), "5000000");
        assert!(balances.tokens.is_empty());
    }

    #[tokio::test]
    async fn test_a_failed_reward_read_fails_the_balances() {
        let provider = provider(Arc::new(Mutex::new(Vec::new())), Some(500));

        assert!(provider.get_account_balances(ADDRESS.to_string(), true, Vec::new()).await.is_err());
    }
}

#[cfg(all(test, feature = "chain_integration_tests"))]
mod chain_integration_tests {
    use super::*;
    use crate::provider::testkit::{TEST_ADDRESS, TEST_USDT_TOKEN_ID, create_test_client};
    use num_bigint::BigUint;
    use primitives::Chain;

    #[tokio::test]
    async fn test_tron_get_balance_coin() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client = create_test_client();
        let balance = client.get_balance_coin(TEST_ADDRESS.to_string()).await?;

        assert_eq!(balance.asset_id.chain, Chain::Tron);
        assert_eq!(balance.asset_id.token_id, None);
        assert!(balance.balance.available > BigUint::from(0u32));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_balance_tokens() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client = create_test_client();
        let token_ids = vec![TEST_USDT_TOKEN_ID.to_string()];

        let balances = client.get_balance_tokens(TEST_ADDRESS.to_string(), token_ids.clone()).await?;

        assert_eq!(balances.len(), token_ids.len());
        for (i, balance) in balances.iter().enumerate() {
            assert_eq!(balance.asset_id.chain, Chain::Tron);
            assert_eq!(balance.asset_id.token_id, Some(token_ids[i].clone()));
            assert!(balance.balance.available > BigUint::from(0u32));
        }

        assert!(balances.first().unwrap().balance.available > BigUint::from(0u32), "USDT balance should be greater than 0");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_balance_staking() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client = create_test_client();
        let balance = client.get_balance_staking(TEST_ADDRESS.to_string()).await?;

        let balance = balance.ok_or("Staking balance not found")?;

        assert_eq!(balance.asset_id.chain, Chain::Tron);
        assert_eq!(balance.asset_id.token_id, None);
        assert!(balance.balance.staked > BigUint::from(0u32));

        let metadata = balance.balance.metadata.as_ref().ok_or("Metadata not found")?;

        assert!(metadata.bandwidth_available > 0);
        assert!(metadata.bandwidth_total >= 600);

        //assert!(metadata.energy_available);
        //assert!(metadata.energy_total > 0);

        assert!(metadata.bandwidth_available <= metadata.bandwidth_total);
        assert!(metadata.energy_available <= metadata.energy_total);

        Ok(())
    }

    #[tokio::test]
    async fn test_tron_get_balance_assets() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client = create_test_client();
        let address = TEST_ADDRESS.to_string();
        let assets = client.get_balance_assets(address).await?;

        assert!(!assets.is_empty(), "TRON test address should have TRC20 tokens");

        for asset in &assets {
            assert_eq!(asset.asset_id.chain, Chain::Tron);
            assert!(asset.balance.available > BigUint::from(0u32));
            assert!(asset.asset_id.token_id.is_some());
        }
        Ok(())
    }
}
