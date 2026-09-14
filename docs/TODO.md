# Open work

Every open item carries a stable id (V vocabulary, R rows, C composition, S sessions, B view boundary, F formatting, P parity, D decisions, O ownership, X platform, G guidance) and a size (**S**/**M**/**L**). Contracts are in [ARCHITECTURE.md](ARCHITECTURE.md). **Delete an item's line in the commit that lands it** — ids are never reused.

One item, one commit, both apps built and tested. Core rule + test, regenerate bindings, delete the app code it replaced on both sides, clear that screen's view leak while you are in it.

## 1. Lists get a row record

Copy: [`GemAssetRow`](../core/gemstone/src/services/assets/model.rs) → [iOS](../ios/Packages/PrimitivesComponents/Sources/ViewModels/ListAssetItemViewModel.swift), [Android](../android/gemcore/src/main/kotlin/com/gemwallet/android/domains/asset/aggregates/AssetInfoDataAggregate.kt).

- **R11** **S** NFT list variants drop the verified badge the grid shows, and the asset count sits in a different place on each — [iOS list](../ios/Features/NFT/Sources/Views/CollectionsPreviewView.swift), [`NftListItem`](../android/ui/src/main/kotlin/com/gemwallet/android/ui/components/list_item/NftListItem.kt). `GemNftRow` already answers both; what is left is the design call on where they belong. Needs product input.
- **R13** **S** Fiat transaction — [iOS](../ios/Packages/PrimitivesComponents/Sources/ViewModels/FiatTransactionViewModel.swift), [Android](../android/features/buy/presents/src/main/kotlin/com/gemwallet/android/features/buy/views/FiatTransactionItem.kt). The provider name and the badge case come from Core; the badge colour is each app's palette, which is [where a colour belongs](ARCHITECTURE.md#a-list-row-is-a-record-of-choices). What is left is amount as a subtitle against a trailing, which is a design call, not a record.
- **R15** **S** Perpetual position text templates — margin renders "amount (type)" and each trigger order "label: price" in [`PerpetualPositionViewModel`](../ios/Features/Perpetuals/Sources/ViewModels/PerpetualPositionViewModel.swift) plus [`AutocloseFormatter`](../ios/Packages/Formatters/Sources/AutocloseFormatter.swift), and again in [`PositionProperties`](../android/features/perpetual/presents/src/main/kotlin/com/gemwallet/android/features/perpetual/views/components/PositionProperties.kt). The values and the hide rules are Core's; only the two templates are still written twice. Whether a template belongs in Core is a [F](#5-numbers-cross-as-a-value-and-a-style) question, not a row one.

Rejected: transaction, transaction detail, delegation, validator, asset select/search, wallet, price alert, fiat quote, currency, fee rate, simulation warning, asset market, collectible detail and banner rows already have a record; network list, recents chips, earn APR, swap detail, price list and onboarding rows carry no choice; swap provider rows and the QR scan-type hint table are iOS only; swap price impact already crosses as `impactType`/`isHigh`/`showsInSummary`; the delegation completion countdown is computed twice but belongs to the delegation record if anywhere.

## 2. Sections, actions, destinations and limits

Copy: [`GemPerpetualMarketCounts::sections`](../core/gemstone/src/services/perpetual/model.rs) → [iOS](../ios/Features/Perpetuals/Sources/ViewModels/PerpetualsSceneViewModel.swift), [Android](../android/features/perpetual/presents/src/main/kotlin/com/gemwallet/android/features/perpetual/views/market/PerpetualMarketScene.kt).


## 3. Screens get a session

Copy: [`fiat/session.rs`](../core/gemstone/src/services/fiat/session.rs) → [iOS](../ios/Features/FiatConnect/Sources/ViewModels/FiatSceneViewModel.swift), [Android](../android/features/buy/viewmodels/src/main/kotlin/com/gemwallet/android/features/buy/viewmodels/FiatViewModel.kt). A smaller one to copy first: [`chart/session.rs`](../core/gemstone/src/services/chart/session.rs) → [iOS](../ios/Features/MarketInsight/Sources/ViewModels/ChartSceneViewModel.swift), [Android](../android/features/asset/viewmodels/src/main/kotlin/com/gemwallet/android/features/asset/viewmodels/chart/viewmodels/ChartViewModel.kt).

A session is a plain Record: the screen's service vends it, `on_*` events return a new one, and `view_state()` derives what the screen shows. It performs no I/O — the app awaits its own service and hands the result back through an event. Do not let the flow that collects a session also write to it; drive loads from the period or trigger that changed.

- **S12** **S** Select asset sequencing — the sections, the flow record, what the flow shows, the search step and the list state now cross. What is left is the query itself: iOS drives an `ObservableQuery` off `searchableQuery` while Android debounces a `TextFieldState` into `SelectAssetFilters`, and neither shape is a Core decision yet.
- **S14** **S** Amount sequencing — the confirm gate is `GemAmountEntry::allows_confirm` and `GemAmountError::display` now decides whether an error reads at all and how the asset is named. What is left is the input sequencing itself: iOS refreshes an entry off a text binding, Android combines a snapshot flow with its providers, and neither shape has been reduced to a Core step.

Rejected: confirm, swap and fiat already hold a session; wallet home already has a `view_state`; asset details, delegation, stake, earn, receive, NFT details, collections, contacts list, transaction details, wallets list and currency are read-only or single-selection; perpetual market has no shared derived set; support chat's only duplication is day grouping (C6) and platform image encoding; security and developer are platform preferences; create-wallet and phrase verification are not symmetric; the perpetuals preview gates on balance on iOS and on user config on Android, which is a § 6 question, not a session.

## 4. Views stop naming Core types

Copy: [`FiatScene.swift`](../ios/Features/FiatConnect/Sources/Scenes/FiatScene.swift) names no Core type. Most of this is absorbed by § 1 to § 3.


## 5. Numbers cross as a value and a style

The largest duplication left. Contract: [a number crosses as a value and a style](ARCHITECTURE.md#a-number-crosses-as-a-value-and-a-style-never-as-a-string-or-a-callback). Do F1 first; the rest depend on it.


  Shape it this way: `Formatters` keeps a dependency-free `NumberPrecision` (`.fraction(min:max:)` / `.significant(max:)`) and the mapping to `NumberFormatStyleConfiguration.Precision` — that is the renderer the note below says must stay. What leaves is `adaptive(for:)`: `CurrencyFormatter.string` and `NumericFormatter.string` take the precision instead of choosing it, and `GemstonePrimitives` maps `GemPrecision` to `NumberPrecision`. The cost is the blast radius, not the design: 39 `CurrencyFormatter(` and 23 `NumericFormatter(` construction sites, every number the app renders. Land it behind a screen-by-screen check, not in a sweep. The two rules do agree today — same 1e-10 and 0.99 thresholds, same two-place and four-significant shapes — so this is drift prevention, not a live bug.

Not in scope: `Formatters` and `Validators` on iOS still cannot import Gemstone, so the renderer that applies a `GemPrecision` must stay dependency-free. That is why this is a value-plus-style contract and not a foreign trait.

## 6. Platform parity

Core exports these and one app calls them. Establish whether it is a missing feature or a duplicated decision first. Exports that belong to a screen already listed above are noted on that item (V2, S3, S5, S9, S16, F8).


Legitimately one-sided, not gaps: `isVersionHigher` (Play update), `migrateToSharedPassword` (Android password store), `set_price_alerts_enabled` (Android one-off migration), `signWithKeystore` (iOS keystore), `isOriginRejected`, `authentication_chain_ids`, `authentication_accounts`, `authentication_methods` in the auth flow (Android-only one-click auth, D4; proposal and sign check the origin inside Core on both), `scanTransaction` (both scan through `GemConfirmService`).

## 7. Everything else

Product or security decisions, one question each:

- **D8** Keystore v4 follow-ups from [KEYSTORE_V4.md](KEYSTORE_V4.md): whether migration failures need durable telemetry or user-visible recovery, and the synchronous [`Keystore` trait](../core/crates/gem_keystore/src/storage/secret.rs) that a browser backend would need an in-memory mirror for.
- **D9** Swap max-amount fee trim — [SWAPPER.md](SWAPPER.md) records that confirmed and signed amounts can differ by up to the fee on providers outside the [reserve rule](../core/crates/swapper/src/fees/reserve.rs). Decide whether the signer or the quote owns the trim.

Ownership, injection and threads:

- **O2** **M** iOS [`Config.swift`](../ios/Packages/GemstonePrimitives/Sources/Config.swift) keeps `.shared` singletons of `GemAddressService`, `GemApplicationMetadataService`, `GemAssetConfigService`, `GemChainService` and `GemConnectionService` that [`ServicesFactory`](../ios/Gem/Services/ServicesFactory.swift) constructs again; about 25 feature call sites read the globals, and `ImportWalletViewModel` hands `GemChainService.shared` to its child.


Platform items:

- **X16** **S** [`debugLog`](../ios/Packages/Primitives/Sources/DebugLog.swift) compiles out in release, so the stream's connect, event and error lines exist in debug builds only and a released app leaves no trace of a dropped event. Decide a release-safe channel for the always-on paths or accept that field reports carry no log.


- **X8** **S** The same row reads differently on each app rather than by a Core decision: a curated asset list puts its count in the subtitle on [iOS](../ios/Features/WalletTab/Sources/ViewModels/AssetListItemViewModel.swift) and in a trailing badge on [Android](../android/features/assets/presents/src/main/kotlin/com/gemwallet/android/features/assets/views/WalletSearchScreen.kt), and a service status endpoint composes `"<name> <flag>"` separately in [iOS](../ios/Features/Settings/Sources/ChainSettings/ViewModels/ServiceStatusItemViewModel.swift) and [Android](../android/features/settings/networks/presents/src/main/kotlin/com/gemwallet/android/features/settings/networks/presents/ServiceStatusItem.kt). Both already take the title, icon and flag from Core, so what is left is a design-parity call, not a record.


- **X2** **L** iOS untyped `.map()` naming — Android names the direction. Generator pair in [`remote_mappers.rs`](../core/bin/generate/src/remote_mappers.rs), ~890 call sites.

Guides:


Do not "fix" the [deliberate divergences](SERVICES.md#deliberate-divergences--do-not-fix-these).
