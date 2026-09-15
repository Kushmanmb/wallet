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
- **R46** **M** `Onboarding/Sources/ViewModels/ImportWalletSceneViewModel.swift` (5) against Android `ImportViewModel`.
- **R47** **S** `Transfer/Sources/ViewModels/AmountSceneViewModel.swift` (4) — balance line, reserved-fee line, max and continue.
- **R48** **S** `Transfer/Sources/ViewModels/AmountPerpetualViewModel.swift` (4).
- **R50** **S** `QRScanner/Sources/ViewModels/QRScannerErrorViewModel.swift` (4) — check the scanner divergence note in SERVICES.md first; only the non-platform half moves.
- **R51** **M** `Perpetuals/Sources/ViewModels/PerpetualsSceneViewModel.swift` (4) against Android `PerpetualMarketViewModel`.

## 5. Screens that may want a session

Decide before building; a read-only screen is a row, not a session.

- **S17** **M** `Settings/Sources/ChainSettings/ViewModels/ChainSettingsSceneViewModel.swift` holds five mutable fields and loads node status concurrently after the node list. Android's `NetworksViewModel` carries the same state plus a refresh nonce it invented to order the two loads.
- **S18** **S** `Transactions/Sources/ViewModels/TransactionsFilterViewModel.swift` against the Android filter sheet.

## 7. Platform

### Hardcoded dp (Android rule: theme constants only)

Re-checked on 2026-09-15 by separating a `\d+.dp` written at a call site from one written as a named constant: **Android has none of the former**. All 58 remaining sites are `private val name = N.dp` declarations, which is the pattern the codebase already uses for a component's own dimensions, and only a handful of those carried a value the theme also names for the same kind of thing — those now read from the theme. What is left is a QR finder's 25 dp corner, a 42 dp search bar, a 296 dp slippage sheet and the like: dimensions with no theme equivalent, each named where it is used. X23–X29 are closed with no change.

### Swallowed errors

Closed on 2026-09-15. Four of the six were the same `try { focusRequester.requestFocus() } catch {}` written out in four screens — Compose throws when the target is not attached yet, so the catch is right and the duplication was the problem; they now call `requestFocusIfAttached()`. The other two were real: binding the camera use cases swallowed its failure, so a camera that could not start showed a blank preview with nothing in logcat, and the collectible details flow swallowed its load error and stayed null forever. Both report now.

### Deferred notes still in the code

- **X33** **S** `ios/Packages/Components/Sources/TextFields/CurrencyTextField.swift` pins the field height to work around <https://developer.apple.com/forums/thread/806828>. Remove the `.frame(height:)` and check the amount field on the oldest supported iOS and the newest; if the text no longer jumps, the pin goes.
- **X35** **S** `ios/Packages/Gemstone/Package.swift` pins Swift 5 language mode. Re-checked on 2026-09-15 against the current toolchain: dropping the pin fails on two `uniffiFutureContinuationCallback` sites in the generated `Gemstone.swift` — "passing closure as a 'sending' parameter risks causing data races". The fix is upstream in uniffi's Swift bindgen, not here; re-check after the next uniffi bump.
- **X36** **S** `LocalKeystore.swift` and `DB.swift` each run `FileMigrator` at launch to move the keystore directory and the database out of the documents directory into application support. Both were marked "delete in 2026" and the notes are gone, but the code stays until someone with the install numbers decides: deleting it while any user is still on a pre-move build points the keystore at a path that does not exist, and that user loses their wallet. The cost of keeping it is one `fileExists` per launch after the first (`FileMigrator.migrate` returns the new URL untouched when the old path is empty), so the default is to keep it. What settles it: the share of active installs whose last upgrade predates the move.
- **X39** **M** `core/crates/nft/src/providers/ton/verified.rs` hardcodes the verified-collection allowlist. Backend gap; track it where the backend work lives or close the note.
- **X40** **S** `core/apps/api/src/devices/mod.rs` — the legacy singular route is due after 2026-11-15.
- **X41** **S** `core/crates/primitives/src/device_locale.rs` `from_client` accepts codes the current apps never send. Checked on 2026-09-15: `in`, `iw` and `tl` are not legacy at all — the JVM still returns those for Indonesian, Hebrew and Filipino, so those arms are permanent. What is removable is the region-less `zh` and `pt` and the long "fall back to English" list, and only once no installed client sends them; `from_locale_identifier` already normalises what both apps send today. The note in the code that called all of it legacy is gone.
- **X42** **S** `core/crates/primitives/src/swap_provider.rs` still carries `CetusAggregator` beside `CetusClmm`. Checked on 2026-09-15: a completed swap stores its provider as a string in `TransactionSwapMetadata.provider`, so dropping the variant does not break a stored row — it breaks reading one back, and an old Cetus swap loses its provider name and its swap-again action. The query that settles it is whether any stored swap metadata still carries `cetus_aggregator`; the note in the code, which blamed client references, is gone.

### Core hardening

- **X45** **M** 255 production `unwrap`/`expect` sites remain in `core/`, counted on 2026-09-15 with test modules, `testkit.rs`, benches and `bin/` excluded. The three explorer registries and the thirteen Hyperliquid payload builders are done; the rest are led by `gem_stellar/provider/transactions.rs` (8), `primitives/transaction.rs` (7), `gem_bitcoin/provider/transactions.rs` (7), `swapper/thorchain/quote_data_mapper.rs` (6) and `gem_solana/provider/transactions.rs` (6). Take one file at a time and either make the value infallible by construction — the way the explorers now build their metadata directly instead of looking a key back up — or return the error.

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

## 11. Missing tests

### Hardcoded user-visible strings

Swept on 2026-09-15 over every non-preview, non-test iOS file: 25 hits, of which 20 are the developer screen (a debug screen that is deliberately untranslated) and 4 are inside a `PreviewProvider`. The one real hit was the wallet screen's "Trade Perpetuals" row, now `perpetuals_trade`. The same sweep over Android found none.

- **L14** **S** `perpetuals_trade`, `widget_empty` and `widget_empty_short` carry the English text in the other 30 locales until the next translation pass.

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
- **T29** **S** `GemPriceWidget` — `PriceWidgetViewModel`, `CoinPriceRowViewModel`. The widget target has no test bundle at all, so this one starts by adding it to `unit_frameworks.xctestplan` and the pbxproj. While reading them, `emptyMessage` turned out to hardcode its two strings; that is fixed and `widget_empty`/`widget_empty_short` need translating out of English in the other 30 locales.

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

- **X106** **S** `core/gemstone/src/gateway/chain_factory.rs` (3), `device.rs` (2), `signer/chain.rs` (1), `block_explorer/explorer.rs` (1).

## Closed with no change

Each of these was a section of the 2026-09-15 sweeps. The work was to check them; the answer was that the sweep measured the wrong thing. They are kept so the same sweep is not run again with the same conclusion.

### 3. Views that decide
Both closed on 2026-09-15. **B9**: all 21 iOS scenes and views that name `Gemstone` `switch` over a row key or read a row record — the contract working — and the two that branch on a Core value (`TransactionSwapProgressView` showing the estimated time on the spinner step, `SupportMessageBubble`) make the same call Android makes in the same place. **B10**: of 52 Android composables that `when` over a Core enum, the branch is an icon, a colour or a painter in almost every case — the style half of the mapper contract — and `android/ui` was keeping it in eight ad-hoc files. The tone, state and verification mappings now live in `ui/style/GemstoneStyle.kt` beside the header-button icon. What is left branches on a Core value to pick a keyboard, a paste handler or a trailing composable, which is rendering.

### 4. Ownership
O15–O21 are closed with no change. `LockSceneViewModel` builds `GemSecurityService` only inside its `static var preview`, which is SwiftUI preview code, not wiring. The 42 "types named by neither app" are the same mistake as § 10: a type that reaches an app as a nested field or an enum payload is never spelled out in Swift or Kotlin, so naming is the wrong test. Every one of the 42 was checked — `GemAmountStakeType` is a field of `GemAmountType`, `GemCollectibleAttribute` is a payload of `GemCollectibleSection`, `GemEIP712Message` is built by the message signer — and none is unreachable. A genuinely dead Core type is still worth finding; a name search does not find it.

### 8. Performance
The one item here landed: the node list built its rows one FFI crossing per node on every emission, and Core now takes the nodes and their statuses together. No other measured regression is open — [PERFORMANCE.md](PERFORMANCE.md) names the owner of each primary journey and says plainly that none has been profiled on a device.

### 9. Core decides it, only one app reads it

All 35 closed by 2026-09-15. Four landed a change — the confirm error chevron, the listed-asset-rank consolidation, the Android amount prefill and the sign-message forwarders — and the rest measured the wrong thing.

Each of these was an `#[uniffi::export]` the sweep found named in one app and in neither the other app's Kotlin nor its Swift. The first run skipped every iOS file whose name ends `+Gemstone.swift`; the corrected run is the one these notes describe.

The sweep measures which *export* each app names, which is not the same as which app *owns* the decision. Eleven were checked on 2026-09-15 and closed with no change, because the app that never calls the export still reads the same Core answer through a different one: Android reads the abbreviation cutoff through `GemValueStyle.abbreviates`, the price-alert kind through the aggregate's `kind.groupsByAsset()`, the dApp name through `GemConfirmDestination.Generic` and `connection_row`, whether to show a memo through the confirm row set, the latest block through the node row's `GemNodeSubtitle.LatestBlock`, the swap minimum through `GemSwapButtonAction.UseMinimumAmount`, and whether to offer rewards through the `GemSettingsRow.REWARDS` the settings service emits from it.

The second pass closed ten more. Three were never exported: `named`, `synchronize` and the avatar service's `set_image` / `remove_image` sit in plain `impl` blocks that the sweep's name match picked up, and the avatar methods reach both apps through `GemWalletService`. Four are the same Core answer asked for differently: Android passes `submit_attempted` straight into the autoclose session constructor instead of calling `on_submit_attempt`, clears the add-asset form with `new_session` instead of `on_chain`, reads the recommended validators out of the stake selection record, takes `swap_quote` through `swapper_quote_summary`, and reads the swap error display off the session rather than through the free function iOS uses to give `SwapperError` a description. The rest are platform plumbing, not decisions: Android shows no error at all for a failed biometric prompt so it has nothing to gate on `is_cancelled`, its image loader caches a support attachment by URL so it never needs `image_file`, and the developer screen simply offers fewer actions than the iOS one. The second pass also closed twelve on the iOS side. `update_balance` and the five perpetual sync methods are plain `impl` blocks, not exports — only the iOS mock reimplements them by name. `listed_asset_rank` is gone: both apps now read the rank from the asset config service. `decode_url` has one caller in the whole repo, an Android instrumentation test; both apps decode a payment link through `load`, which goes to the same `PaymentURLDecoder`. Android needs `chain_from_caip2` because it routes on the `Chain` enum app-side while iOS hands the CAIP-2 string straight to Core, and it builds a session from `on_auto` where iOS builds the same session from the `Auto` selection. The rest are one app not having the flow at all: iOS inserts no default asset record when a wallet is created, has no invalid-word highlighting in the import field, no NFT receive chain picker, no token-search sync behind its price widget, no timed retry after a failed biometric prompt, and its update check compares versions inside `newest` rather than against a Play archive.

The WalletConnect items resolved on the same pass. `message_preview`, `message_address_names` and `address_url` were forwarders on the connect service over the sign-message service iOS already calls directly; they are gone and Android holds the sign-message service. `is_origin_rejected` and `user_rejected_error` are both applied inside `process_request` and `session_proposal`, which is how iOS gets them; Android's extra calls belong to the one-click authentication request, a flow iOS does not implement at all — the same reason `authentication_accounts`, `authentication_chain_ids` and `authentication_methods` look one-sided. The swap session transitions are not two sets either: `on_request_changed` composes `on_refresh_requested`, which composes `on_quote_invalidated`. Android calls the innermost one eagerly when the user picks an asset, switches the pair or changes slippage, so the transfer phase clears before the request recomputes; iOS reaches the same state through the outer transition.




### iOS does not read an Android-read decision

### 11. Core behaviour with no test

All eight closed on 2026-09-15. The orchestration in `wallet_home`, `asset_discovery`, `assets/details`, `app_start`, `rewards` and `perpetual` now has tests. `asset_discovery/testkit.rs` assembles the balance, discovery, transactions and NFT graph behind one constructor; `app_start` and `rewards` build on the wallet testkit so they sign with a real keystore; and `TestAlienProvider::with_json_by_path` answers each endpoint with its own body.

### 10. Exports no app calls at all
Closed on 2026-09-15 with no change. The sweep counted **app** callers, which is the wrong test for a `rules.rs` function: rules are called by the service that owns them, and the app calls the service. Every function listed here has Core callers — `sanitize_number_input` has fifteen, `node_url` twenty-three, `price_alert_toggle` is read by the asset row, `shows_header` by the confirm screen — and the explorer getters are used by nine other services. A Core export with no caller anywhere is still worth finding; counting app callers alone does not find it.

### 12. Localization hygiene

Twenty English strings exist under two keys — `wallet_send` / `transfer_send_title`, `wallet_stake` / `transfer_stake_title`, `wallet_import_address_field` / `transfer_recipient_address_field` and seventeen more. Checked on 2026-09-15: each pair carries its own context comment in `localization/app/en.ftl` and names a different place in the product, so the pairs are deliberate and must not be merged — a language that needs a different form for a label and a screen title depends on them being separate.
What went wrong in V40 and V41 was not the pair; it was one app's mapper reaching for the other half of a pair. That is only visible by comparing the two apps, which `just check-mappers` now does on every variant both apps map. The fifteen keys nothing read are gone.
