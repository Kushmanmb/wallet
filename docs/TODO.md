# Open work

Every open item carries a stable id (V vocabulary, R rows, C composition, S sessions, B view boundary, F formatting, P parity, D decisions, O ownership, X platform, G guidance, PERF performance) and a size (**S**/**M**/**L**). Contracts are in [ARCHITECTURE.md](ARCHITECTURE.md) and [SERVICES.md](SERVICES.md). **Delete an item's line in the commit that lands it** — ids are never reused.

The goal is that Gemstone decides once and both clients read that decision. Track duplicated decisions and concrete performance work at their existing owners: shared rules and orchestration in Core; rendering, observation, scheduling, and localized formatting in the apps.

Keep each item independently reviewable. Shared decision changes land in Core and both apps; platform-only work stays on that platform. Regenerate only when shared interfaces or integration change, and run the applicable [Quality Checks](../skills/quality-checks.md). Verify affected primary-screen journeys under [Performance](PERFORMANCE.md). Remove replaced paths within the item's scope; do not bundle an unrelated row migration or product change.

This list was rebuilt on 2026-09-15 from scripted sweeps over the whole repo. Each item names the file the sweep hit, so it can be confirmed before it is started.

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

84 sites across 40 files. Grouped by where they are.

- **X21** **S** `ui/components/chart` — `GemLineChart.kt` (11), `GemCandlestickChart.kt` (7), `CandlestickTooltip.kt` (3).
- **X22** **S** `ui/components/list_item` — `ListItemPositionClip.kt` (5), `SubheaderItem.kt` (3).
- **X23** **M** the rest of `android/ui` — `QRScanner.kt` (4), `SearchBar.kt` (3), `buttons/CopyButton.kt` (2), `filters/FormDialog.kt` (2) and the remainder of the 56 in that module.
- **X24** **S** `features/settings/settings/presents` — `SupportMessageBubble.kt` (7) and one more.
- **X25** **S** `features/receive/presents` — `ReceiveScreen.kt` (4) and one more.
- **X26** **S** `features/perpetual/presents` (3).
- **X27** **S** `features/buy/presents` — `FiatScene.kt` (2).
- **X28** **S** `features/import_wallet/presents` (2).
- **X29** **S** One site each in `app`, `features/activities/presents`, `features/add_asset/presents`, `features/create_wallet/presents`, `features/nft/presents`, `features/referral/presents`.

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
