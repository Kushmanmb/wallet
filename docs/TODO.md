# Open work

**Refilled on 2026-09-16.** Every id from the 2026-09-15 sweeps is closed — landed, or closed against evidence, with the reasoning kept in the lower half of this file so the same lead is not re-raised. Sections 16–25 are the app-shaped pass, built from lenses the structural sweeps miss: the row census, a composed-label pass, client-side arithmetic, invented failures, call-site thresholds, collection shaping, client-side time, hand-built URLs and cross-app member-name collisions. Sections 26–30 are the Core-shaped pass over the same corpus: records that cross with a bare number, screens holding no Core service at all, rules duplicated between Core crates, the gaps the screen-service map already names, and app ports Core could own. No test items: coverage is tracked by the decisions a screen makes, not by file names.

Every open item carries a stable id (V vocabulary, R rows, C composition, S sessions, B view boundary, F formatting, P parity, D decisions, O ownership, X platform, G guidance, T tests, L localization, N naming, PERF performance) and a size (**S**/**M**/**L**). Contracts are in [ARCHITECTURE.md](ARCHITECTURE.md) and [SERVICES.md](SERVICES.md). **Delete an item's line in the commit that lands it** — ids are never reused.

The goal is that Gemstone decides once and both clients read that decision. Track duplicated decisions and concrete performance work at their existing owners: shared rules and orchestration in Core; rendering, observation, scheduling, and localized formatting in the apps.

Keep each item independently reviewable. Shared decision changes land in Core and both apps; platform-only work stays on that platform. Regenerate only when shared interfaces or integration change, and run the applicable [Quality Checks](../skills/quality-checks.md). Verify affected primary-screen journeys under [Performance](PERFORMANCE.md). Remove replaced paths within the item's scope; do not bundle an unrelated row migration or product change.

This list was rebuilt on 2026-09-15 from scripted sweeps over the whole repo and widened the same day by a second, deeper pass. Each item names the file or symbol the sweep hit, so it can be confirmed before it is started; a sweep hit is a lead, not a verdict, and an item that turns out to be correct as written is closed by deleting its line with a one-line note in the commit.

These files were checked during the 2026-09-15 mapper sweep and need no change — each calls its module mapper or switches over an app type, not a Core one: iOS `NFT/CollectibleViewModel`, `Settings/GemAddNodeFailure+Settings`, `Swap/SwapSlippageViewModel`, `Swap/Views/SwapDetailsView`, `Transfer/Types/ConfirmInfoSheetBuilder`, `Transfer/ConfirmRecipientViewModel`, `Transfer/RecipientSceneViewModel`; Android `earn`, `import_wallet`, `perpetual`.

## 7. Platform

### Hardcoded dp (Android rule: theme constants only)

Re-checked on 2026-09-15 by separating a `\d+.dp` written at a call site from one written as a named constant: **Android has none of the former**. All 58 remaining sites are `private val name = N.dp` declarations, which is the pattern the codebase already uses for a component's own dimensions, and only a handful of those carried a value the theme also names for the same kind of thing — those now read from the theme. What is left is a QR finder's 25 dp corner, a 42 dp search bar, a 296 dp slippage sheet and the like: dimensions with no theme equivalent, each named where it is used. X23–X29 are closed with no change.

### Swallowed errors

Closed on 2026-09-15. Four of the six were the same `try { focusRequester.requestFocus() } catch {}` written out in four screens — Compose throws when the target is not attached yet, so the catch is right and the duplication was the problem; they now call `requestFocusIfAttached()`. The other two were real: binding the camera use cases swallowed its failure, so a camera that could not start showed a blank preview with nothing in logcat, and the collectible details flow swallowed its load error and stayed null forever. Both report now.

### Deferred notes still in the code

All five closed on 2026-09-16, each against evidence rather than a re-read.

**X42** is fixed in code and needed no install data after all. `CetusAggregator` was a live provider that wrote `cetus_aggregator` into `TransactionSwapMetadata.provider`, and the stored value is a free string, so deleting the variant never broke a stored row — it broke reading one back, in the single place that parses it (`get_transaction_swap_url`). The variant is gone and `cetus_aggregator` is now an alias onto `CetusClmm` for both `FromStr` and serde, so an old Sui swap keeps its "Cetus" name and its explorer link. Neither app parses the enum from a stored row; both only pass the string to Core.

**X41** had its premise backwards. `from_locale_identifier` lowercases the language and hands the bare tag to `from_client`, so `pt-PT` reaches the `"pt"` arm and `sv-SE` reaches the long English fallback list — both are load-bearing for the clients shipping today, not legacy. The only arm the normalizer never reaches is the bare `"zh"`, because it always names the script; that one is the wire-level safety net for a raw value. A test now asserts the dependency so the list is not "cleaned up" later.

**X35** was re-tested, not re-read: the pin was removed and uniffi bumped to 0.32.1, the newest release. Both `uniffiTraitInterfaceCallAsync` sites still fail with "passing closure as a 'sending' parameter", because the generated `Task { }` captures three `@escaping` non-`Sendable` parameters. The pin is correct and the bump is not the fix; re-test after a uniffi release that changes that function.

**X36** is decided rather than deferred. `FileMigrator.migrate` returns the new URL untouched once the old path is empty, so the standing cost is one `fileExists` per launch, and the downside of deleting early is a lost wallet. It stays. Install numbers would only ever justify saving one stat call, which is not worth a tracked item.

**X40** keeps its dated note beside the route in `core/apps/api/src/devices/mod.rs`. The date is the tracker; a backlog line restating a comment that already carries the deadline is duplicate bookkeeping.

## 11. Missing tests

### Hardcoded user-visible strings

Swept on 2026-09-15 over every non-preview, non-test iOS file: 25 hits, of which 20 are the developer screen (a debug screen that is deliberately untranslated) and 4 are inside a `PreviewProvider`. The one real hit was the wallet screen's "Trade Perpetuals" row, now `perpetuals_trade`. The same sweep over Android found none. All three new keys — `perpetuals_trade`, `widget_empty` and `widget_empty_short` — were translated into the other 30 locales on 2026-09-16, each following the locale's existing `perpetuals_title` and `errors_no_data_available` wording.


T29 closed on 2026-09-16. `widget_empty` and `widget_empty_short` are translated in all 31 locales. What is left is one rule — how many coins each widget family shows — and a row model that formats a price, a percentage and a colour. Both would cost a new app-extension test target plus the package-product link that T28 found to be broken, which is more scaffolding than the rule is worth; the rule belongs in the same fix.

T28 closed on 2026-09-16 after trying it. `ScanReceiveModeViewModel` is an id and a title, `MainTabViewModel` is one `ObservableQuery`, and `RootSceneViewModel` is the composition root: thirteen collaborators and a body that is `Task { await service... }` in every method, so a test of it is a test of the mocks. The one rule worth covering — a required upgrade offers only the update action — sits behind that constructor. Wiring the existing empty `GemTests` unit-test target into `unit_frameworks.xctestplan` was tried and does not link: attaching any package product to a test bundle makes Xcode rebuild `Store` as a dynamic package product, and that product does not resolve `BigInt`, so the link fails before the tests run. That is the thing to fix first if the app target ever needs tests.

T27 closed on 2026-09-16. `ReceiveViewModel` was already covered by eleven tests — the count was by file name again. `AmountEarnViewModel` and `PerpetualModifyViewModel` decide something and are tested; `KeystoreAuthenticationViewModel` is a three-case glyph map and `ReceiveNetworkSelectorViewModel` builds its items from the ids it is handed, which is the mapper contract, not a decision.

T26 closed on 2026-09-16 with one file. `SwapTokenViewModel` decides what the pay and receive rows allow and prices what was typed, and is tested; `SwapProvidersViewModel` is a `SelectableListAdoptable` whose three members are localized constants and `SwapPairSelectorViewModel` is two optional asset ids with no logic at all — a test over either would assert the compiler.

### iOS view models with no test file

Counting by file name is what made this section long, and it was wrong three times: `Features/Transactions` (T21), `ReceiveViewModel` (T27) and `ConfirmViewModel` (T37) were all already covered from differently named files. Count the decisions, not the files.


T37 closed on 2026-09-16. `ConfirmViewModel` already had eight tests across four files named for what they cover — the header, the request, the retry and the network fee sheet — so the count by file name missed it again. `NetworkFeeCustomViewModel` was the real gap and is tested: it opens on the rate it was handed, it never lets a letter reach the rate, and a rate over the maximum cannot be confirmed.

T33 closed on 2026-09-16. `AutocloseViewModel` and `PerpetualDetailsViewModel` are tested; `PerpetualsPreviewViewModel` is two `stateIn` passthroughs over a config flag and the position list, with nothing of its own to assert.

T31 closed on 2026-09-16. Four of the five — `BuySelectViewModel`, `SendSelectViewModel`, `ManageSelectViewModel`, `ReceiveSelectViewModel` — are one-line bindings of a `GemSelectAssetType` to `BaseAssetSelectViewModel`, so the base is what was tested: the chain filter narrows the list, clearing the filters puts it back, and pinning an asset tells Core and names it in the toast. Two more were dropped after being written: the recent list reacts to `snapshotFlow { queryState.text }`, which needs Compose snapshot notifications a plain JVM test does not dispatch, and an asset carrying a balance never reaches `assetsContent` in a JVM test, so the balance filter cannot be exercised from outside the view model.

### Android view models with no test

All closed. The pattern that recurred: a feature's view models are usually one orchestrator plus several one-line bindings of an enum to it, so the orchestrator is what earns a test.


## 14. Decisions still made on a client

The standing goal, restated on 2026-09-16: **any business logic moves to Gemstone, and nothing is decided twice on the clients.** The sweep that finds this work compares the two apps rather than reading one: the same computation on both sides, or one app calling Core where the other does the arithmetic itself, is a divergence waiting to happen — not a style difference. Four landed on 2026-09-16:

- Rejecting a WalletConnect proposal. iOS mapped the error to a CAIP-25 reason and deleted the stored session; Android sent the string `"Reject Session"` for every rejection and kept the session. `session_rejection` now returns the reason, the code, the message the dApp sees and whether the session is deleted.
- The slippage percent. Android asked Core; iOS wrote `Double(bps) / 100` in two places, and Android wrote it a third time in `SwapDetailsUIModelFactory`. All three read `slippage_percent` now, and `GemSwapQuoteSummary` carries it for the screens that hold the summary but no service.
- Margin usage on the portfolio screen. Both apps computed `account_value * usage` and `usage * 100` and composed the same `"value (percent)"`. `PortfolioMarginUsage` carries `used_value` and `usage_percent`.
- Crypto to fiat. iOS converted through `CryptoFiatConverter`; Android multiplied two doubles in `BalanceInfoUIModel`, `DelegationInfo` and `AssetPriceValue`, which loses Core's precision rules. All of them go through Core now, and `GemSwapValue::fiat_value` is exported for the swap screens.

Crypto to fiat came back on 2026-09-16: four iOS sites still read the atomic value into a `Double` and multiplied it by the price — the asset balance, the swap pay row, the swap provider row and both delegation rows — which is the precision loss Android had already been fixed for. `CryptoFiatConverter.to_fiat` no longer throws (a price that is not a number is not a price, and a `BigInt` always stringifies), and `PriceViewModel.fiatValueText(value:decimals:)` is the one iOS path into it.

Checked and clean: the price-impact model, the swap rate text and Android's `EquivalentValue` all read a Core value and map only the label, the colour or the locale format.

A fifth landed the same day: the developer screen. iOS held five stores beside its service and built an eleven-row table of sample transactions by hand; Android held the service alone and could offer none of the actions. `GemDeveloperStore` names the seven database operations, `sample_transactions` owns the table, and both screens now reach the database only through `GemDeveloperService`.




## 15. More platform work

### iOS errors thrown away

Re-checked on 2026-09-15. `Store/Migrations.swift` holds 88 of the 174 sites and they are the idempotent-migration idiom — `try? db.alter` for a column that may already exist, `try? db.drop` for a table that may not. Rewriting those against a shipped wallet database is a data-loss risk with no defect behind it, so they stay. `clearChainData` was the one that was wrong: it deleted a removed chain's rows from seven tables with `try?`, so a locked table or a constraint left the rows behind silently. It now asks `tableExists` the way `clearTables` beside it already did and lets a real error through.

`WalletIdMigration.swift` had the same shape and the same one real problem: it rewrote `walletId` across ten child tables and deleted a wallet's child rows with `try?`, so a failure on any one of them left the wallet's data pointing at an id that no longer exists, and `cleanupOrphanedRecords` then swallowed the foreign-key check as well. All three now skip a table that does not exist and let a real failure roll the migration back.

Checked and kept: `LocalKeystore.findV3File` scans a directory and `try?` is how an unreadable file is skipped; `SwapSceneViewModel.currentInput` returns nil because "no complete input yet" is what its errors mean; `NavigationPathState` and `NavigationHandler` decode a path or a wallet id where nil is a real answer.

The rest are one or two per file and each needs reading on its own.

X74 is closed after reading the remaining files. One was wrong: `RewardsViewModel` built the referral link with `try? ... ?? ""`, so a link the service could not build was shared and copied as an empty string instead of the action disappearing; `referralLink` now returns nil and logs, and `shareText` follows it. The rest are the nil-is-the-answer shape — `try? AssetId(id:)` on a deeplink string, `try? NumberInput.value` on half-typed input in the swap and delegation rows, `try? service.suggestPair` where no suggestion is a normal outcome — plus the developer screen, which is deliberately forgiving.

`AnyCodableValue.swift`, `AssetId.swift` and `AnyCodableValue+Store.swift` (X73) are closed with no change. All fifteen sites are the decoder-probe idiom: `try? container.decode(Bool.self)` asks "is this a bool", and the answer "no" is what drives the next branch — the last one throws a real `DecodingError` when nothing matched, and `AssetId` falls through from the string form to the keyed form. The `decode(_:)` and `encode(_:)` helpers return an Optional on purpose; nil means "the value is not that type", which is the same answer. Rewriting any of them would turn a branch condition into a thrown error with nowhere to go.


`WalletConnectorService.swift` (X71) held the three worst of them and is done. Two were the reject path swallowing its own failure, which now logs. The third encoded a WalletConnect request's params with `try?` and fell back to `""`, so a request whose params would not encode reached Core as a request with no parameters; it now rejects the request instead. Rejecting a proposal also stopped being decided twice — see [the finished value](ARCHITECTURE.md) — so Core names the reason, the code, the message and whether the session is deleted.



### Logging left in shipping paths

Re-checked on 2026-09-15 by separating what actually ships: 116 of the 122 iOS sites are `debugLog`, which compiles to nothing outside `DEBUG`, and the remaining six `print` calls are inside a `#Preview` or a macOS-only availability note. On Android 60 of the 69 are `Log.e` on a real failure. The seven `Log.d` that shipped are gone — they were logging WalletConnect request method, chain and id, the whole stream payload, and connection transitions into logcat on a release build — and the two migration errors that were logged at debug level now log as errors. X75–X83 are closed.

### iOS layout numbers outside Style

Re-checked on 2026-09-15 the way the Android dp items were: most of the 31 hits are inside a `#Preview` or are a named configuration field (`QRScannerDisplayConfiguration.default`), which is the pattern. What was real: a segmented picker width repeated in three scenes and a chart height repeated in two now read `Sizing.picker.segmentedWidth` and `Sizing.chart.height` (Android has named the chart height all along), and the `spacing: 0` / `cornerRadius: 10` / `spacing: 24` call sites read from `Spacing`. X84–X88 are closed.


### Files that have outgrown one module, second pass

All five closed on 2026-09-16 after measuring what each length is made of.

**X89** and **X90** are the same file in two languages and both say so on line one: `RemoteTypeMappers.kt` and `models/remote_types.rs` are emitted by `just generate-models` from `core/bin/generate/remote_types.yml`. The answer to "how many the generator could emit" is all of them; the length is the number of types that cross the FFI.

**X91** `chain_config.rs` is a table — the config types, then one `ChainConfig { .. }` literal per chain, 102 of them. **X92** `message/signer.rs` measured 771 file lines and 231 production lines; the rest is its test module, the same miscount as X46–X52. **X95**'s `GemCandlestickChart.kt` is one chart and its private drawing helpers, and `AmountListHead.kt` is one component family — five composables sharing a private item type and private layout state — so splitting it means widening those to `internal` and trading file length for a wider surface.

### The shared localized mapper

**X94** closed on 2026-09-16 with no change. `PrimitivesComponents/Extensions/Gemstone+Localized.swift` is 35 extensions that each give one Core enum its localized label, and the length is the number of Core enums the apps draw, not a mix of concerns — the style half already lives beside it in `Gemstone+Style.swift`. One shared mapper per app is the rule ([ARCHITECTURE.md](ARCHITECTURE.md)); splitting it into several files is what that rule exists to prevent, and a module that wants its own copy is the mistake the rule catches. It grows when Core names a new outcome, which is the contract working.

### The two Android migration files

**X96** closed on 2026-09-16. Both are reachable: `provideRoom` registers every step from 41 to 71 by hand and then `gemDatabaseMigrations` from 71 to the current 94, so the chain from the oldest schema is unbroken and neither can be deleted without breaking an upgrade from an old install. `Migration_71_72.kt` stays as one file — it is one schema step whose nine ordered helpers have to run in one transaction, and the comment at the top says why the order matters. `Migration_41_42.kt` was the real find: it held three migrations (41→42, 42→43, 43→44) under one migration's name, which is why it measured long. Each now has its own file, the way every migration from 44 on already did.

### The services factory

**X97** closed on 2026-09-16 with no change. `ServicesFactory.makeServices` is one function because it is one dependency graph: 91 local bindings in construction order, each feeding the next, ending in a single `AppResolver.Services`. `ViewModelFactory` split cleanly (X54) because every screen constructor is independent of the others; here a split into `makeCoreServices`, `makeWalletServices` and so on would have to thread twenty-odd intermediate values between the halves, which is more moving parts than the straight line it replaces. The length tracks the number of services the app has, not a mix of concerns.

### App-side twins of Core types

[No hand-written twins](ARCHITECTURE.md): an FFI-only type is used as the uniffi type; a twin is only for a type an app persists. Each of the three below has the same cases and the same payload types as its Core counterpart and is never written to storage.

Checked and kept: `KeystoreAuthentication` and `LockPeriod` are both written to the keychain by raw value, which the rule allows; `AmountType` carries a recipient its Core namesake does not; `SelectAssetType` and `PaymentDestination` are navigation types carrying app payloads and already map to Core through `flowType`.

## 16. Presentation still decided on a client

Rebuilt on 2026-09-16 from the row census and a composed-label pass: a `*ViewModel`/`*UIModel` that declares three or more label-shaped `String` members and names no Core `Row`/`ViewState`/`Details`/`Sections` record is deciding its own presentation, and a label whose body interpolates, concatenates or branches is a decision rather than a lookup. Each item names the file the sweep hit; confirm the member before starting, because a member that is one `Localized.` constant is the mapper contract working.

- **R54** **M** `ios/Features/WalletConnector/.../ConnectionProposalViewModel.swift` — 11 label members and no Core record; `appName` and `websiteText` are both composed. Android reads `GemConnectionRow`/`GemConnectionDetails` for the same screen.
- **R55** **M** `ios/Packages/PrimitivesComponents/.../AssetDataViewModel.swift` — 10 label members. The asset row record already exists in Core for the list; the detail screen still composes its own.
- **R56** **M** `ios/Features/Stake/.../DelegationViewModel.swift` — 8 label members plus `rewardsText`, which Android also declares in `StakeViewModel.kt`.
- **R57** **M** `ios/Features/Transfer/.../AmountSceneViewModel.swift` — 7 label members; `assetName` is declared on Android too, in `RecipientError.kt`.
- **R58** **S** `ios/Packages/PrimitivesComponents/.../BalanceViewModel.swift` — 6 members; `energyText` and `bandwidthText` are composed from a Tron resource pair that Core already models.
- **R59** **S** `ios/Packages/PrimitivesComponents/.../AddressListItemViewModel.swift` — 6 members and no Core record.
- **R60** **S** `ios/Features/Onboarding/.../ImportWalletSceneViewModel.swift` — 6 members; the import type list is a product decision.
- **R61** **S** `ios/Features/Contacts/.../ManageContactViewModel.swift` — 6 members beside `GemManageContactService`, which already answers the screen.
- **R62** **S** `ios/Packages/PrimitivesComponents/.../AssetViewModel.swift` — 5 members; the canonical asset title lives in Core.
- **R63** **S** `ios/Features/Settings/.../RewardRedemptionOptionViewModel.swift` — 5 members; the redemption rows landed in Core (b638ab54a9) but this model still composes.
- **R64** **S** `ios/Features/MarketInsight/.../MarketValueViewModel.swift` — 5 members over market statistics Core already carries.
- **R65** **S** `ios/Packages/PrimitivesComponents/.../ChartHeaderViewModel.swift` — 4 members, and `dateText`/`headerValueText` are both declared on Android in `ChartHeaderUIModel.kt`. The same header is composed twice.
- **R66** **S** `ios/Features/Swap/.../PriceImpactViewModel.swift` — 4 members; `showsInSummary` is also declared in Android's `SwapDetailsUIModel.kt`.
- **R67** **S** `ios/Features/Support/.../SupportChatSceneViewModel.swift` — 4 members over a chat transcript Core already groups.
- **R68** **S** `ios/Packages/PrimitivesComponents/.../PerpetualDetailsViewModel.swift` — `positionText`, `leverageText` and `listItemSubtitle` are all composed.
- **R69** **S** `ios/Features/Perpetuals/.../AutocloseViewModel.swift` — `title`, `profitTitle` and `percentText` are composed beside a Core autoclose session that already returns a view state.
- **R70** **S** `ios/Features/Assets/.../AssetSceneViewModel.swift` — `pinText` and `enableText` branch on state to pick a verb.
- **R71** **S** `ios/Features/Perpetuals/.../PerpetualSceneViewModel.swift` — `navigationTitle` is composed from the pair name.
- **R72** **S** `ios/Features/PriceAlerts/.../SetPriceAlertViewModel.swift` — `completeMessage` joins a lowercased direction label with an amount, which is sentence construction in the app.
- **R73** **S** `ios/Features/Settings/Currency/.../CurrencyViewModel.swift` — `title` falls back through `Locale.current.localizedString(forCurrencyCode:)`; Android has its own currency naming.
- **R74** **S** `ios/Features/Transfer/.../ReceiveViewModel.swift` — `copyTitle` and `warningMessage` are composed; the warnings come from Core but the joining does not.
- **R75** **S** `ios/Features/WalletTab/.../PortfolioSceneViewModel.swift` — `navigationTitle` and the statistic `title` are chosen in the model.
- **R76** **S** `ios/Packages/PrimitivesComponents/.../SimulationPayloadFieldViewModel.swift` — `subtitle` picks between an address name and a raw value.
- **R77** **S** `ios/Packages/PrimitivesComponents/.../TransactionViewModel.swift` — `title` is chosen by a badge flag and again by a `switch` over the row subtitle.
- **R78** **S** `android/ui-models/.../chart/ChartHeaderUIModel.kt`, `chart/CandlestickChartUIModel.kt` — the Android half of R65, same three label members.
- **R79** **S** `android/features/bridge/.../WCRequestViewModel.kt` — three label members and two Core services; the other half of R54.
- **R80** **S** `android/ui-models/.../perpetual/autoclose/AutocloseUIModel.kt` — the Android half of R69.

## 17. Numbers the app derives

A rendered number the app computes is the same class of bug as the fiat multiplication fixed on 2026-09-16: the app reaches a `Double` and loses Core's precision rules. These are the remaining sites the arithmetic sweep found outside `Formatters`.

- **D11** **S** `ios/Packages/PrimitivesComponents/.../NumericViewModel.swift:57` — `currencyFormatter.string(quote.price * value)` is the fifth client-side fiat multiplication; the four found on 2026-09-16 now go through `CryptoFiatConverter`.
- **D12** **S** `ios/Packages/Primitives/Sources/Extensions/BigInt+Primitives.swift:29` — `self * BigInt(percent) / 100` is a percentage-of-amount rule written in the app; Core owns bps and percent conversion.
- **D13** **S** `ios/Packages/Primitives/Sources/ChartValues.swift:42` — the x-axis is padded by `timeIntervalSince(first) * 0.02`; Core already owns candlestick geometry (08f9789016).
- **D14** **S** `ios/Packages/Components/Sources/Interval.swift:13` — `Interval(value) * 60` converts minutes in the app.
- **D15** **S** `ios/Packages/Primitives/Sources/Extensions/Double+Primitives.swift` `rounded` and `android/gemcore/.../ValueFormatter.kt` `rounded` — the same rounding helper on both apps.
- **D16** **S** `ios/Packages/Formatters/.../BigNumberFormatter.swift` `decimal` and `android/gemcore/.../NumericFormatter.kt` `decimal` — the same decimal parse on both apps.
- **D17** **S** `ios/Packages/Formatters/.../ValueFormatter.swift` and `android/gemcore/.../ValueFormatter.kt` both declare `formattedDustThreshold`; the dust *predicate* is Core's `is_value_dust`, the *threshold text* is written twice.
- **D18** **S** `ios/Packages/PrimitivesComponents/Sources/Types/AmountDisplay.swift:131` — the sign prefix is picked from `value > 0` / `value < 0`; Android does the same in `AutocloseUIModelFactory.kt:70` and `GetWalletSummaryImpl.kt:125`.
- **D19** **S** `android/gemcore/.../domains/price/ValueDirection.kt:13` — up/down/flat from a raw comparison; Core names the direction on the row it already returns.
- **D20** **S** `android/ui/.../list_item/transaction/TransactionDataAggregateExt.kt:57` and `android/app/.../widgets/PricesWidget.kt:163` — profit colour chosen from `pnl > 0` in two places, with the widget hardcoding hex colours.
- **D21** **S** `android/ui/.../chart/GemLineChart.kt:341,366` — chart point averaging and x-position are computed in the composable.
- **D22** **S** `ios/Features/Perpetuals/.../AutocloseViewModel.swift:44,59` — `isProfit` and the sign are derived beside a Core estimator that already answers both; `android/ui-models/.../AutocloseUIModelFactory.kt:55,70` is the same rule.
- **D23** **S** `ios/Packages/PrimitivesComponents/.../PriceViewModel.swift:60` — the price-change background colour branches on `priceChange > 0`.
- **D24** **S** `ios/Features/WalletTab/.../PortfolioSceneViewModel.swift` and `android/.../PortfolioChartViewModel.kt` both declare `availablePeriods`; Core's `PortfolioData` already carries them.
- **D25** **S** `ios/Features/Perpetuals/.../CandlestickChartViewModel.swift:95` — nearest-candle selection by absolute time distance is written in the app.

## 18. Errors the app invents

The rule from 2026-09-16: a `try?` or a `runCatching { }.getOrNull()` that turns a real failure into a silent default is the app inventing a failure mode. `Migrations.swift` (80 sites) and the decoder probes in `AnyCodableValue.swift` are already closed as the idempotent-migration and type-probe idioms; these are the rest.

- **F19** **S** `ios/Features/Swap/.../SwapSceneViewModel.swift` — four sites; `currentInput` is closed as nil-is-the-answer, the other three are not.
- **F20** **S** `ios/Packages/GemstoneServices/Sources/Keystore/LocalKeystore.swift` — four sites in a wallet-critical path; `findV3File` is closed, the rest need reading.
- **F21** **S** `ios/Packages/Primitives/Sources/Generated/ListItem.swift` — four sites inside a generated file, which means the generator emits them.
- **F22** **S** `ios/Gem/Navigation/NavigationHandler.swift` and `ios/Packages/Components/Sources/NavigationPathState.swift` — three each; the path-decoding ones are closed, the wallet-id ones are not.
- **F23** **S** `ios/Packages/Formatters/Sources/RelativeDateFormatter.swift` — two sites in date formatting.
- **F24** **S** `ios/Packages/PrimitivesComponents/.../SwapMetadataViewModel.swift` — two sites reading a stored swap's metadata, which is exactly the shape the Cetus alias fixed in Core.
- **F25** **S** `ios/Packages/SwiftHTTPClient/WebSocketClient/WebSocketConnection.swift` — two sites on a live socket.
- **F26** **S** `ios/Features/MarketInsight/.../ChartSceneViewModel.swift`, `ios/Features/NFT/.../CollectibleViewModel.swift`, `ios/Features/Assets/.../SelectAssetViewModel.swift`, `ios/Features/QRScanner/.../QRScannerSceneViewModel.swift`, `ios/Features/Support/.../SupportChatSceneViewModel.swift` — one site each.
- **F27** **S** `android/gemcore/.../serializer/RoutePayload.kt` — two `runCatching { }.getOrNull()` on route payload decoding.
- **F28** **S** `android/data/coordinators/.../session/SessionCoordinator.kt` — a swallowed failure in the session path.
- **F29** **S** `android/data/coordinators/.../update/AppUpdateCoordinator.kt` — `runCatching { appUpdateService.check(...) }.getOrNull()` hides an update check failure as "no update".
- **F30** **S** `android/data/coordinators/.../wallet_connect/WalletConnectCoordinator.kt` and `.../perpetual/HyperliquidObserverService.kt` — one each on live connections.
- **F31** **S** `android/features/recipient/.../RecipientViewModel.kt`, `.../referral/ReferralViewModel.kt`, `.../settings/currency/CurrenciesViewModel.kt`, `.../receive/presents/components/QRCode.kt` — one each.

## 19. Thresholds and limits written at a call site

A comparison against a literal in a view model is a product rule with no name. The sweep excluded layout numbers, `AuthenticationPolicy` bit flags and SQL.

- **S19** **S** `ios/Features/Assets/Sources/Types/AddAssetInput.swift:12` `chains.count > 1`, `ios/Features/Onboarding/.../ImportWalletSceneViewModel.swift:87` `importTypes.count > 1`, `ios/Features/Settings/.../RewardsViewModel.swift:107` `wallets.count > 1` — "more than one, so offer a picker" written three times on iOS.
- **S20** **S** `ios/Packages/PrimitivesComponents/.../AssetDataViewModel.swift:101,117` — "has a balance" as `> 0` in two places; `ios/Features/Perpetuals/.../PerpetualsHeaderViewModel.swift:59` and `ios/Features/Assets/.../AssetSceneViewModel.swift:201` repeat it.
- **S21** **S** `ios/Features/Stake/.../EarnSceneViewModel.swift:92` — `.filter { BigInt($0.base.balance) > 0 }` decides which delegations show.
- **S22** **S** `ios/Features/NFT/.../CollectionsViewModel.swift:40` and `android/features/nft/.../NftListScene.kt:116` — the unverified-collections row appears when the count is above zero, decided on both apps.
- **S23** **S** `ios/Features/Swap/Sources/Types/SwapValueFormatter.swift:17` — a zero guard in front of swap value text.
- **S24** **S** `android/data/services/store/.../entities/DbAssetInfo.kt:124,146` — resource metadata and price presence decided by `> 0` while mapping a row out of the database.
- **S25** **S** `android/features/asset_select/.../BaseAssetSelectViewModel.kt:126` — the balance filter is `it.balance.totalAmount > 0.0` in the view model.
- **S26** **S** `android/ui-models/.../chart/CandlestickChartUIModel.kt:57` — `if (tickCount < 2) return emptyList()`.
- **S27** **S** `android/app/.../di/ClientsModule.kt:29-31` — connect timeout, read timeout and the idle connection pool are literals; iOS sets its own in `URLSessionConfiguration`, so the network budget is decided twice.
- **S28** **S** `android/features/update_app/.../InAppUpdateServiceImpl.kt:42-43` — a second, different pair of HTTP timeouts inside the same app.
- **S29** **S** `ios/GemPriceWidget/Widget/PriceWidgetProvider.swift:33` one minute vs `android/app/.../widgets/WidgetPriceSyncWorker.kt:34` `REFRESH_INTERVAL_MINUTES` — the widget refresh cadence is a product decision made twice.
- **S30** **S** `ios/Packages/PrimitivesComponents/.../CopyTypeViewModel.swift:63` — the pasteboard expiry interval is set in the app; Android's clipboard path has its own.
- **S31** **S** `ios/Features/WalletTab/.../WalletSearchSceneViewModel.swift:173,181` — the section caps come from Core's `limits` but the `prefix` is applied app-side on iOS only.
- **S32** **S** `ios/GemPriceWidget/.../PriceWidgetViewModel.swift:23,25` — one coin for the small family, three for the medium; the Android widget picks its own counts.
- **S33** **S** `android/data/services/store/.../PerpetualDao.kt:34` — `WHERE volume24h > 0` decides which markets exist, in SQL.

## 20. Ordering, filtering and grouping in app code

Which rows exist, and in what order, is a product decision. The sweep skipped stores and DAOs except where the query encodes a rule.

- **C10** **S** `ios/Features/WalletTab/.../NetworkAssetsSceneViewModel.swift:65,77` — `filter { $0.asset.type != .native }` twice to split active from hidden.
- **C11** **S** `ios/Features/PriceAlerts/.../AssetPriceAlertsViewModel.swift:67` — `filter { $0.priceAlert.type != .auto }` decides which alerts are listed.
- **C12** **S** `ios/Features/Recents/.../RecentsSceneViewModel.swift:78` — recents are intersected with a matching set in the model.
- **C13** **S** `ios/Features/Stake/Sources/Scenes/DelegationScene.swift:30` — `model.rows.filter { $0 != .rewards }` filters a Core row list inside the view.
- **C14** **S** `ios/Packages/Components/Sources/ListViews/Types/ListSearch.swift:30` — section search filtering is a shared component rule with no Core counterpart.
- **C15** **S** `ios/Features/ManageWallets/.../WalletsSceneViewModel.swift:60` calls `service.sorted(wallets:)` — confirm the Android list uses the same Core call and is not sorting itself.
- **C16** **S** `android/gemcore/.../application/wallet/cases/GetAllWallets.kt` and `.../coordinators/wallet/GetAllWalletsImpl.kt` — wallet ordering decided in an application case.
- **C17** **S** `android/gemcore/.../application/pricealerts/cases/GetPriceAlerts.kt` and `.../coordinators/pricealerts/GetPriceAlertsImpl.kt` — the Android half of C11.
- **C18** **S** `android/data/coordinators/.../stake/StakeReadsImpl.kt` — delegation filtering beside a Core stake service.
- **C19** **S** `android/data/services/gemstone/.../stores/NftStore.kt` and `.../stores/SwapStore.kt` — filtering inside a store adapter, which is where a Core rule should have been passed in.
- **C20** **S** `android/features/assets/.../WalletSearchViewModel.kt` — the Android half of S31.
- **C21** **S** `android/features/asset_select/.../RecentsSheetViewModel.kt` — recents ordering in the sheet model.
- **C22** **S** `android/app/.../PaymentNavigation.kt` — a filter decides which payment destination is taken.
- **C23** **S** `android/features/confirm/presents/components/FeeDetails.kt` — fee rows filtered in the composable.
- **C24** **S** `ios/Features/Stake/.../ValidatorSelectSceneViewModel.swift` — validator list shaping beside a Core stake service.

## 21. Time decided on a client

- **P63** **S** `ios/Features/Support/Sources/Types/SupportChatDayBuilder.swift:17` and `ios/Packages/PrimitivesComponents/Sources/Types/DateSectionBuilder.swift:25` — two separate `Calendar.current.startOfDay` groupings on iOS; Android groups the same transcript and the same activity list its own way.
- **P64** **S** `ios/Features/Stake/.../StakeSceneViewModel.swift:121` — the unlock date is built by adding `service.lockTimeSeconds(chain:)` to now; Core has the seconds and could carry the date.
- **P65** **S** `ios/Features/Support/.../SupportChatSceneViewModel.swift:51` — the sync cursor is derived from the last agent message's timestamp in the model.
- **P66** **S** `ios/Packages/GemstonePrimitives/Sources/Extensions/Date+GemstonePrimitives.swift:9` — a `dateComponents` helper the app owns.
- **P67** **S** `ios/Packages/Store/Sources/Models/PriceRecord.swift:120` — a missing `updatedAt` becomes the epoch, which is a decision about staleness.
- **P68** **S** `android/data/services/gemstone/.../assets/RecentAssetsService.kt:39`, `.../stores/BalanceStore.kt:34`, `.../entities/DbTransaction.kt:68`, `.../entities/DbPerpetualPosition.kt:120` — four separate `System.currentTimeMillis()` stamps written while saving, where Core decides freshness elsewhere.
- **P69** **S** `android/data/services/store/.../TransactionsDao.kt:164,189` — `updatedAt` defaulted at the DAO.
- **P70** **S** `android/ui/.../list_item/transaction/TransactionItem.kt:185,217` — `createdAt` stamped inside a UI item.
- **P71** **S** `android/data/services/gemstone/.../stream/WebSocketConnection.kt:45` reads `pingIntervalMilliseconds()` from Core — confirm the iOS socket does the same rather than using its own interval.

## 22. URLs built in the app

- **V59** **M** `ios/Packages/GemstonePrimitives/Sources/Config.swift` (4 URLs) against `android/gemcore/.../AppUrl.kt` and `android/gemcore/.../ext/UpdateUrl.kt` — the app's own URLs are listed twice, once per platform.
- **V60** **S** `ios/Packages/PrimitivesComponents/.../DeepLinkViewModel.swift` (5 URLs) — deep link targets built in a view model while Core owns `Deeplink::to_gem_url`.
- **V61** **S** `ios/GemPriceWidget/Services/WidgetPriceService.swift:82`, `ios/Packages/Components/Sources/AssetImageView.swift`, `.../Grid/GridPosterView.swift`, `.../Lists/ListAssetItemView.swift`, `ios/Packages/GemstonePrimitives/.../GemImage+GemstonePrimitives.swift` — the asset image URL is assembled from `assets.gemwallet.com/blockchains/<chain>/assets/<tokenId>` in five places on iOS.
- **V62** **S** `ios/Features/NFT/.../CollectibleViewModel.swift`, `ios/Features/Transfer/.../TransferDataViewModel.swift`, `ios/Features/WalletConnector/.../ConnectionView.swift`, `ios/Features/Settings/.../PreferencesScene.swift`, `ios/Features/QRScanner/.../QRScannerScene.swift` — one hand-built URL each.
- **V63** **S** `android/ui/.../UriHandlerExt.kt` and `android/ui/.../Markdown.kt` — URL handling helpers with no Core counterpart.
- **V64** **S** `android/features/settings/networks/.../NodeItem.kt` and `android/features/update_app/.../InAppUpdateBanner.kt` — URLs built in composables.

## 23. Ownership: a view model holding more than its service

The rule is in [ARCHITECTURE.md](ARCHITECTURE.md) §7 and now covers stores as well as services. The store sweep is clean on both apps; these are the remaining multi-service holders.

- **O31** **S** `ios/Features/Contacts/.../ManageContactViewModel.swift` — `GemManageContactServiceProtocol` plus `GemNameServiceProtocol`, held only to pass to `ManageContactAddressViewModel`'s `AddressInputViewModel`. Decide whether a shared component's service is a port or a second service.
- **O32** **S** `ios/Features/Onboarding/.../ImportWalletViewModel.swift` — `GemWalletServiceProtocol` plus `GemNameServiceProtocol`, the same conduit shape as O31.
- **O33** **M** `ios/Gem/ViewModels/RootSceneViewModel.swift` — four Core services plus `ViewModelFactory`; the app root, and the one rule inside it (a required update offers only the update action) is unreachable from a test.
- **O34** **S** `android/features/bridge/.../WCRequestViewModel.kt` — `GemWalletConnectServiceInterface` plus `GemSignMessageServiceInterface`.
- **O35** **S** `ios/Gem/ViewModels/RootSceneViewModel.swift:41` — `currentWallet` reads `viewModelFactory.stores.walletStore.getWallet(id:)` with `try?` on every `body` pass. The session service has the async answer; making it sync would flash onboarding, so this needs a decision, not a rewrite.

## 24. Chain-specific branches in app code

- **N10** **S** `ios/Packages/Primitives/Sources/WalletId.swift:98` and `android/gemcore/.../wallet/cases/WalletIdGenerator.kt:22` — both apps pick the Ethereum account to seed a wallet id. The same rule, written twice, in the identity path.
- **N11** **S** `ios/Packages/PrimitivesComponents/.../BannerViewModel.swift:41` — a `case .bitcoin` branch decides banner behaviour.
- **N12** **S** `ios/Packages/PrimitivesComponents/Sources/Types/ChainImage.swift` — 14 chain cases; confirm against Android's chain icon map, which the mapper contract allows, and close if it matches.
- **N13** **S** `ios/Packages/PrimitivesComponents/.../SwapProviderType+Gemstone.swift:25` — a lone `case .hyperliquid` beside the provider icon map.

## 25. The About screen, decided twice

- **L15** **S** `ios/Features/Settings/.../AboutUsScene.swift:36` and `android/features/settings/aboutus/.../AboutUsScreen.kt:46` — the label-map fingerprint pairs these at 0.83 on `community`, `privacypolicy`, `termsofservice`, `version`, `website`. The rows of the About screen, their order and their links are chosen in each app.

## 26. Records that hand the app a bare number

Swept on 2026-09-16 over every `#[uniffi::Record]` in `core/gemstone/src` for `f64`/`i32`/`u32`/`i64`/`u64` fields whose name is a rendered quantity, keeping only the records an app actually names. The contract is [the row carries the finished value](ARCHITECTURE.md): a record that crosses with a raw `f64` makes both apps decide the precision, the sign and the unit. The count is how often the app source names the record.

- **F32** **S** `GemFormattedNumber.value` (`formatted_number.rs`, 19 app mentions) — the record that exists to carry finished text still exposes the raw value beside it; confirm every reader takes the text.
- **F33** **S** `GemRewardsState.invite_reward_points` (`rewards/model.rs`, 17) — iOS bolds it with `String(_:).boldMarkdown()` inside the invite description.
- **F34** **S** `GemAutocloseField.price` and `.original_price` (`perpetual/autoclose.rs`, 15) — both apps format these through their own perpetual formatter.
- **F35** **M** `ChainConfig.account_activation_fee`, `.token_activation_fee`, `.minimum_account_balance` (`chain.rs`, 12) — activation fees are rendered from raw numbers in both apps.
- **F36** **S** `GemBannerContext.asset_rank_score` (`banner/model.rs`, 6) — a score that decides whether a banner shows, exposed as a number rather than the decision.
- **F37** **S** `GemPriceUpdate.price`, `.price_usd`, `.price_change_percentage_24h` (`price/model.rs`, 4).
- **F38** **S** `GemPriceAlertSession.current_price` (`price_alert/session.rs`, 4) — the session already returns suggestions; the price beside them is formatted twice.
- **F39** **S** `GemPerpetualTransferData.price` and `.leverage` (`perpetual/model.rs`, 4).
- **F40** **S** `GemPerpetualChartLayout.price_low` / `.price_high` (`perpetual/model.rs`, 4) — the chart axis labels are built from these.
- **F41** **S** `GemDurationPart.value` (`duration_formatter.rs`, 4) — a duration part that carries a number and leaves the unit to the app.
- **F42** **S** `GemAssetDetailsState.price_alerts_count` (`assets/model.rs`, 4) — a count the app turns into a badge string.
- **F43** **S** `GemAssetDetailsInput.price` (`assets/model.rs`, 4).
- **F44** **S** `GemPerpetualAutoclose.take_profit` / `.stop_loss` (`perpetual/model.rs`, 3).
- **F45** **S** `GemFiatQuoteRequest.amount` (`fiat/session.rs`, 3).
- **F46** **S** `GemPerpetualDefaults.leverage`, `.take_profit_percent`, `.stop_loss_percent` (`perpetual/rules.rs`, 2) — defaults that both apps render as text.
- **F47** **S** `GemBalanceValue.amount` (`balance/model.rs`, 2).
- **F48** **S** `GemChart.base_value` (`chart/mod.rs`, 1) — the chart baseline.

## 27. Screens with no Core service

156 iOS view models name no `Gem*ServiceProtocol`; Android has 8. The asymmetry is the shape of the gap: Android injects a service into almost every model, iOS keeps a family of small models that decide presentation locally. A child model handed a Core record is allowed — these are the ones heavy enough to be deciding something. The weight in brackets is members plus methods.

- **B11** **M** `PrimitivesComponents/NetworkFeeSceneViewModel.swift` [29] — the heaviest model in the repo with no service, beside a Core fee-rate record Android reads through `FeeDetailsModel`.
- **B12** **M** `Features/Perpetuals/AutocloseSceneViewModel.swift` [28] — Android's `AutocloseViewModel` drives the same screen from `GemAutocloseSession`.
- **B13** **M** `PrimitivesComponents/AssetDataViewModel.swift` [26] — the shared asset model behind most rows; see also R55.
- **B14** **M** `Features/WalletConnector/ConnectionProposalViewModel.swift` [24] — Android's proposal scene holds `GemWalletConnectServiceInterface`.
- **B15** **M** `PrimitivesComponents/PerpetualDetailsViewModel.swift` [22] — Android holds `GemPerpetualDetailsServiceInterface` for this screen.
- **B16** **M** `Features/AppLock/LockSceneViewModel.swift` [22] — the lock screen, which is a security surface with no Core service on either app.
- **B17** **S** `Features/Support/SupportMessageBubbleViewModel.swift` [19] — bubble grouping and status beside `GemSupportService`.
- **B18** **M** `Features/Swap/SwapDetailsViewModel.swift` [18] — Android builds the same rows in `SwapDetailsUIModelFactory` from `GemSwapQuoteSummary`.
- **B19** **S** `Features/Perpetuals/PerpetualPositionViewModel.swift` [18].
- **B20** **S** `Features/Transactions/TransactionsFilterViewModel.swift` [16] — the activity filter; Android's `TransactionsViewModel` holds the Core filter list.
- **B21** **S** `Features/Perpetuals/CandlestickChartViewModel.swift` [16] — see D25.
- **B22** **S** `PrimitivesComponents/TransactionViewModel.swift` [15] — see R77.
- **B23** **S** `PrimitivesComponents/NetworkFeeCustomViewModel.swift` [15] — Android's namesake takes `FeeDetailsModel` and calls `customFee`; iOS does not.
- **B24** **S** `PrimitivesComponents/BannerViewModel.swift` [14] — `canClose` is declared on Android too, in `BannerItemUIModel.kt`.
- **B25** **S** `Features/Onboarding/VerifyPhraseViewModel.swift` [14] — phrase verification is a security rule with no Core owner.
- **B26** **S** `PrimitivesComponents/AddressListItemViewModel.swift` [13] — see R59.
- **B27** **S** `PrimitivesComponents/InputValidationViewModel.swift` [11] — validation on the iOS side of the `Validators` boundary.
- **B28** **S** `PrimitivesComponents/BalanceViewModel.swift` [11] — see R58.
- **B29** **S** `Features/Settings/ChainNodeViewModel.swift` [11] — node rows beside `GemChainSettingsService`.
- **B30** **S** `Features/QRScanner/QRScannerSceneViewModel.swift` [11] — scan handling with no Core payment service.
- **B31** **S** `Features/FiatConnect/FiatQuoteViewModel.swift` [11] — quote rows beside `GemFiatQuoteService`.
- **B32** **S** `Features/Assets/AssetsFilterViewModel.swift` [11] — `chainsFilter` is declared on Android too, in `TransactionsViewModel.kt`.
- **B33** **S** `PrimitivesComponents/AssetViewModel.swift` [10] — see R62.
- **B34** **S** `PrimitivesComponents/PriceViewModel.swift` [9] — see D23.
- **B35** **S** `PrimitivesComponents/EmptyContentTypeViewModel.swift` [9] — `actions` is declared on Android too, in `DelegationViewModel.kt`.
- **B36** **S** `PrimitivesComponents/TextInputSheet/TextInputViewModel.swift` [9].
- **B37** **S** `Features/Transfer/TransferDataViewModel.swift` [9].
- **B38** **S** `PrimitivesComponents/FiatTransactionViewModel.swift` [8] — the fiat transaction row, which SERVICES.md says already reads the same on both apps.
- **B39** **S** `PrimitivesComponents/CopyTypeViewModel.swift` [8] — see S30.
- **B40** **S** `PrimitivesComponents/Types/ChartHeaderViewModel.swift` [8] — see R65.
- **B41** **S** `PrimitivesComponents/Protocols/ValueHeaderViewModel.swift` [8].
- **B42** **S** `Features/Transfer/TransactionInputViewModel.swift` [8].
- **B43** **S** `Features/Swap/SwapTokenViewModel.swift` [8].
- **B44** **S** `Features/Swap/PriceImpactViewModel.swift` [8] — see R66.
- **B45** **S** `Features/Settings/RewardRedemptionOptionViewModel.swift` [8] — see R63.
- **B46** **S** `Features/PriceAlerts/PriceAlertItemViewModel.swift` [8].
- **B47** **S** `Features/Perpetuals/OpenPositionItemViewModel.swift` [8].
- **B48** **S** `PrimitivesComponents/WalletHeaderViewModel.swift` [7].
- **B49** **S** `PrimitivesComponents/ChainViewModel.swift` [7].
- **B50** **S** `GemPriceWidget/CoinPriceRowViewModel.swift` [7] — the widget row, which cannot import Gemstone today.
- **B51** **S** `Features/Support/SupportMessageInputBarViewModel.swift` [7].
- **B52** **S** `Features/Settings/ServiceStatusItemViewModel.swift` [7].
- **B53** **S** `Features/Perpetuals/PerpetualsHeaderViewModel.swift` [7].
- **B54** **S** `Features/Perpetuals/PerpetualViewModel.swift` [7] — `priceText` is declared on Android too, in `ChartHeaderUIModel.kt`.
- **B55** **S** `Features/Perpetuals/PerpetualPositionItemViewModel.swift` [7].
- **B56** **S** `Features/Perpetuals/AutocloseViewModel.swift` [7] — see R69 and D22.
- **B57** **S** `PrimitivesComponents/SimulationWarningViewModel.swift` [6] — simulation warnings landed in Core (d458faac6e); confirm this model only maps them.
- **B58** **S** `PrimitivesComponents/PnLViewModel.swift` [6] — position profit landed in Core (6321c9eb20); same check.
- **B59** **S** `PrimitivesComponents/ListAssetItemViewModel.swift` [6].
- **B60** **S** `PrimitivesComponents/ChartValuesViewModel.swift` [6].
- **B61** **S** `Features/WalletTab/PerpetualsPreviewViewModel.swift` [6].
- **B62** **S** `Features/Transactions/TransactionTypesFilterViewModel.swift` [6].
- **B63** **S** `Features/Swap/SwapProvidersViewModel.swift` [6] and `SwapButtonViewModel.swift` [6].
- **B64** **S** `Features/Perpetuals/PerpetualItemViewModel.swift` [6].
- **B65** **S** `Features/Assets/AssetHeaderViewModel.swift` [6].

## 28. Core duplicated inside Core

The same `pub fn` name in two crates is not always duplication, but these thirteen name a rule rather than a constructor. Each is a chain crate or a provider crate carrying a copy of something a sibling already has.

- **G8** **M** `calculate_transaction_fee` in `gem_cosmos`, `gem_solana` and `gem_ton` — three chain crates computing a fee the same way.
- **G9** **S** `calculate_fee_rates` in `gem_cosmos` and `gem_solana`.
- **G10** **S** `calculate_network_apy` in `gem_cosmos` and `gem_solana` — the staking APY formula in two chain crates.
- **G11** **S** `create_staking_client` in `gem_bsc` and `gem_monad`.
- **G12** **S** `format_price`, `format_input_price` and `format_size` in `gem_hypercore` and `core/gemstone/src` — perpetual formatting written on both sides of the FFI boundary.
- **G13** **S** `checksum_address` in `core/gemstone/src` and `swapper`.
- **G14** **S** `chain_from_id` and `chain_id` in `dexscreener` and `swapper`.
- **G15** **S** `create_eth_client` in `swapper` and `yielder`.
- **G16** **S** `deposit_addresses` in `gem_evm` and `swapper`.
- **G17** **S** `for_chain` in `gem_evm`, `settings_chain` and `swapper` — three per-chain lookups.
- **G18** **S** `config_session_properties` in `gem_wallet_connect` and `core/gemstone/src`.
- **G19** **S** `has_price` / `has_size` in `primitives` and `core/gemstone/src`.
- **G20** **S** `execution_error` in `gem_sui` and `primitives`.

## 29. Gaps the screen-service map already names

[SERVICES.md](SERVICES.md) says a screen service only one app holds is the next consolidation. These are the rows where the table itself shows one side empty or asymmetric.

- **P72** **M** `GemAppUpdateService` — iOS `AboutUsViewModel` holds it; Android uses Play in-app update instead, so the update decision is made by two different owners. `AppUpdateCoordinator` already maps `upgradeRequired` itself (see F29).
- **P73** **M** `GemAvatarService` — Android has no avatar surface at all, so wallet avatars are an iOS-only feature rather than a Core one.
- **P74** **S** `GemNotificationsService` — iOS `NotificationsViewModel` holds it; Android's `SettingsViewModel` uses push cases instead.
- **P75** **S** `GemTransactionDetailsService` — iOS holds the service, Android reaches the same answer through `GetTransactionDetailsImpl` as an observed read, so the links are built in two places.
- **P76** **S** `GemChainService` — iOS holds it in the chain picker, Android in two unrelated models (`ContactChainSelectViewModel`, `SelectImportTypeViewModel`); confirm the three screens ask the same question.
- **P77** **S** `GemRecentActivityService` — three iOS holders against two Android; the extra iOS holder is `SelectAssetViewModel`, which Android answers inside `BaseAssetSelectViewModel`.
- **P78** **S** `GemBannerService` — held by no screen on either app, composed inside two services; confirm the banner rules have not drifted between those two compositions.
- **P79** **S** `GemWalletSessionService` — iOS spreads it over `RootSceneViewModel` and `NavigationHandler`; Android keeps it in `SessionCoordinator`. The iOS split is what produced O35.
- **P80** **S** `GemStakeService` — three screens each side, but iOS `EarnSceneViewModel` filters delegations itself (S21) where Android does not.
- **P81** **S** About 67 exported records and enums are named by neither app. SERVICES.md says review before deleting; do the review and record the ones that are reached through a nested field so the list stops being re-swept.

## 30. App ports that Core could own

- **V65** **S** `ios/Packages/FeatureServices/.../Reconnectable.swift` — `reconnectDelayMilliseconds` and `pingIntervalMilliseconds` are a Swift protocol; Android reads both from Core (`WebSocketConnection.kt:45`). The iOS socket should read the same numbers.
- **V66** **S** `ios/Packages/.../ConnectionComponentMonitoring.swift` and `ConnectivityMonitor.swift` — two single-method protocols over connection health, beside `GemConnectionService`.
- **V67** **S** `ios/Packages/.../WebSocketRequestProvider.swift` — one method, building the authenticated socket request that Core's device auth already signs.
- **V68** **S** `ios/Packages/Store/Sources/BindableQuery.swift` — a one-method protocol behind every observed read on iOS; Android has narrow cases instead. Worth one decision about which shape both apps use.
- **V69** **M** `ios/Packages/Formatters` and `ios/Packages/Validators` cannot import Gemstone, which is what keeps D15–D17 duplicated. The item is the dependency, not the formatter: decide whether the widget and these two packages get a Gemstone-free Core surface or move under one that can import it.
- **V70** **S** `android/features/update_app/.../InAppUpdateServiceImpl.kt` — an app-owned update service beside `GemAppUpdateService`, with its own HTTP client and timeouts (S28).
- **V71** **S** `android/ui/.../UriHandlerExt.kt` — URL opening policy in the UI module (V63).
- **V72** **S** `ios/Packages/GemstonePrimitives/Sources/Config.swift` — the iOS half of V59; decide whether app URLs are a Core config record or stay per-platform.

## Closed with no change

Each of these was a section of the 2026-09-15 sweeps. The work was to check them; the answer was that the sweep measured the wrong thing. They are kept so the same sweep is not run again with the same conclusion.

### 2. Lists and screens without a row record

All 13 closed by 2026-09-16. Four landed a record — the recipient sections, the perpetual market sections, the contact address fields and the node check rows — and the rest were screen chrome over decisions Core already makes.

Nine were checked and need no record. The add-node, contact, add-asset, import-wallet and connection-proposal screens are the same case in a different shape: every decision they make — the phase of the node check, whether a chain takes a memo, which import kinds a chain offers, whether an address shows a view-only warning — already comes from Core, and what is left is a screen title, a field label and a button, fixed and unconditional. The connection proposal's two permission lines are the same list in the same order on both apps with nothing conditional behind them; naming them in Core would be the lookup wrapper the guidance bans. The amount screen's balance and reserved-fee lines are a localized template around an amount each platform formats with the device locale, and the `shows_asset_balance` and `can_change_value` decisions already come from `GemAmountInput` — moving the text into Core would take the locale out of the number. The autoclose summary was the one real find: Android hand-joined `"$label: $value"` where iOS and Android's own position row both call `trigger_order_text`, so a cleared trigger read differently; Android now calls it too. The scanner error is two platform conditions — no camera, permission denied — with the same two strings on both apps and nothing for Core to decide.

Copy: [`GemAssetRow`](../core/gemstone/src/services/assets/model.rs) → [iOS](../ios/Packages/PrimitivesComponents/Sources/ViewModels/ListAssetItemViewModel.swift), [Android](../android/gemcore/src/main/kotlin/com/gemwallet/android/domains/asset/aggregates/AssetInfoDataAggregate.kt). Each of these iOS view models composes four or more user-facing strings and holds no Core record; the count in brackets is how many. Land each with its Android mirror.

### 5. Screens that may want a session

Both closed on 2026-09-16. The chain settings screen got one: the nodes and their statuses were two app-side fields and each app guarded staleness differently, so `GemNodeListSession` now drops a status for a node the list no longer has and Android's refresh nonce is gone. The transactions filter did not need a session — it needed the filter set, which Core now builds; the sheet's own state is two selections and a presentation flag.

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

### Files that have outgrown one module

All nine closed by 2026-09-16, two of them by changing the file.

**X46–X52** counted file lines, and in a Rust module the tests live at the bottom of the file they test. The production halves are 622, 596, 556, 518, 489, 465 and 443 lines — the top of a smooth distribution across 44 `rules.rs` files whose median is about 130, not a cliff. Each one is also a single subject: `transactions/rules.rs` is 38 functions that all feed `row`, `detail_rows` and `details`, and `transfer/rules.rs` is one trait and its impl for `TransactionInputType`. Splitting either would move a private helper away from its only caller and add a module layer that removes nothing.

**X53** is real and fixed: `Keychain/Types/Status.swift` was 1240 lines because 820 of them were a hand-copied English table of Apple's own `OSStatus` messages. `SecCopyErrorMessageString` returns the same message from the system, localized, so `description` is now one line and the file is 434 — the enum of status codes and nothing else.

**X54** is real and fixed: `ViewModelFactory.swift` was one struct with 55 stored services, 98 imports and 61 screen constructors. The struct and its properties stay in `ViewModelFactory.swift`; the constructors moved to `ViewModelFactory+Wallet`, `+Settings`, `+Wallets`, `+Transfer`, `+Activity`, `+Perpetuals` and `+Collectibles`, each carrying only the imports its screens need. The largest is 217 lines.

### Core hardening, second pass

**X106** closed on 2026-09-16. `block_explorer/explorer.rs` is gone: `Explorer::new(&str)` was dead outside its own tests, and the service builds `Explorer { chain }` from a real `Chain`. The four `X::from_chain(chain).unwrap()` sites in `gateway/chain_factory.rs` and `signer/chain.rs` sit inside a matching `chain_type()` arm, and that invariant is now a test — `every_chain_type_resolves_to_its_sub_chain` walks every chain and asserts the Bitcoin, EVM and Cosmos conversions exist for the type the config claims — so a new chain added with the wrong type fails in CI instead of in the app. `device.rs` keeps both: a device key must not be built from a failed OS RNG or a seed the signer rejected, and there is no safe value to fall back to.

### Core `unwrap` and `expect`

**X45** closed on 2026-09-16. The 255 it counted was 255 of nothing in particular: the sweep excluded `#[cfg(test)] mod tests` and nothing else, so every `#[cfg(all(test, feature = "chain_integration_tests"))]` module, every `#[cfg(test)] impl`, every bare `#[test]` function and every `src/testkit/` directory counted — which is why it named three integration-test modules as the leaders. The real number was 150, and 100 of those are gone: the UTXO branch of `Transaction::finalize` and the Bitcoin transaction mapper no longer index into a node response that may not carry an address, the ThorChain and Uniswap quote builders, the HyperCore EIP-712 writers and the Sui stake and transfer builders return their parse errors, a swap provider whose chain has no endpoint is left out of the list instead of panicking the whole swapper, the daemon's metric locks and the wallet-connect seen-message lock survive a poisoned mutex, `get_block_explorer` falls back to the chain's first explorer when a stored name is gone, an asset or NFT link with an unknown type is skipped instead of crashing the import, and every `SystemTime::now().duration_since(UNIX_EPOCH)` takes the zero default.

The 50 that remain are two deliberate groups. Boot wiring — the daemon's `main`, the API's stream producer, settings, search index, migrations, the localizer fallback and the six reqwest builders — is meant to stop the process, and degrading instead is a product decision, not a cleanup. The rest are infallible by construction: `Chain::from_str(self.as_ref())` on the chain enums, a const base58, BOC, address or URL parsed once behind a `LazyLock`, an HMAC that accepts any key length, and every `X::from_chain(chain).unwrap()` inside a matching `chain_type()` arm. Making that last set provable needs a total conversion keyed on `ChainType` rather than a different call at each site, and that is a type change, not a panic to remove.

### The TON verified-collection allowlist

**X39** closed on 2026-09-16 with no change beyond deleting the note. Every NFT provider derives verification from whatever its upstream gives it — OpenSea and Alchemy read a safelist status, Alchemy also reads a spam flag — and TON's token metadata carries no such field, only a `valid` flag that already gates which token info is picked and the marketplace the collection was listed on. The allowlist is that provider's signal, not a placeholder for one. Widening it is data work on the collections the API already stores (`nft_collections.is_verified`), not a gap in this crate.

### The currency field's height pin

**X33** closed on 2026-09-16 with no change. Measured on iOS 26.5, the newest runtime: unpinned, the field is 53.0 pt while it is empty and 54.7 pt as soon as it holds a digit, so the amount jumps on the first keystroke and the pin is still doing its job. `CurrencyTextFieldTests` now measures the height across four amounts, so the check is already written for whoever revisits this once the SwiftUI fix lands.

### 12. Localization hygiene

Twenty English strings exist under two keys — `wallet_send` / `transfer_send_title`, `wallet_stake` / `transfer_stake_title`, `wallet_import_address_field` / `transfer_recipient_address_field` and seventeen more. Checked on 2026-09-15: each pair carries its own context comment in `localization/app/en.ftl` and names a different place in the product, so the pairs are deliberate and must not be merged — a language that needs a different form for a label and a screen title depends on them being separate.
What went wrong in V40 and V41 was not the pair; it was one app's mapper reaching for the other half of a pair. That is only visible by comparing the two apps, which `just check-mappers` now does on every variant both apps map. The fifteen keys nothing read are gone.
