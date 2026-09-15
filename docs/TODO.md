# Open work

Every open item carries a stable id (V vocabulary, R rows, C composition, S sessions, B view boundary, F formatting, P parity, D decisions, O ownership, X platform, G guidance, T tests, L localization, N naming, PERF performance) and a size (**S**/**M**/**L**). Contracts are in [ARCHITECTURE.md](ARCHITECTURE.md) and [SERVICES.md](SERVICES.md). **Delete an item's line in the commit that lands it** — ids are never reused.

The goal is that Gemstone decides once and both clients read that decision. Track duplicated decisions and concrete performance work at their existing owners: shared rules and orchestration in Core; rendering, observation, scheduling, and localized formatting in the apps.

Keep each item independently reviewable. Shared decision changes land in Core and both apps; platform-only work stays on that platform. Regenerate only when shared interfaces or integration change, and run the applicable [Quality Checks](../skills/quality-checks.md). Verify affected primary-screen journeys under [Performance](PERFORMANCE.md). Remove replaced paths within the item's scope; do not bundle an unrelated row migration or product change.

This list was rebuilt on 2026-09-15 from scripted sweeps over the whole repo and widened the same day by a second, deeper pass. Each item names the file or symbol the sweep hit, so it can be confirmed before it is started; a sweep hit is a lead, not a verdict, and an item that turns out to be correct as written is closed by deleting its line with a one-line note in the commit.

These files were checked during the 2026-09-15 mapper sweep and need no change — each calls its module mapper or switches over an app type, not a Core one: iOS `NFT/CollectibleViewModel`, `Settings/GemAddNodeFailure+Settings`, `Swap/SwapSlippageViewModel`, `Swap/Views/SwapDetailsView`, `Transfer/Types/ConfirmInfoSheetBuilder`, `Transfer/ConfirmRecipientViewModel`, `Transfer/RecipientSceneViewModel`; Android `earn`, `import_wallet`, `perpetual`.

## 2. Lists and screens without a row record

Copy: [`GemAssetRow`](../core/gemstone/src/services/assets/model.rs) → [iOS](../ios/Packages/PrimitivesComponents/Sources/ViewModels/ListAssetItemViewModel.swift), [Android](../android/gemcore/src/main/kotlin/com/gemwallet/android/domains/asset/aggregates/AssetInfoDataAggregate.kt). Each of these iOS view models composes four or more user-facing strings and holds no Core record; the count in brackets is how many. Land each with its Android mirror.

- **R36** **M** `Transfer/Sources/ViewModels/RecipientSceneViewModel.swift` (9) against the Android `recipient` screens.
- **R37** **M** `WalletConnector/.../ViewModels/ConnectionProposalViewModel.swift` (8) against Android `ProposalSceneViewModel`.
- **R39** **M** `Settings/Sources/ChainSettings/ViewModels/AddNodeSceneViewModel.swift` (6) against Android `AddNodeViewModel`.
- **R41** **M** `Contacts/Sources/ViewModels/ManageContactViewModel.swift` (6) against Android `ManageContactViewModel`.
- **R42** **M** `Assets/Sources/ViewModels/AddAssetSceneViewModel.swift` (6) against Android `AddAssetViewModel`.
- **R44** **S** `Transfer/Sources/ViewModels/ConfirmRecipientViewModel.swift` (5).
- **R46** **M** `Onboarding/Sources/ViewModels/ImportWalletSceneViewModel.swift` (5) against Android `ImportViewModel`.
- **R47** **S** `Transfer/Sources/ViewModels/AmountSceneViewModel.swift` (4) — balance line, reserved-fee line, max and continue.
- **R48** **S** `Transfer/Sources/ViewModels/AmountPerpetualViewModel.swift` (4).
- **R49** **S** `Settings/Sources/ChainSettings/ViewModels/AddNodeResultViewModel.swift` (4).
- **R50** **S** `QRScanner/Sources/ViewModels/QRScannerErrorViewModel.swift` (4) — check the scanner divergence note in SERVICES.md first; only the non-platform half moves.
- **R51** **M** `Perpetuals/Sources/ViewModels/PerpetualsSceneViewModel.swift` (4) against Android `PerpetualMarketViewModel`.
- **R53** **S** `Contacts/Sources/ViewModels/ManageContactAddressViewModel.swift` (4).

## 3. Views that decide

- **B9** **S** Re-run the § 5 grep over the 12 iOS scene files that name `Gemstone` once G2 lands, and move anything that decides into its model. Most are row-key dispatch and stay.
- **B10** **M** The same pass over the 94 Android `presents/` composables that name `uniffi.gemstone`.

Rejected: `import_wallet/views/ImportScreen.kt` switches over a `Throwable` and delegates its Core arm to the module mapper already.

## 4. Ownership

- **O13** **S** `Settings/Sources/Settings/ViewModels/PreferencesViewModel.swift` holds both `GemPreferencesServiceProtocol` and `GemSettingsServiceProtocol`. [§ 7](ARCHITECTURE.md#7-at-most-one-core-service-on-ios-narrow-cases-on-android) allows one: either the settings service answers the perpetual writes, or the preferences service answers the rows.
- **O14** **M** `Onboarding/Sources/ViewModels/ImportWalletViewModel.swift` holds three — wallet, name and chain. The name service is the documented shared-component dependency; decide whether the chain service is the dependency-free one it may build itself.
- **O15** **S** `Features/LockManager/Sources/ViewModels/LockSceneViewModel.swift` constructs `GemSecurityService` at the call site, against [§ 8](ARCHITECTURE.md#8-services-are-injected-never-constructed-at-a-call-site).
- **O16** **S** Core types named by neither app — perpetual: `GemPerpetualCloseInput`, `GemPerpetualOrderAction`, `GemPerpetualOrderInput`, `GemPerpetualRefreshStep`, `GemPerpetualSocketUpdate`. Confirm each is not a nested field before removing it.
- **O17** **S** Same, EIP-712: `GemEIP712Message`, `GemEIP712Section`, `GemEIP712Value`, `GemEIP712ValueType`.
- **O18** **S** Same, amount: `GemAmountEarnType`, `GemAmountMaxEntry`, `GemAmountStakeType`.
- **O19** **S** Same, chart and portfolio: `GemChartCurrent`, `GemChartViewState`, `GemPortfolioValues`.
- **O20** **M** Same, transaction and balance: `GemTransactionData`, `GemTransactionLoadInput`, `GemTransactionPreloadInput`, `GemTransactionStateResult`, `GemTransferOutput`, `GemBalanceUpdate`, `GemBalanceUpdateType`.
- **O21** **M** Same, the remaining 17: `GemAddAssetViewState`, `GemAppStartStep`, `GemAssetRefreshStep`, `GemAssetSectionIds`, `GemAutocloseViewState`, `GemCollectibleAttribute`, `GemContactRow`, `GemDeviceStreamRequest`, `GemDiscoveryStep`, `GemFiatTransactionStatus`, `GemPercentageFormat`, `GemPostProcessingFailure`, `GemPostProcessingStep`, `GemSelectRowAction`, `GemSimulationChange`, `GemStoredSecretMigration`, `GemSupportChatGroup`.

## 5. Screens that may want a session

Decide before building; a read-only screen is a row, not a session.

- **S17** **M** `Settings/Sources/ChainSettings/ViewModels/ChainSettingsSceneViewModel.swift` holds five mutable fields and loads node status concurrently after the node list. Android's `NetworksViewModel` carries the same state plus a refresh nonce it invented to order the two loads.
- **S18** **S** `Transactions/Sources/ViewModels/TransactionsFilterViewModel.swift` against the Android filter sheet.

## 6. Parity

- **P14** **S** Both apps carry the same deferred feature with the same wording: `ios/Packages/PrimitivesComponents/Sources/Scenes/NetworkFeeScene.swift` and `android/features/confirm/.../components/FeeDetails.kt` both say "present fee items in a separate section when nonempty". Build it once in Core and render it twice, or drop both notes.

## 7. Platform

### Hardcoded dp (Android rule: theme constants only)

Re-checked on 2026-09-15 by separating a `\d+.dp` written at a call site from one written as a named constant: **Android has none of the former**. All 58 remaining sites are `private val name = N.dp` declarations, which is the pattern the codebase already uses for a component's own dimensions, and only a handful of those carried a value the theme also names for the same kind of thing — those now read from the theme. What is left is a QR finder's 25 dp corner, a 42 dp search bar, a 296 dp slippage sheet and the like: dimensions with no theme equivalent, each named where it is used. X23–X29 are closed with no change.

### Swallowed errors

- **X30** **S** Four empty `catch`/`onFailure` blocks in `android/ui`. Each drops an error with no log and no state.
- **X31** **S** The same in `android/app`, `features/settings/price_alerts/presents` and `features/transfer_amount/presents`.

### Deferred notes still in the code

- **X32** **S** `ios/Features/Transactions/Sources/Scenes/TransactionScene.swift` — the button corner radius is marked unresolved.
- **X33** **S** `ios/Packages/Components/Sources/TextFields/CurrencyTextField.swift` — a fixed height works around a filed Apple bug; re-check whether it still reproduces.
- **X34** **S** `ios/Packages/Components/Sources/ViewModifiers/NavigationStackModifier.swift` — a `Binding` extension is parked in the wrong file.
- **X35** **S** `ios/Packages/Gemstone/Package.swift` pins Swift 5 language mode until `GemstoneFFI` is Swift 6 clean. Re-check against the current toolchain.
- **X36** **S** Two dated iOS removals: `GemstoneServices/Sources/Keystore/LocalKeystore.swift` and `Store/Sources/DB.swift` are both marked for 2026. Confirm the install base and delete, or re-date them with the reason.
- **X37** **S** `core/bin/generate/src/main.rs` takes a value it should read from the command line.
- **X38** **M** `core/crates/gem_auth/src/signature.rs` verifies one chain type and answers `false` for the rest. SERVICES.md records this as fail-closed by construction; either widen it or delete the note that says it is temporary.
- **X39** **M** `core/crates/nft/src/providers/ton/verified.rs` hardcodes the verified-collection allowlist. Backend gap; track it where the backend work lives or close the note.
- **X40** **S** `core/apps/api/src/devices/mod.rs` — the legacy singular route is due after 2026-11-15.
- **X41** **S** `core/crates/primitives/src/device_locale.rs` — legacy locale compatibility, removable once clients send `DeviceLocale`.
- **X42** **S** `core/crates/primitives/src/swap_provider.rs` — `CetusAggregator` goes when no stored swap carries it. Add the query that proves it.

### Core hardening

- **X43** **M** `core/crates/primitives/src/explorers/blockchair.rs` has 23 `unwrap`/`expect` calls outside tests, against [defensive programming](../core/AGENTS.md).
- **X44** **M** `core/crates/gem_hypercore/src/core/hypercore.rs` has 14.
- **X45** **M** Audit the remaining production `unwrap`/`expect` sites in `core/` and either prove each infallible in a comment-free way (a type change) or return an error.

### Files that have outgrown one module

- **X46** **M** `core/gemstone/src/services/confirm/rules.rs` — 1387 lines.
- **X47** **M** `core/gemstone/src/services/perpetual/rules.rs` — 1373.
- **X48** **M** `core/gemstone/src/services/transactions/rules.rs` — 1266.
- **X49** **M** `core/gemstone/src/services/stake/rules.rs` — 1265.
- **X50** **M** `core/gemstone/src/services/amount/rules.rs` — 1251.
- **X51** **M** `core/gemstone/src/services/transfer/rules.rs` — 1194.
- **X52** **M** `core/gemstone/src/services/assets/rules.rs` — 1168.
- **X53** **M** `ios/Packages/Keychain/Sources/Types/Status.swift` — 1241 lines of status mapping.
- **X54** **M** `ios/Gem/Services/ViewModelFactory.swift` — 777 lines and every screen's constructor.

### View models with no test

The logic weight in brackets is methods plus computed properties. 95 of 159 iOS and 45 of 65 Android feature view models have no test file; these are the heaviest.

- **X55** **M** iOS `Settings/.../RewardsViewModel.swift` (53).
- **X56** **M** iOS `Perpetuals/.../PerpetualSceneViewModel.swift` (45).
- **X57** **S** iOS `Transfer/.../ReceiveViewModel.swift` (26).
- **X58** **S** iOS `Settings/.../ChainSettingsSceneViewModel.swift` (23) and `AddNodeSceneViewModel.swift` (17).
- **X59** **S** iOS `WalletTab/.../NetworkAssetsSceneViewModel.swift` (22) and `AssetsResultsSceneViewModel.swift` (20).
- **X60** **S** iOS `WalletConnector/.../ConnectionsViewModel.swift` (21) — its old test was deleted when the sections moved to Core; the wiring still has none.
- **X61** **S** iOS `ManageWallets/.../WalletIDetailViewModel.swift` (21).
- **X62** **S** iOS `PriceAlerts/.../SetPriceAlertViewModel.swift` (20).
- **X63** **S** iOS `Settings/.../PreferencesViewModel.swift` (18), `AboutUsViewModel.swift` (18), `SecurityViewModel.swift` (17) — all three moved to Core rows with no app test.
- **X64** **S** iOS `Support/.../SupportChatSceneViewModel.swift` (16) and `SupportMessageBubbleViewModel.swift` (19).
- **X65** **S** Android `settings/networks/viewmodels` — `NetworksViewModel.kt`, `AddNodeViewModel.kt`, `ServiceStatusViewModel.kt`.
- **X66** **S** Android `bridge/viewmodels` — `WCRequestViewModel.kt`, `WCAuthViewModel.kt`.

## 8. Performance

Measure before and after; these are code-backed candidates, not measured regressions.

- **PERF19** **S** Android `NetworksViewModel` calls `service.nodeRow(...)` for every node on every state emission, including the per-node status updates that arrive one at a time.

## 9. Core decides it, only one app reads it

Each of these is an `#[uniffi::export]` the sweep found named in one app and in neither the other app's Kotlin nor its Swift. The first run skipped every iOS file whose name ends `+Gemstone.swift`; the list below is the corrected one.

**Read the preamble before starting one.** The sweep measures which *export* each app names, which is not the same as which app *owns* the decision. Eight were checked on 2026-09-15 and closed with no change, because the app that never calls the export still reads the same Core answer through a different one: Android reads the abbreviation cutoff through `GemValueStyle.abbreviates`, the price-alert kind through the aggregate's `kind.groupsByAsset()`, the dApp name through `GemConfirmDestination.Generic` and `connection_row`, and whether to show a memo through the confirm row set; the developer screen simply offers fewer actions than the iOS one. So for each item below, first find what the other app renders for the same thing — only if it computes the answer itself is there work here.

### Android does not read an iOS-read decision

- **P24** **S** `assets/config.rs` `default_token_rank` and `matching_assets`.
- **P27** **S** `confirm/error.rs` `has_info_sheet` — Android decides which confirm errors open a sheet.
- **P28** **S** `support/mod.rs` `image_file`.
- **P29** **S** `security/rules.rs` `is_cancelled`.
- **P31** **S** `node/model.rs` `latest_block`.
- **P32** **M** `perpetual/mod.rs` `sync_markets_if_needed`, `sync_markets`, `sync_current_positions`, `clear_markets`, `markets_updated_at` — Android schedules the same five itself.
- **P33** **S** `swap/rules.rs` `minimum_amount`.
- **P34** **S** `assets/add.rs` `on_chain`.
- **P35** **S** `perpetual/autoclose.rs` `on_submit_attempt`.
- **P36** **S** `amount/model.rs` `prefilled_amount`.
- **P37** **S** `stake/rules.rs` `recommended_validators`.
- **P38** **S** `avatar/mod.rs` `set_image` and `remove_image`.
- **P39** **S** `wallet_preferences/mod.rs` `reset_transactions_timestamp`.
- **P40** **M** `wallet/mod.rs` `setup_chains` — Android runs its own chain setup after import.
- **P41** **S** `wallet_session/mod.rs` `shows_rewards`.
- **P42** **M** `message/signer.rs` `sign_with_keystore` — security-critical; confirm what Android signs with before changing anything.
- **P43** **S** `swap/session.rs` `swap_error_display` and `swap/model.rs` `swap_quote`.
- **P44** **S** `device/mod.rs` `synchronize`.
- **P62** **S** `transfer/model.rs` `named`.

### iOS does not read an Android-read decision

- **P45** **M** `wallet_connect/mod.rs` `authentication_accounts`, `authentication_chain_ids`, `authentication_methods` — iOS builds the SIWE authentication payload itself.
- **P47** **S** `wallet_connect/mod.rs` `is_origin_rejected` and `user_rejected_error`.
- **P48** **S** `wallet_connect/mod.rs` `message_preview` and `message_address_names`.
- **P49** **S** `chain/mod.rs` `chain_from_caip2`.
- **P50** **S** `payment.rs` `decode_url` — iOS decodes payment URLs through a different entry point; confirm they agree.
- **P51** **S** `assets/config.rs` `default_asset_basic`.
- **P52** **S** `mnemonic.rs` `find_invalid_words` — iOS surfaces invalid words only through the import error.
- **P53** **S** `app_update/mod.rs` `is_version_higher` — iOS compares versions inside `newest`; pick one.
- **P54** **S** `transactions/mod.rs` `listed_asset_rank`.
- **P55** **S** `security/rules.rs` `retry_delay_milliseconds` — iOS retries biometry prompts on its own schedule. (iOS does read `lock_periods` and `lock_period_from_minutes` through `LockPeriod+Gemstone.swift`.)
- **P56** **M** `wallet/mod.rs` `migrate_to_shared_password` and `preview_import` — security-critical keystore paths; read [KEYSTORE_V4](KEYSTORE_V4.md) first.
- **P57** **S** `swap/slippage.rs` `on_auto`.
- **P58** **S** `swap/session.rs` — the two apps drive the same Core session through different transitions: Android calls `on_quote_invalidated` and `on_refresh_requested`, iOS calls `on_refresh_resumed` and `on_request_changed`. One set should cover both.
- **P59** **S** `nft/mod.rs` `receive_accounts`.
- **P60** **S** `assets/mod.rs` `sync_assets`.
- **P61** **S** `perpetual/mod.rs` `update_balance`.

## 10. Exports no app calls at all

151 exported functions have no caller in either app. Most are trait methods the apps implement rather than call, or Core-internal. These are the ones whose names read like a decision an app should be reading.

- **O22** **S** `confirm/rules.rs` `balance_change_sign`, `build_metadata`, `is_insufficient_network_fee`, `selectable_fee_assets`, `preload_simulation`.
- **O23** **S** `amount/rules.rs` `amount_title`, `stake_amount_type`, `transfer_amount_type`, `transfer_display_asset`, `transfer_prefilled_amount`, `plain_number`, `value_from_input`, `sanitize_number_input`.
- **O24** **S** `stake/rules.rs` `apply_validator_state`, `earn_validators`, `merge_validators`, `missing_validators`, `stale_delegation_ids`, `stale_validator_ids`, `validator_address_names`, `shows_stake_balance`.
- **O25** **S** `price_alert/rules.rs` `displayed_price_alert_ids`, `price_alert_row`, `price_alert_toggle`, `reconcile`.
- **O26** **S** `simulation.rs` `payload_fields`, `shows_header`, `warning_rows`.
- **O27** **S** `explorer/mod.rs` — seven getters (`get_address_url`, `get_token_url`, `get_nft_url`, `get_validator_url`, `get_transaction_link`, `get_explorer_name`, `get_explorers`) with no app caller.
- **O28** **M** `preferences/mod.rs` and `wallet_preferences/mod.rs` — roughly thirty paired getters/setters with no app caller. Decide per pair whether the app should be reading it or the pair should go.
- **O29** **S** `node/mod.rs` `sorted_nodes` and `node_url`, `node/settings.rs` `can_delete_node`.

## 11. Missing tests

### Core behaviour with no test

Thirty gemstone files carry `pub fn`s and no `#[cfg(test)]`, but most of them declare records or forward to a `rules.rs` that is already tested — `GemStakeAmountInput`'s four methods and `candlestick_header` are both covered from their rules module. What is left is the orchestration below: each named function has a body of its own and no test anywhere in the crate calls it. Several need a store or gateway mock first; `services/*/testkit.rs` is the pattern.

- **T1** **M** `services/perpetual/mod.rs` — `on_socket_message` (27 lines, five socket message kinds, each writing positions, balances or prices), plus `refresh`, `sync_enablement`, `sync_markets_if_needed`, `sync_current_positions`, `account_mode`.
- **T2** **M** `services/assets/details.rs` `refresh` (26 lines) — the concurrent detail load and its per-step failures.
- **T4** **S** `services/wallet_home/mod.rs` `refresh`.
- **T5** **M** `services/app_start/mod.rs` `setup_wallets`.
- **T7** **S** `services/asset_discovery/mod.rs` `discover`.
- **T8** **M** `services/rewards/mod.rs` — `create_referral`, `use_referral_code`, `redeem`. Money paths with no test.
- **T10** **S** `services/fiat/quote.rs` `quote_url` — it enables the asset as a side effect of fetching the URL.

### iOS view models with no test file

130 across `Features`, `Packages`, `Gem` and the widget — the wider count that X55–X64's preamble narrows to feature modules. Grouped by module so each item is one test target's worth of work; X55–X64 already name the heaviest.

- **T21** **M** `Features/Transactions` — 16 view models, none tested.
- **T22** **M** `Packages/PrimitivesComponents` — 24 view models, none tested.
- **T23** **S** `Features/Perpetuals` — 6.
- **T24** **S** `Features/Onboarding` — 9.
- **T25** **S** `Features/NFT` — 4, and `Features/Contacts` — 1.
- **T26** **S** `Features/Swap` — 3 (`SwapProvidersViewModel`, `SwapTokenViewModel`, `SwapPairSelectorViewModel`).
- **T27** **S** `Features/Transfer` — 5 (`AmountEarnViewModel`, `PerpetualModifyViewModel`, `KeystoreAuthenticationViewModel`, `ReceiveNetworkSelectorViewModel`, `ReceiveViewModel`).
- **T28** **S** `Gem/ViewModels` — `RootSceneViewModel`, `MainTabViewModel`, `ScanReceiveViewModel`, `ScanReceiveModeViewModel`.
- **T29** **S** `GemPriceWidget` — `PriceWidgetViewModel`, `CoinPriceRowViewModel`.

### Android view models with no test

49 of them; X65 and X66 already name five.

- **T30** **M** `features/bridge/viewmodels` — the remaining three (`ProposalSceneViewModel`, `ConnectionViewModel`, `ConnectionsViewModel`).
- **T31** **M** `features/asset_select/viewmodels` — all five.
- **T32** **S** `features/settings/contacts/viewmodels` — all three.
- **T33** **S** `features/perpetual/viewmodels` — `PerpetualDetailsViewModel`, `AutocloseViewModel`, `PerpetualsPreviewViewModel`.
- **T34** **S** `features/wallet-details/viewmodels` — all three.
- **T35** **S** `features/activities/viewmodels` — `TransactionsViewModel`, `TransactionDetailsViewModel`.
- **T36** **S** `features/receive/viewmodels` — both.
- **T37** **S** `features/confirm/viewmodels` `ConfirmViewModel.kt` (412 lines) and `presents/components/NetworkFeeCustomViewModel.kt`.
- **T38** **S** `app` — `MainViewModel`, `AppViewModel`, `MainScreenViewModel`, `SetupWalletViewModel`.
- **T39** **S** `features/settings/security/viewmodels` `SecurityViewModel.kt` — authentication toggles with no test.

## 12. Localization hygiene

### Keeping both apps on the same key for the same Core variant

Twenty English strings exist under two keys — `wallet_send` / `transfer_send_title`, `wallet_stake` / `transfer_stake_title`, `wallet_import_address_field` / `transfer_recipient_address_field` and seventeen more. Checked on 2026-09-15: each pair carries its own context comment in `localization/app/en.ftl` and names a different place in the product, so the pairs are deliberate and must not be merged — a language that needs a different form for a label and a screen title depends on them being separate.

What went wrong in V40 and V41 was not the pair; it was one app's mapper reaching for the other half of a pair. That is only visible by comparing the two apps.

### Keys with no reader

- **L12** **S** Thirteen keys in `localization/app/en.ftl` have no `R.string.` reader in Kotlin and no `Localized.` reader in Swift: `common_no_thanks`, `transfer_amount_title`, `errors_transfer`, `errors_decoding`, `errors_connections_invalid_send_parameters`, `errors_connections_invalid_sign_parameters`, `errors_connections_unsupported_method`, `errors_token_unable_fetch_token_information`, `update_app_downloading`, `banner_enable_notifications_title`, `banner_enable_notifications_description`, `perpetuals_empty_state_no_markets`, `confirm_fee_error`. Deleting a key retires it in every locale, so confirm each against the generated accessors first.
- **L13** **S** `camera_permission_request_camera` and `notifications_permission_request_notification` have no Kotlin or Swift reader either, but permission copy is often referenced from a manifest or plist — find the reader or delete the pair. (`application_name` is read by `AndroidManifest.xml` and stays.)

## 13. Names the guides forbid

- **N3** **S** `ios/Packages/Store/Sources/Requests` — `applyFilter`, `applyFilters`, `fetchAllAssetRecordsRequest`.
- **N4** **S** `ios/Packages/Store/Sources/Stores/StoreManager.swift` and `ios/Packages/Primitives/Sources/Extensions/NSFileManager+Primitives.swift`.
- **N5** **S** `ios/Features/LockManager` — the module, `LockWindowManager.swift` and its view modifier.
- **N6** **S** `ios/GemPriceWidget/Services/WidgetPriceService.swift` — `fetchTopCoinPrices`, `fetchRemoteImage`.
- **N9** **S** Core `resolve_*` in the swapper and portfolio crates — `resolve_token`, `resolve_asset_id`, `resolve_deposit_mode`, `resolve_quote_waiting_time`, `resolve_app_fees`, `resolve_asset`, `resolve_primary`, `resolve_expire_at`.

## 14. Documentation that has fallen behind

- **G4** **M** Twenty Gemstone services are not named anywhere in [SERVICES.md](SERVICES.md): `GemAppStartService`, `GemAssetDiscoveryService`, `GemAssetsService`, `GemAuthService`, `GemConfigService`, `GemConnectionService`, `GemDeviceKeyService`, `GemExplorerService`, `GemFiatService`, `GemPerpetualStreamService`, `GemPriceService`, `GemPushNotificationService`, `GemScanService`, `GemSearchService`, `GemSecurityService`, `GemSimulationService`, `GemStreamSubscriptionService`, `GemSubscriptionService`, `GemSwapService`, `GemWalletConfigurationService`.
- **G5** **S** The screen-service map in SERVICES.md predates the row records added since; walk it against the current `ViewModelFactory.swift` and `di/` modules.
- **G6** **S** ARCHITECTURE.md's implementation index still points at examples that moved during the row migration; re-resolve every link.
- **G7** **S** [PERFORMANCE.md](PERFORMANCE.md) records budgets for screens whose data path moved to Core sessions; restate each budget against the current path or mark it unmeasured.

## 15. More platform work

### iOS errors thrown away

Re-checked on 2026-09-15. `Store/Migrations.swift` holds 88 of the 174 sites and they are the idempotent-migration idiom — `try? db.alter` for a column that may already exist, `try? db.drop` for a table that may not. Rewriting those against a shipped wallet database is a data-loss risk with no defect behind it, so they stay. `clearChainData` was the one that was wrong: it deleted a removed chain's rows from seven tables with `try?`, so a locked table or a constraint left the rows behind silently. It now asks `tableExists` the way `clearTables` beside it already did and lets a real error through.

`WalletIdMigration.swift` had the same shape and the same one real problem: it rewrote `walletId` across ten child tables and deleted a wallet's child rows with `try?`, so a failure on any one of them left the wallet's data pointing at an id that no longer exists, and `cleanupOrphanedRecords` then swallowed the foreign-key check as well. All three now skip a table that does not exist and let a real failure roll the migration back.

Checked and kept: `LocalKeystore.findV3File` scans a directory and `try?` is how an unreadable file is skipped; `SwapSceneViewModel.currentInput` returns nil because "no complete input yet" is what its errors mean; `NavigationPathState` and `NavigationHandler` decode a path or a wallet id where nil is a real answer.

The rest are one or two per file and each needs reading on its own.

- **X71** **S** `ios/Packages/FeatureServices/WalletConnectorService/WalletConnectorService.swift` — 3.
- **X73** **S** `ios/Packages/Primitives/Sources/AnyCodableValue.swift` — 11, plus `AssetId.swift` (2) and `Store/Extensions/AnyCodableValue+Store.swift` (2).
- **X74** **S** The remaining 24 first-party files with one or two sites each.

### Logging left in shipping paths

Re-checked on 2026-09-15 by separating what actually ships: 116 of the 122 iOS sites are `debugLog`, which compiles to nothing outside `DEBUG`, and the remaining six `print` calls are inside a `#Preview` or a macOS-only availability note. On Android 60 of the 69 are `Log.e` on a real failure. The seven `Log.d` that shipped are gone — they were logging WalletConnect request method, chain and id, the whole stream payload, and connection transitions into logcat on a release build — and the two migration errors that were logged at debug level now log as errors. X75–X83 are closed.

### iOS layout numbers outside Style

Re-checked on 2026-09-15 the way the Android dp items were: most of the 31 hits are inside a `#Preview` or are a named configuration field (`QRScannerDisplayConfiguration.default`), which is the pattern. What was real: a segmented picker width repeated in three scenes and a chart height repeated in two now read `Sizing.picker.segmentedWidth` and `Sizing.chart.height` (Android has named the chart height all along), and the `spacing: 0` / `cornerRadius: 10` / `spacing: 24` call sites read from `Spacing`. X84–X88 are closed.

### Files that have outgrown one module (second pass)

- **X89** **M** `android/gemcore/.../ext/RemoteTypeMappers.kt` — 2278 lines of hand-written remote mappers. Check how many the generator could emit.
- **X90** **M** `core/gemstone/src/models/remote_types.rs` — 1929 lines.
- **X91** **M** `core/crates/primitives/src/chain_config.rs` — 1388.
- **X92** **M** `core/gemstone/src/message/signer.rs` — 771.
- **X93** **M** `ios/Packages/GemstonePrimitives/TestKit/GemServiceMocks.swift` — 1047 lines of mocks in one file.
- **X94** **M** `ios/Packages/PrimitivesComponents/Sources/Extensions/Gemstone+Localized.swift` — 499 lines; the shared mapper is becoming the place every module's leftovers land.
- **X95** **M** `android/ui/.../components/list_head/AmountListHead.kt` (516) and `chart/GemCandlestickChart.kt` (470).
- **X96** **S** `android/data/services/store/.../database/di/Migration_71_72.kt` (546) and `Migration_41_42.kt` (406) — confirm both are still reachable from the oldest supported schema.
- **X97** **M** `ios/Gem/Services/ServicesFactory.swift` (469) alongside X54's `ViewModelFactory.swift`.

### App-side twins of Core types

[No hand-written twins](ARCHITECTURE.md): an FFI-only type is used as the uniffi type; a twin is only for a type an app persists. Each of the three below has the same cases and the same payload types as its Core counterpart and is never written to storage.

Checked and kept: `KeystoreAuthentication` and `LockPeriod` are both written to the keychain by raw value, which the rule allows; `AmountType` carries a recipient its Core namesake does not; `SelectAssetType` and `PaymentDestination` are navigation types carrying app payloads and already map to Core through `flowType`.

### Core hardening (second pass)

- **X105** **S** `core/gemstone/src/services/transaction_state/tracker.rs` — 5 `unwrap`/`expect` outside tests on the transaction state path.
- **X106** **S** `core/gemstone/src/gateway/chain_factory.rs` (3), `device.rs` (2), `signer/chain.rs` (1), `block_explorer/explorer.rs` (1).
