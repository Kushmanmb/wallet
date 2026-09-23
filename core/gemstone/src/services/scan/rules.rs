use primitives::{ScanAddressTarget, ScanTransactionPayload, TransactionInputType, TransactionPreloadInput};

use crate::models::gateway::GemTransactionPreloadInput;

fn requires_scan(input_type: &TransactionInputType) -> bool {
    match input_type {
        TransactionInputType::Transfer { .. } | TransactionInputType::Swap { .. } | TransactionInputType::TokenApprove { .. } | TransactionInputType::Generic { .. } | TransactionInputType::Payment { .. } => true,
        TransactionInputType::Deposit { .. }
        | TransactionInputType::Withdrawal { .. }
        | TransactionInputType::Stake { .. }
        | TransactionInputType::TransferNft { .. }
        | TransactionInputType::Account { .. }
        | TransactionInputType::Perpetual { .. }
        | TransactionInputType::Earn { .. } => false,
    }
}

pub fn transaction_payload(input: GemTransactionPreloadInput) -> Option<ScanTransactionPayload> {
    if !requires_scan(&input.input_type) {
        return None;
    }
    let input: TransactionPreloadInput = input.into();
    Some(ScanTransactionPayload {
        origin: ScanAddressTarget {
            asset_id: input.input_type.get_asset().id.clone(),
            address: input.sender_address.clone(),
        },
        target: scan_target(&input),
        website: input.get_website(),
        transaction_type: input.input_type.transaction_type(),
    })
}

fn scan_target(input: &TransactionPreloadInput) -> ScanAddressTarget {
    if let TransactionInputType::Swap { from_asset, swap_data, .. } = &input.input_type
        && !swap_data.data.to.is_empty()
    {
        return ScanAddressTarget {
            asset_id: from_asset.id.clone(),
            address: swap_data.data.to.clone(),
        };
    }
    ScanAddressTarget {
        asset_id: input.input_type.get_recipient_asset().id.clone(),
        address: input.destination_address.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::swap::ApprovalData;
    use primitives::{
        AccountDataType, ApplicationMetadata, Asset, ContractCallData, Delegation, DelegationValidator, EarnType, NFTAsset, PerpetualConfirmData, PerpetualDirection, PerpetualType, StakeType, SwapData, SwapQuoteData, TransactionType,
        TransferDataExtra,
    };

    #[test]
    fn test_a_swap_scans_the_contract_that_receives_the_funds() {
        let swap = GemTransactionPreloadInput {
            input_type: TransactionInputType::Swap {
                from_asset: Asset::mock_eth(),
                to_asset: Asset::mock_spl_token(),
                swap_data: SwapData {
                    data: SwapQuoteData {
                        to: "0xrouter".to_string(),
                        ..SwapQuoteData::mock()
                    },
                    ..SwapData::mock()
                },
            },
            sender_address: "sender".to_string(),
            destination_address: "own-solana-address".to_string(),
            references: vec![],
        };

        let target = transaction_payload(swap.clone()).unwrap().target;
        assert_eq!(target.address, "0xrouter", "the scan asks about the contract, not the address the user already owns");
        assert_eq!(target.asset_id, Asset::mock_eth().id, "the contract is on the chain the funds leave");

        let without_contract = transaction_payload(GemTransactionPreloadInput {
            input_type: TransactionInputType::Swap {
                from_asset: Asset::mock_eth(),
                to_asset: Asset::mock_spl_token(),
                swap_data: SwapData {
                    data: SwapQuoteData { to: String::new(), ..SwapQuoteData::mock() },
                    ..SwapData::mock()
                },
            },
            destination_address: "recipient".to_string(),
            ..swap
        })
        .unwrap()
        .target;
        assert_eq!(without_contract.address, "recipient", "a swap with no contract falls back to the recipient");
    }

    #[test]
    fn test_scan_payload_covers_every_input_type() {
        let swap = GemTransactionPreloadInput {
            input_type: TransactionInputType::Swap {
                from_asset: Asset::mock_sol(),
                to_asset: Asset::mock_spl_token(),
                swap_data: SwapData::mock(),
            },
            sender_address: "sender".to_string(),
            destination_address: "router".to_string(),
            references: vec![],
        };
        let payload = transaction_payload(swap).unwrap();
        assert_eq!(payload.transaction_type, TransactionType::Swap);
        assert_eq!(payload.origin.asset_id, Asset::mock_sol().id);
        assert_eq!(payload.website, None);

        let generic = GemTransactionPreloadInput {
            input_type: TransactionInputType::Generic {
                asset: Asset::mock_sol(),
                metadata: ApplicationMetadata::mock(),
                extra: TransferDataExtra::mock(),
            },
            sender_address: "sender".to_string(),
            destination_address: "contract".to_string(),
            references: vec![],
        };
        let payload = transaction_payload(generic).unwrap();
        assert_eq!(payload.transaction_type, TransferDataExtra::mock().transaction_type);
        assert_eq!(payload.website, Some(ApplicationMetadata::mock().url));
    }

    #[test]
    fn test_requires_scan() {
        for (input_type, should_scan) in [
            (TransactionInputType::Deposit { asset: Asset::mock_erc20() }, false),
            (
                TransactionInputType::Earn {
                    asset: Asset::mock_erc20(),
                    earn_type: EarnType::Deposit(DelegationValidator::mock()),
                    data: ContractCallData::mock(),
                },
                false,
            ),
            (TransactionInputType::Transfer { asset: Asset::mock_erc20() }, true),
            (TransactionInputType::mock_payment(Asset::mock_erc20(), TransferDataExtra::mock()), true),
            (TransactionInputType::Withdrawal { asset: Asset::mock_erc20() }, false),
            (
                TransactionInputType::Earn {
                    asset: Asset::mock_erc20(),
                    earn_type: EarnType::Withdraw(Delegation::mock()),
                    data: ContractCallData::mock(),
                },
                false,
            ),
            (
                TransactionInputType::Account {
                    asset: Asset::mock_erc20(),
                    account_type: AccountDataType::Activate,
                },
                false,
            ),
            (
                TransactionInputType::Stake {
                    asset: Asset::mock_sol(),
                    stake_type: StakeType::Rewards(vec![]),
                },
                false,
            ),
            (
                TransactionInputType::TransferNft {
                    asset: Asset::mock_eth(),
                    nft_asset: NFTAsset::mock(),
                },
                false,
            ),
            (
                TransactionInputType::Perpetual {
                    asset: Asset::mock_erc20(),
                    perpetual_type: PerpetualType::Open {
                        data: PerpetualConfirmData::mock(PerpetualDirection::Long, 0, None, None),
                    },
                },
                false,
            ),
            (
                TransactionInputType::Swap {
                    from_asset: Asset::mock_eth(),
                    to_asset: Asset::mock_erc20(),
                    swap_data: SwapData::mock(),
                },
                true,
            ),
            (
                TransactionInputType::TokenApprove {
                    asset: Asset::mock_erc20(),
                    approval_data: ApprovalData::mock(),
                },
                true,
            ),
            (
                TransactionInputType::Generic {
                    asset: Asset::mock_erc20(),
                    metadata: ApplicationMetadata::mock(),
                    extra: TransferDataExtra {
                        transaction_type: TransactionType::EarnDeposit,
                        ..TransferDataExtra::mock()
                    },
                },
                true,
            ),
        ] {
            assert_eq!(requires_scan(&input_type), should_scan);
        }
    }
}
