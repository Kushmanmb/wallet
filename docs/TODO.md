# Open work

Every open item carries a stable id (V vocabulary, R rows, C composition, S sessions, B view boundary, F formatting, P parity, D decisions, O ownership, X platform, G guidance) and a size (**S**/**M**/**L**). Contracts are in [ARCHITECTURE.md](ARCHITECTURE.md). **Delete an item's line in the commit that lands it** — ids are never reused.

One item, one commit, both apps built and tested. Core rule + test, regenerate bindings, delete the app code it replaced on both sides, clear that screen's view leak while you are in it.

## 1. Lists get a row record

Copy: [`GemAssetRow`](../core/gemstone/src/services/assets/model.rs) → [iOS](../ios/Packages/PrimitivesComponents/Sources/ViewModels/ListAssetItemViewModel.swift), [Android](../android/gemcore/src/main/kotlin/com/gemwallet/android/domains/asset/aggregates/AssetInfoDataAggregate.kt).


- **R16** **S** `GemSwapProgressStep` is switched on inside the view on both apps to pick a marker glyph and a tone — [iOS](../ios/Features/Transactions/Sources/Views/TransactionSwapProgressView.swift) plus [its extension](../ios/Features/Transactions/Sources/ItemModels/TransactionSwapProgressItemModel.swift), [Android](../android/features/activities/presents/src/main/kotlin/com/gemwallet/android/features/activities/presents/details/components/SwapProgressItem.kt) — and the two have drifted: `refunded` reads as an arrow-swap in orange on iOS and as a red close icon on Android, and `waiting` is an ellipsis glyph against three hand-drawn dots. Copy [`GemTransactionStateTone`](../core/gemstone/src/services/transactions/model.rs): the step answers a tone and a marker kind, each app maps those to its own palette and icon set. Live divergence, not drift prevention.

Rejected: transaction, transaction detail, delegation, validator, asset select/search, wallet, price alert, fiat quote, currency, fee rate, simulation warning, asset market, collectible detail and banner rows already have a record; network list, recents chips, earn APR, swap detail, price list and onboarding rows carry no choice; swap provider rows and the QR scan-type hint table are iOS only; swap price impact already crosses as `impactType`/`isHigh`/`showsInSummary`; the delegation completion countdown is computed twice but belongs to the delegation record if anywhere.

## 2. Sections, actions, destinations and limits

Copy: [`GemPerpetualMarketCounts::sections`](../core/gemstone/src/services/perpetual/model.rs) → [iOS](../ios/Features/Perpetuals/Sources/ViewModels/PerpetualsSceneViewModel.swift), [Android](../android/features/perpetual/presents/src/main/kotlin/com/gemwallet/android/features/perpetual/views/market/PerpetualMarketScene.kt).


## 3. Screens get a session

Copy: [`fiat/session.rs`](../core/gemstone/src/services/fiat/session.rs) → [iOS](../ios/Features/FiatConnect/Sources/ViewModels/FiatSceneViewModel.swift), [Android](../android/features/buy/viewmodels/src/main/kotlin/com/gemwallet/android/features/buy/viewmodels/FiatViewModel.kt). A smaller one to copy first: [`chart/session.rs`](../core/gemstone/src/services/chart/session.rs) → [iOS](../ios/Features/MarketInsight/Sources/ViewModels/ChartSceneViewModel.swift), [Android](../android/features/asset/viewmodels/src/main/kotlin/com/gemwallet/android/features/asset/viewmodels/chart/viewmodels/ChartViewModel.kt).

A session is a plain Record: the screen's service vends it, `on_*` events return a new one, and `view_state()` derives what the screen shows. It performs no I/O — the app awaits its own service and hands the result back through an event. Do not let the flow that collects a session also write to it; drive loads from the period or trigger that changed.


Rejected: confirm, swap and fiat already hold a session; wallet home already has a `view_state`; asset details, delegation, stake, earn, receive, NFT details, collections, contacts list, transaction details, wallets list and currency are read-only or single-selection; perpetual market has no shared derived set; support chat's only duplication is day grouping (C6) and platform image encoding; security and developer are platform preferences; create-wallet and phrase verification are not symmetric; the perpetuals preview gates on balance on iOS and on user config on Android, which is a § 6 question, not a session.

## 4. Views stop naming Core types

Copy: [`FiatScene.swift`](../ios/Features/FiatConnect/Sources/Scenes/FiatScene.swift) names no Core type. Most of this is absorbed by § 1 to § 3.


## 5. Numbers cross as a value and a style

Contract: [a number crosses as a value and a style](ARCHITECTURE.md#a-number-crosses-as-a-value-and-a-style-never-as-a-string-or-a-callback). The precision rules, the value ladder, the abbreviation threshold and the dust cut are Core's, and `GemFormattedNumber` carries a number with its resolved display. What is left is the records that still hand the apps a bare `f64` and let each pick a style. Copy [`GemFiatQuoteRow`](../core/gemstone/src/services/fiat/model.rs) and [`GemAmountError::display`](../core/gemstone/src/services/amount/model.rs).

- **F9** **S** `GemConfirmError` — both apps format `requirement.required`, `available` and `shortfall` with the full style and compose the same three-part message, in [iOS](../ios/Features/Transfer/Sources/Extensions/GemConfirmError+Localizations.swift) and [Android](../android/features/confirm/presents/src/main/kotlin/com/gemwallet/android/features/confirm/presents/components/ConfirmErrorInfo.kt). iOS also decides `hasInfoSheet` per variant while Android maps its own info-sheet entity, so the same per-variant question is answered twice. Give it a `display()` the way `GemAmountError` has one.
- **F10** **S** `SwapperError` — Android re-cases it into an app-side [`SwapFailure`](../android/features/swap/viewmodels/src/main/kotlin/com/gemwallet/android/features/swap/viewmodels/models/SwapFailure.kt) twin, and the two apps compose different text: [iOS](../ios/Features/Swap/Sources/Extensions/SwapperError+Swap.swift) switches to "minimum amount X" when a minimum is known, Android appends the minimum to "amount too small". Give `SwapperError` a `display()` and delete the twin.
- **F11** **S** Perpetual market values — `perpetual.volume24h` and `perpetual.openInterest` cross as bare `f64` and both apps abbreviate them in USD: [iOS](../ios/Features/Perpetuals/Sources/ViewModels/PerpetualViewModel.swift), [Android](../android/data/coordinators/src/main/kotlin/com/gemwallet/android/data/coordinators/perpetuals/GetPerpetualImpl.kt).
- **F12** **S** [`GemPriceAlertRow`](../core/gemstone/src/services/price_alert/rules.rs) carries `price` and `percent` as bare `f64` and each app picks the style from a different input: [iOS](../ios/Features/PriceAlerts/Sources/ViewModels/PriceAlertItemViewModel.swift) switches on `row.kind`, [Android](../android/data/coordinators/src/main/kotlin/com/gemwallet/android/data/coordinators/pricealerts/GetPriceAlertsImpl.kt) on whether `pricePercentChange` is set, so the percent sign can differ for the same alert.
- **F13** **S** Rewards redemption — `option.value` is formatted with the short style and the option's asset on [iOS](../ios/Features/Settings/Sources/Settings/ViewModels/RewardRedemptionOptionViewModel.swift) and [Android](../android/features/referral/presents/src/main/kotlin/com/gemwallet/android/features/referral/views/components/ReferralInfo.kt).

Not in scope: `Formatters` and `Validators` on iOS still cannot import Gemstone, so the renderer that applies a `GemPrecision` must stay dependency-free. That is why this is a value-plus-style contract and not a foreign trait.

## 6. Platform parity

Core exports these and one app calls them. Establish whether it is a missing feature or a duplicated decision first. Exports that belong to a screen already listed above are noted on that item (V2, S3, S5, S9, S16, F8).


Legitimately one-sided, not gaps: `isVersionHigher` (Play update), `migrateToSharedPassword` (Android password store), `set_price_alerts_enabled` (Android one-off migration), `signWithKeystore` (iOS keystore), `isOriginRejected`, `authentication_chain_ids`, `authentication_accounts`, `authentication_methods` in the auth flow (Android-only one-click auth, D4; proposal and sign check the origin inside Core on both), `scanTransaction` (both scan through `GemConfirmService`).

## 7. Everything else

Product or security decisions, one question each:


Ownership, injection and threads:

- **O9** **M** Six Android view models hold two or three Core services where iOS composes one screen service: [`TransactionsViewModel`](../android/features/activities/viewmodels/src/main/kotlin/com/gemwallet/android/features/activities/viewmodels/TransactionsViewModel.kt), `RecentsSheetViewModel`, `WCAuthViewModel` (three), `WCRequestViewModel`, `PerpetualMarketViewModel` and `SettingsViewModel`. Copy the screen-service shape in [SERVICES.md](SERVICES.md#the-screen-service-map); a launch host or a flow parent vending child models is [allowed to hold several](ARCHITECTURE.md#7-at-most-one-core-service-on-ios-narrow-cases-on-android) and these are neither.



Platform items:

- **X18** **S** Two exports have no production reader on either app, only mocks and tests: `name_record_debounce_milliseconds` in [`services/name/mod.rs`](../core/gemstone/src/services/name/mod.rs), superseded by the debounce `name_input_step` already returns, and `requested_name` on [`GemNameRecordState`](../core/gemstone/src/services/name/model.rs). Un-export both and keep the Rust functions, which Core calls. Of 523 exported methods these are the only two without a caller a test does not create; the rest of that sweep is dry.






Guides:


Do not "fix" the [deliberate divergences](SERVICES.md#deliberate-divergences--do-not-fix-these).
