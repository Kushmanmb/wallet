# Open work

**There are no open items.** Every id that has ever appeared here is retired; ids are never reused. Add a new one with the next free number in its series (V vocabulary, R rows, C composition, S sessions, B view boundary, F formatting, P parity, D decisions, O ownership, X platform, G guidance, PERF performance) and a size (**S**/**M**/**L**), and **delete its line in the commit that lands it**.

The goal is that Gemstone decides once and both clients read that decision. Track duplicated decisions and concrete performance work at their existing owners: shared rules and orchestration in Core; rendering, observation, scheduling, and localized formatting in the apps. Contracts are in [ARCHITECTURE.md](ARCHITECTURE.md) and [SERVICES.md](SERVICES.md).

Keep each item independently reviewable. Shared decision changes land in Core and both apps; platform-only work stays on that platform. Regenerate only when shared interfaces or integration change, and run the applicable [Quality Checks](../skills/quality-checks.md). Verify affected primary-screen journeys under [Performance](PERFORMANCE.md). Remove replaced paths within the item's scope; do not bundle an unrelated row migration or product change.

## Settled — do not re-open these

A sweep that finds one of these has found a decision already made, not a gap.

**Rows.** A screen's chrome — its sheet title and its cancel, clear and done buttons — is each app's own. A camera's permission and support states are the platform's, and the two apps' states do not even match, so the QR scanner's error text stays local. Network list, recents chips, earn APR, price list and onboarding link rows carry no choice. Swap provider rows and the QR scan-type hint table are iOS only. A row a screen only ever draws one way keeps its shape in the view; only a row drawn differently somewhere carries its layout in the record ([the rule](ARCHITECTURE.md#a-row-that-a-screen-only-ever-draws-one-way-keeps-its-shape-app-side)).

**Sessions.** Asset details, delegation, stake, earn, receive, NFT details, collections, contacts list, transaction details, wallets list and currency are read-only or single-selection. Perpetual market has no shared derived set. Support chat's only duplication is platform image encoding. Security and developer are platform preferences. Create-wallet and phrase verification are not symmetric.

**Numbers.** Counts a screen uses to build sections, decimals, bps, indices, timeouts and chart geometry stay numbers — the contract is about numbers the app renders as text. `Formatters` and `Validators` on iOS cannot import Gemstone, which is why a number crosses as a value plus a style rather than through a foreign trait.

**One-sided by design, not a parity gap.** `isVersionHigher` (Play update), `migrateToSharedPassword` (Android password store), `set_price_alerts_enabled` (Android one-off migration), `signWithKeystore` (iOS keystore), `isOriginRejected`, `authentication_chain_ids`, `authentication_accounts` and `authentication_methods` in the auth flow (Android-only one-click auth; proposal and sign check the origin inside Core on both), `scanTransaction` (both scan through `GemConfirmService`).

**Crossings.** Identifiers cross as their stored string and [stay that way](SERVICES.md#3-each-app-implements-a-thin-store-adapter). The confirm load's stage order is [fixed by policy, not only by data](ARCHITECTURE.md#a-staged-load-names-what-each-stage-waits-for). A balance refresh publishes [one atomic batch](SERVICES.md#6-publish-a-multi-source-refresh-as-one-batch). Swap quote timing is [measured, with the responsible provider named](SWAPPER.md#timing).

Do not "fix" the [deliberate divergences](SERVICES.md#deliberate-divergences--do-not-fix-these).
