# Open work

Every open item carries a stable id (V vocabulary, R rows, C composition, S sessions, B view boundary, F formatting, P parity, D decisions, O ownership, X platform, G guidance, PERF performance) and a size (**S**/**M**/**L**). Contracts are in [ARCHITECTURE.md](ARCHITECTURE.md). **Delete an item's line in the commit that lands it** — ids are never reused.

The goal is that Gemstone decides once and both clients read that decision. Track duplicated decisions and concrete performance work at their existing owners: shared rules and orchestration in Core; rendering, observation, scheduling, and localized formatting in the apps.

Keep each item independently reviewable. Shared decision changes land in Core and both apps; platform-only work stays on that platform. Regenerate only when shared interfaces or integration change, and run the applicable [Quality Checks](../skills/quality-checks.md). Verify affected primary-screen journeys under [Performance](PERFORMANCE.md). Remove replaced paths within the item's scope; do not bundle an unrelated row migration or product change.

## 0. Performance on primary screens

These are code-backed candidates, not measured speedups. Capture a before/after trace for each implemented item; use controlled completion order for concurrency tests rather than timing assertions. Preserve domain outputs, ordering, errors, wallet context, and security gates. Prefer changes that also remove duplicate code, clarify ownership, or simplify state; added caches and coordination must earn their complexity. Start with PERF2–PERF6. PERF7–PERF9 require investigation before changing behavior.

### Small codebase improvements


### Implementation candidates


### Investigate before changing behavior

- **PERF7** **M** Wallet balance publication — [`GemBalanceService.update`](../core/gemstone/src/services/balance/mod.rs) waits for all chains before writing. Trace a fast chain beside a delayed chain and document what observers assume about portfolio totals, write batches, wallet switching, and overlapping refreshes. The deliverable is a tested publication contract for complete per-chain results through the existing `GemBalanceStore`. Do not implement incremental writes until mixed-age totals, superseded responses, error reporting, and notification frequency are accounted for. Preserve unchanged-write suppression and the existing treatment of failed stake/earn components; changing partial-failure recovery is a separate behavioral task.
- **PERF8** **M** Confirmation dependency graph — [`GemConfirmService.load`](../core/gemstone/src/services/confirm/mod.rs) waits for scan and simulation before transaction loading. Trace each stage and audit `get_transaction_load` across providers for reads, mutations, and external effects before moving it earlier. Produce a dependency graph and controlled-order test demonstrating any safe overlap. Keep scan rejection and simulation failure ahead of ready/signable state, and preserve authentication, approval, signing, and broadcast order. Record the existing scanner-outage policy explicitly; do not change it or start new provider work for rejected inputs as an incidental optimization. Only then define the implementation commit.
- **PERF9** **S** Swap provider timing — [`Swapper`](../core/crates/swapper/src/swapper.rs) awaits all route preloads and all quotes. Measure per-provider discovery and quote durations, existing transport timeouts, cancellation, and cold/warm route-cache behavior. Identify the provider or phase responsible before proposing a change. Preserve [Swapper](SWAPPER.md)'s preload-before-quote contract, live quotes, route hints, ranking, and current-input checks. A new deadline changes which providers can compete and requires a separate documented policy; do not add a timeout, first-result selection, quote cache, or speculative preloading in this investigation.

## 1. Lists get a row record

Copy: [`GemAssetRow`](../core/gemstone/src/services/assets/model.rs) → [iOS](../ios/Packages/PrimitivesComponents/Sources/ViewModels/ListAssetItemViewModel.swift), [Android](../android/gemcore/src/main/kotlin/com/gemwallet/android/domains/asset/aggregates/AssetInfoDataAggregate.kt).



- **R17** **L** Settings has no Core service at all. Both apps show the same nine rows in the same order — wallets, security, notifications, preferences, WalletConnect, support, rewards, about, developer — so there is no drift today; the row set, its order, the icons and the visibility conditions are simply written twice — [iOS `SettingsViewModel`](../ios/Features/Settings/Sources/Settings/ViewModels/SettingsViewModel.swift) as `xTitle`/`xImage` pairs, [Android `SettingsScene`](../android/features/settings/settings/presents/src/main/kotlin/com/gemwallet/android/features/settings/settings/presents/views/SettingsScene.kt) inline in the composable. Add a `settings` service with a row record carrying the key, the icon key and the destination, and let each app map the key to its own image and string. `PreferencesViewModel`, `SecurityViewModel` and `AboutUsViewModel` are the same screen family and go with it.
- **R19** **M** WalletConnect connections — [`ConnectionsViewModel`](../ios/Features/WalletConnector/Sources/WalletConnector/ViewModels/ConnectionsViewModel.swift) and `ConnectionSceneViewModel` hold no Core record and build the row and the detail fields themselves; Android's bridge screens do the same. `GemConnectionRow` already exists and answers part of it.
- **R20** **M** Chain settings and nodes — `ChainSettingsSceneViewModel`, `ChainNodeViewModel` and `ServiceStatusItemViewModel` on iOS against the Android networks screens. `GemNodeSelection` and `GemServiceEndpoint` cross, but the section titles, the node subtitle and the explorer row are each app's.
- **R21** **M** Stake and delegation — `StakeSceneViewModel`, `DelegationSceneViewModel` and `DelegationViewModel` build nine section and field titles app-side against the Android earn screens.

- **R24** **L** The shared row models in [`PrimitivesComponents`](../ios/Packages/PrimitivesComponents/Sources/ViewModels) are the largest unmigrated group: 29 models, 57 user-facing strings, none holding a Core record — `AssetViewModel`, `AddressListItemViewModel`, `AssetDataViewModel`, `NetworkSelectorViewModel`, `WalletHeaderViewModel`, `MarketValueViewModel`. Android mirrors each in `ui-models` and `gemcore/domains`. Take them one row at a time; `GemAssetRow` is the exemplar and several already have a Core record they do not hold. `GemAssetRow` names the row's shape for a whole screen, so the per-item values still come from `AssetDataViewModel` on iOS and `AssetInfoDataAggregate` on Android; the row model drops that second object once Core carries the values.
- **R25** **M** Perpetuals — `PerpetualSceneViewModel` alone decides nine section and button titles, with `AutocloseViewModel`, `PerpetualsHeaderViewModel` and `ChartLineViewModel` behind it, against the Android perpetual screens.
- **R26** **M** Onboarding — ten models across import, setup, phrase verification and secret display decide their own titles and footer text against the Android `import_wallet` and `create_wallet` screens.
- **R27** **M** Transfer — `AmountSceneViewModel`, `ReceiveViewModel` and `AmountPerpetualViewModel` against the Android `transfer_amount` and `receive` screens.

Rejected: a screen's chrome — its sheet title and its cancel, clear and done buttons — is each app's own and stays there; a camera's permission and support states are the platform's, and the two apps' states do not even match, so the QR scanner's error text stays local; transaction, transaction detail, delegation, validator, asset select/search, wallet, price alert, fiat quote, currency, fee rate, simulation warning, asset market, collectible detail and banner rows already have a record; network list, recents chips, earn APR, swap detail, price list and onboarding rows carry no choice; swap provider rows and the QR scan-type hint table are iOS only; swap price impact already crosses as `impactType`/`isHigh`/`showsInSummary`; the delegation completion countdown is computed twice but belongs to the delegation record if anywhere.

- **R33** **M** Row models with no Core row at all: iOS `PerpetualItemViewModel`, `PerpetualPositionItemViewModel`, `OpenPositionItemViewModel` and `AssetListItemViewModel` decide their own name, symbol, image, subtitle and right view. Give each the row its screen needs, in the shape of `GemPriceAlertRow`.
- **R34** **L** `ListAssetItemViewable` itself is a parity question: its `subtitleView` and `rightView` are a fixed set of shapes (price, balance, toggle, copy, none) that both apps re-derive per screen. Decide whether Core names the shape — a `GemListItemSubtitle`/`GemListItemAccessory` enum every row carries — so a row's layout is chosen once. Weigh it against [§ 3 Keep the crossings few](ARCHITECTURE.md#keep-the-crossings-few) before starting; this is the largest of the row items and should follow R33 and R24.


- **O12** **M** `WalletId` and `Chain` still cross as bare strings, so every call unwraps one by hand on both platforms. `WalletId` is a struct in the apps and a `String` across the FFI — that is why [`GemWalletService.swift`](../ios/Packages/GemstonePrimitives/Sources/Services/GemWalletService.swift) is sixty lines of `wallet.id.id` and `.toPrimitives()`, a bridge with no decisions in it. `Chain` is declared under `codes` in [`remote_types.yml`](../core/bin/generate/remote_types.yml) and crosses the same way, so `chain.rawValue` and `Chain(core:)` litter both apps and every new Core parameter tempts the next caller to type it `String`. Make them cross as the records the apps already hold, the way `AssetId` and `Currency` do, and the bridges go with them. Check first what persists a wallet id as a raw string.

## 2. Sections, actions, destinations and limits

Per-variant labels: a primitives enum both apps map to a string themselves is a decision written twice. Both languages force a `switch`/`when` over a Core enum to be exhaustive, so these sets cannot silently drift — every one checked below maps the same variants to the same meaning. That makes the V series maintenance cost and a place for drift to start, not a live bug; the exception is a catch-all branch, which **X20** covers. The migrated shape is a Core text key each app resolves once in its own `Gemstone+Localized.swift` / `GemstoneText.kt` — `GemTransactionTitle`, `GemBannerTitle` and `GemWalletSubtitle` already work that way. These do not:

- **V32** **L** Info sheets, and this is a product gap before it is a duplication: iOS `InfoSheetType` has 30 cases, Android's [`InfoSheetEntity`](../android/ui/src/main/kotlin/com/gemwallet/android/ui/components/InfoBottomSheet.kt) has 14. Every Android sheet exists on iOS, so nothing is Android-only; **sixteen explanations are iOS-only** — `assetStatus`, `autoclose`, `circulatingSupply`, `fullyDilutedValuation`, `fundingApr`, `fundingPayments`, `liquidationPrice`, `maliciousTransaction`, `maxSupply`, `memoRequired`, `minimumAmount`, `noQuote`, `openInterest`, `priceImpact`, `slippage`, `totalSupply` and `watchWallet`. `maliciousTransaction` is a security explanation an Android user never sees. Decide the set in Core, then each app renders it.
- **V34** **M** The confirm and swap detail row sets — `ConfirmTransferScene.itemModel` maps 15 cases on iOS against the Android confirm screen, and the swap detail rows (provider, price impact, the impact warning) name their own titles on both apps. One row record per screen answers both; the fixed labels move with it.
- **V35** **M** Select-asset presentation is a structural difference, not a label map: iOS reuses one modal and maps [`SelectAssetType`](../ios/Features/Assets/Sources/Types/SelectAssetPresentation.swift) to eleven titles in one place, while Android gives each caller its own screen — manage, price alert, swap pay/receive, fee asset — each naming its own title. Give Android the one presentation before the titles can cross; until then there is nothing to centralise.

- **V31** **S** The swap select side (pay and receive) is mapped inside Android's `SwapSelectScreen`; iOS already resolves it in its select-asset presentation, so this lands with **V35**.


Copy: [`GemPerpetualMarketCounts::sections`](../core/gemstone/src/services/perpetual/model.rs) → [iOS](../ios/Features/Perpetuals/Sources/ViewModels/PerpetualsSceneViewModel.swift), [Android](../android/features/perpetual/presents/src/main/kotlin/com/gemwallet/android/features/perpetual/views/market/PerpetualMarketScene.kt).


## 3. Screens get a session

Copy: [`fiat/session.rs`](../core/gemstone/src/services/fiat/session.rs) → [iOS](../ios/Features/FiatConnect/Sources/ViewModels/FiatSceneViewModel.swift), [Android](../android/features/buy/viewmodels/src/main/kotlin/com/gemwallet/android/features/buy/viewmodels/FiatViewModel.kt). A smaller one to copy first: [`chart/session.rs`](../core/gemstone/src/services/chart/session.rs) → [iOS](../ios/Features/MarketInsight/Sources/ViewModels/ChartSceneViewModel.swift), [Android](../android/features/asset/viewmodels/src/main/kotlin/com/gemwallet/android/features/asset/viewmodels/chart/viewmodels/ChartViewModel.kt).

A session is a plain Record: the screen's service vends it, `on_*` events return a new one, and `view_state()` derives what the screen shows. It performs no I/O — the app awaits its own service and hands the result back through an event. Do not let the flow that collects a session also write to it; drive loads from the period or trigger that changed.


Rejected: confirm, swap and fiat already hold a session; wallet home already has a `view_state`; asset details, delegation, stake, earn, receive, NFT details, collections, contacts list, transaction details, wallets list and currency are read-only or single-selection; perpetual market has no shared derived set; support chat's only duplication is day grouping (C6) and platform image encoding; security and developer are platform preferences; create-wallet and phrase verification are not symmetric; the perpetuals preview gates on balance on iOS and on user config on Android, which is a § 6 question, not a session.

## 4. Views stop naming Core types

Copy: [`FiatScene.swift`](../ios/Features/FiatConnect/Sources/Scenes/FiatScene.swift) names no Core type. Most of this is absorbed by § 1 to § 3.


## 5. Numbers cross as a value and a style

Contract: [a number crosses as a value and a style](ARCHITECTURE.md#a-number-crosses-as-a-value-and-a-style-never-as-a-string-or-a-callback). The precision rules, the value ladder, the abbreviation threshold and the dust cut are Core's, and `GemFormattedNumber` carries a number with its resolved display. What is left is the records that still hand the apps a bare `f64` and let each pick a style. Copy [`GemFiatQuoteRow`](../core/gemstone/src/services/fiat/model.rs) and [`GemAmountError::display`](../core/gemstone/src/services/amount/model.rs).



Not in scope: counts a screen uses to build sections (`GemWalletSearchCounts`, `GemPerpetualMarketCounts`, `GemNetworkAssetCounts`, `GemRecentsCounts`), decimals, bps, indices, timeouts and chart geometry stay numbers — the contract is about numbers the app renders as text.

Not in scope: `Formatters` and `Validators` on iOS still cannot import Gemstone, so the renderer that applies a `GemPrecision` must stay dependency-free. That is why this is a value-plus-style contract and not a foreign trait.

## 6. Platform parity

Core exports these and one app calls them. Establish whether it is a missing feature or a duplicated decision first. Exports that belong to a screen already listed above are noted on that item (V2, S3, S5, S9, S16, F8).


Legitimately one-sided, not gaps: `isVersionHigher` (Play update), `migrateToSharedPassword` (Android password store), `set_price_alerts_enabled` (Android one-off migration), `signWithKeystore` (iOS keystore), `isOriginRejected`, `authentication_chain_ids`, `authentication_accounts`, `authentication_methods` in the auth flow (Android-only one-click auth, D4; proposal and sign check the origin inside Core on both), `scanTransaction` (both scan through `GemConfirmService`).

## 7. Everything else

Product or security decisions, one question each:


Ownership, injection and threads:

- **O9** **M** Four Android view models hold two or three Core services where iOS composes one screen service: [`TransactionsViewModel`](../android/features/activities/viewmodels/src/main/kotlin/com/gemwallet/android/features/activities/viewmodels/TransactionsViewModel.kt), `WCAuthViewModel` (three), `WCRequestViewModel` and `SettingsViewModel`. Copy the screen-service shape in [SERVICES.md](SERVICES.md#the-screen-service-map); a launch host or a flow parent vending child models is [allowed to hold several](ARCHITECTURE.md#7-at-most-one-core-service-on-ios-narrow-cases-on-android) and these are neither.



Platform items:








Guides:


Do not "fix" the [deliberate divergences](SERVICES.md#deliberate-divergences--do-not-fix-these).
