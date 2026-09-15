# Performance and responsiveness

## Goal and scope

Daily wallet use should feel immediate on both iOS and Android: responsive taps and typing, smooth transitions, stable scrolling, and refreshes that preserve usable content. Prioritize wallet home, asset details, transaction lists and details, confirmation, and swap, including their shared rows, images, search, storage, subscriptions, and navigation.

This document defines engineering principles, initial performance targets, and a repeatable evaluation plan. The targets are proposed project budgets, not measured results or claims of existing automated coverage. Establish baselines on both platforms before enforcing numerical regression gates. Visible freezes, incorrect state, and unsafe transaction behavior are failures regardless of averages.

Follow [Architecture](ARCHITECTURE.md), [Services](SERVICES.md), [Swapper](SWAPPER.md), and [Security](../skills/security.md). Core owns domain decisions and orchestration; the apps own rendering, observation, navigation, and platform adapters. Improve the layer that causes the cost.

## What good feels like

| Primary surface | Expected experience | Exercise under load |
|---|---|---|
| Wallet home | Cached balances and assets appear promptly; refreshing keeps the list usable; changing wallets never flashes the previous wallet's data | Scroll during balance and price updates; pull to refresh repeatedly; switch wallets during refresh; return from background |
| Asset details | Header and available balance appear without waiting for chart, history, or images; updates preserve layout and scroll position | Change chart ranges rapidly; scroll history while prices update; enter and leave before loading finishes |
| Transactions | First page appears promptly; pagination and status changes preserve row identity and position | Long history, rapid filters, new transactions arriving while scrolled, pending-to-final transitions, repeated detail navigation |
| Confirmation | Transaction summary appears promptly; fees and simulation show explicit loading or errors; navigation and controls remain responsive | Slow simulation, fee changes during preload, insufficient balance, authentication cancellation, repeated send taps, delayed broadcast |
| Swap | Typing and pair selection remain responsive while quotes load; only results for the current input can be selected | Rapid amount edits, Max, pair reversal, slippage changes, provider failure, cold/warm routes, expired quotes, swap-to-confirm and back |

“Instant refresh” means immediate feedback and timely application of available results. Network and chain completion times remain visible and measurable; cached data must never be presented as newly verified data. Slow optional content must not delay the screen's useful content or block navigation.

## Initial budgets

Measure each scenario separately on each device. A percentile describes a distribution: p95 means 95% of samples finish within that value. Report median, p95, sample count, and worst observed latency; report p99 only with enough samples to support it.

| Measurement | Initial target | Measurement boundary |
|---|---|---|
| Input feedback | p95 ≤ 100 ms | Delivered tap, keystroke, or refresh gesture → first frame acknowledging it; do not include human authentication time |
| Warm screen content | p95 ≤ 200 ms | Navigation accepted → first frame with usable locally available content; retain normal transition duration |
| Local refresh application | p95 ≤ 100 ms | Result available at the app boundary → first frame displaying the corresponding state; trace decoding and storage inside this interval when they follow receipt |
| Frames during scrolling and transitions | ≥ 99% meet the active display deadline; zero reproducible visible hitches | Measure frames during active gestures and animations, including simultaneous updates; approximately 16.7 ms at 60 Hz and 8.3 ms at 120 Hz are whole-frame intervals, not main-thread CPU allowances |
| Main-thread work | No synchronous I/O or unbounded computation; investigate any app-owned continuous task or wait ≥ 50 ms | Include FFI, callbacks, lock waits, mapping, layout, and rendering; 50 ms is a diagnostic trigger, not an acceptable frame budget |
| Repeated navigation | No sustained growth in retained screen objects, tasks, observers, or requests after settling | Repeat the same journey 20 times; bounded cache warmup may increase memory, retained screens must not accumulate |
| Network-dependent readiness | Establish p50/p95 per scenario and controlled network profile | Action → fresh content, valid quote, or confirmation ready; separately report provider wait, local work, and time to initial feedback |

The aspiration is zero user-visible jank. The numerical frame target does not excuse a reproducible glitch. Use each platform's frame deadline and hitch metrics; raw iOS hitch rates and Android frame overruns are different measurements and must not be averaged together.

Cold launch, first wallet load, warm navigation, offline use, and empty-cache loading need separate baselines. Do not hide a cold-path regression inside warm-path results or promise an arbitrary network completion deadline that the app cannot control.

## Engineering principles

### 1. Keep the UI thread available

- Render already prepared state. Do not fetch, query storage, decode JSON or images, sort large collections, build transaction data, or perform expensive formatting inside SwiftUI `body`, Compose composition, row getters, or layout callbacks.
- An `async` function or a task launched from a view model does not prove work runs off the UI thread. Inspect the actual executor, dispatcher, and native callback path, including synchronous work before suspension and after resumption.
- Keep synchronous point-read contracts where the architecture requires them; schedule potentially blocking service calls through the platform's existing background execution boundary. A synchronous API is not permission to read storage on the main thread.
- Keep blocking I/O and heavy CPU work off both the UI thread and executor threads serving unrelated async work. Use the owning layer's supported scheduling facilities; do not add a new runtime or detached task per row.

### 2. Minimize work across FFI

Foreign function interface (FFI) calls cross between Swift/Kotlin and Rust. Measure the complete crossing: argument conversion, copies, serialization, Rust work, callbacks into app stores, result conversion, and state publication.

- Ask for one coherent screen or section record per relevant state change. Batch row projections and related reads through the existing owner where useful; avoid one crossing per property per render.
- Derive a record once from its inputs and reuse it until those inputs change. Include locale, currency, display preferences, wallet, and asset context in invalidation where relevant. Keep localized formatting in the apps as required by the architecture.
- Decode boundary JSON once and reuse typed values. Avoid repeated Swift/Kotlin → JSON → Rust → JSON round trips or copying a full history for a single changed row.
- Measure calls, bytes or item counts, allocations, and total duration per interaction. Fewer calls are useful only if they reduce total cost; one enormous payload can be worse than a bounded page.
- Preserve independent loading boundaries. Batching must not make cached content wait for optional remote content. Keep domain rules in Core even when optimizing their delivery.

### 3. Run independent work concurrently

Draw the dependency graph before changing scheduling. Start independent work together in the existing orchestrator, cap concurrency according to provider and device capacity, and publish coherent results as their required inputs become available. The critical path is the chain of dependencies that determines when a result can be shown.

| Flow | Work that can overlap when independent | Ordering that must remain |
|---|---|---|
| Wallet | Balance refresh and asset discovery; existing home refresh already joins these operations | Writes and observer delivery must retain the correct wallet context |
| Asset | Chart, transaction history, and independent metadata work | A derived balance or action must use the required inputs for the selected asset |
| Transactions | Independent metadata enrichment and status work where the storage contract permits it | Persist a complete sync before advancing its cursor; maintain pagination and deduplication rules |
| Confirmation | Initial state, fee preload, and independent supplied-simulation enrichment; `GemConfirmation.load` already overlaps these | Simulation that depends on preload waits for it; signing uses the validated confirmation inputs after required checks and authentication |
| Swap | Eligible provider requests and independent discovery probes, within the existing swapper | Route preload precedes live quoting under the current contract; the selected current quote feeds confirmation and transaction construction |

For two independent requests taking 150 ms and 200 ms, the I/O portion should approach 200 ms plus overhead rather than 350 ms. Verify overlap in a trace or with controlled completion barriers; concurrent syntax alone does not prove concurrent execution.

- Fetch a shared input once in the owner and pass it to dependents. Use existing exact-request coalescing where applicable; do not introduce global locks or singleton caches to conceal duplicate callers.
- Do not hold locks across network calls or slow callbacks unless the owning invariant explicitly requires it. Trace lock and executor queue time when concurrency fails to improve latency.
- Bound request fan-out, retries, image prefetch, and background CPU use. Unbounded parallelism can starve rendering, exhaust connections, increase battery use, and worsen tail latency.
- Give optional failures their existing local error state. A failed chart must not erase a usable balance; a failed required simulation must not become a successful confirmation.

### 4. Refresh incrementally and own task lifetimes

- Preserve usable content while refreshing. Apply only changed records and observe the smallest relevant state; unchanged writes and whole-screen invalidations create avoidable work.
- Preserve stable row identifiers, scroll anchors, image dimensions, keyboard focus, and selection. Do not reset the list to a loading screen, regenerate row identities, or animate the entire list on every price tick.
- Collapse redundant refresh triggers through the existing owner. Preserve the current subscription policy: wallet home uses socket prices and pull refresh; do not add a polling timer as a performance fix.
- Cancel obsolete screen work and detach observers when their owner ends. If the underlying request cannot be cancelled, reject its stale result and measure the remaining resource cost.
- Bind every result to its request context. Wallet, chain, asset, amount, fee selection, and swap inputs must not change underneath a result that later overwrites current state.
- Keep required transaction tracking alive beyond screen dismissal through its existing service lifetime. Broadcast completion and recording the pending transaction unblock the send flow; chain finality is tracked in the background.

### 5. Keep lists, charts, and images cheap

- Use lazy lists and bounded pages; prepare only the needed rows and a bounded prefetch window. Avoid rescanning or copying all assets or transactions on every observed update.
- Reuse formatters and prepared display values according to their real dependencies. Avoid repeated sorting, grouping, big-number conversion, and object allocation during scrolling.
- Decode and downsample images away from the main thread, use appropriately sized cached images, and reserve stable placeholder space. Bound caches and cancel offscreen image work where supported.
- Limit chart point preparation to what the visible range needs. Trace chart layout, drawing, effects, and image work alongside CPU and state updates; smoothness can be limited by rendering even when networking is fast.
- Scope SwiftUI observation and Compose state reads so one changed price does not rebuild unrelated rows. Verify update frequency and cost; a low recomposition count alone does not establish smoothness.

### 6. Make swap fast without reusing stale quotes

- Update the amount field immediately. Debounce expensive quote requests using the existing flow policy, not the visible text update; measure both last edit → request start and last edit → usable quote.
- Key quote results to all effective inputs: wallet/account, source and destination assets, amount, Max mode, slippage, and applicable provider settings. Cancel or discard earlier generations, including responses that arrive after pair reversal or navigation.
- Reuse the existing process-owned `GemSwapper` and route cache. [Swapper](SWAPPER.md) permits route hints and exact concurrent RPC coalescing; quote results, prices, balances, approvals, and transaction data are not cached by that mechanism.
- Test cache hits, process restarts, failed probes, and invalid cached hints. A route hint still requires a live amount quote; preloading remains best-effort and failures must retain the normal discovery fallback.
- Measure the slowest provider's contribution to the quote critical path. Preserve Core's selection and completion policy when optimizing; do not silently pick the first provider, skip price-impact checks, or retain an expired selectable quote to improve a timing number.

### 7. Preserve transaction integrity

Performance changes must retain explicit amounts, recipients, chain IDs, fees, approval spenders, simulation warnings, authentication, and signing checks. Bind confirmation and execution to the same validated input context; input changes must invalidate dependent work. Prevent duplicate sends during repeated taps, and do not retry an ambiguous broadcast blindly. Distinguish submitted/pending from final success. Test these boundaries alongside latency; neither app may trade correctness for a faster transition.

## Run a performance evaluation

### Fixtures and conditions

Use deterministic synthetic wallets and recorded or injected public responses, never real funds or user secrets. Extend existing testkit and injected client boundaries; unit tests must not start ad hoc HTTP servers. Maintain equivalent data and response ordering on both platforms.

| Dimension | Minimum coverage |
|---|---|
| Data size | Empty wallet; typical fixture with 20 assets and 100 transactions; stress fixture with 500 assets and 10,000 stored transactions, loaded in bounded pages |
| Network | Controlled fast responses; 300 ms and 2 s response delays; offline; one required failure; one optional failure; out-of-order and duplicate responses |
| State | Cold process and empty cache; cold process with populated database; warm screen; route-cache miss/hit; wallet switch; background/foreground |
| Devices | Per platform: an older supported physical device and a representative current device; exercise 60 Hz and high refresh rates where available |
| Presentation | Default and large text, long localized labels, missing images, active price/status updates during scrolling |

Use a separate live-provider run to observe real network variability. It supplements deterministic comparisons and cannot establish that a local code change caused a timing difference.

### Capture the right evidence

- **iOS:** Profile a release-optimized build on a physical device with Instruments. Use Time Profiler plus Hangs/Hitches and the SwiftUI instrument to locate expensive or repeated updates. Add narrow signposted intervals when needed to relate screen actions to service work. See Apple's [responsiveness guidance](https://developer.apple.com/documentation/xcode/improving-app-responsiveness) and [SwiftUI performance analysis](https://developer.apple.com/documentation/swiftui/performance-analysis).
- **Android:** Use a profileable, non-debuggable build matching release optimization and a physical device. Use Macrobenchmark for repeated journeys, `FrameTimingMetric` for frame behavior, and system traces for thread, scheduling, and Compose work. Record compilation/profile configuration. See [Macrobenchmark setup](https://developer.android.com/topic/performance/benchmarking/macrobenchmark-overview) and [metrics](https://developer.android.com/topic/performance/benchmarking/macrobenchmark-metrics).
- **Shared/Core:** Trace service entry, FFI conversion, store callbacks, RPC start/end, locks, and result delivery. Use focused benchmarks for an identified hot mapper, parser, row projection, or batch operation; a Rust microbenchmark alone does not prove mobile responsiveness.
- **Journey correctness:** Reuse [Maestro](../skills/testing-maestro.md) for shared navigation and state assertions and native tests for platform mechanics. Simulator/emulator runs and screenshots help reproduce glitches; they are not physical-device timing evidence. Keep authentication enabled when measuring confirmation; a debug bypass is only a separate functional test configuration.

Suggested interval names are `wallet.refresh`, `asset.load`, `transactions.page`, `confirm.preload`, `confirm.execute`, `swap.preload_routes`, `swap.quote`, and `ui.apply_state`. These are instrumentation proposals, not existing events. Correlate nested work with synthetic operation IDs and monotonic timestamps. Record durations, counts, input sizes, cache outcomes, cancellations, and typed error categories; never include seed phrases, keys, signing payloads, authentication tokens, or identifying wallet data.

Separate queued time, CPU work, FFI conversion, storage, network wait, and frame presentation. Overlapping intervals must not be summed as if they ran sequentially. Measure the frame that presents the result, not only the view-model assignment.

### Repeatable procedure

1. Pick one primary journey and its correctness assertions. Record baseline and candidate commits, device, OS, build configuration, fixture revision, network profile, refresh rate, and cache state. Verify the installed binary matches the commit.
2. Record a baseline trace and screen recording for the scenario. Keep temperature, power mode, background load, and test-driver pacing comparable. Record thermal throttling instead of silently discarding slow runs.
3. For warm scenarios, run 5 unmeasured warmups, then at least 30 measured repetitions per build and device. Reset deliberately for each cold/cache-miss repetition. Alternate baseline and candidate batches to reduce environmental drift. Use longer runs for stable tail estimates.
4. Scroll continuously for at least 30 seconds while injecting balance, price, history, or quote updates. Run the scenario's rapid edits, cancellation, and failure variants. Repeat navigation 20 times and inspect settled memory, observers, and pending work.
5. Identify the dominant cost and its owner. Change one cause, then repeat the same measurements. For concurrency or stale-response fixes, add a deterministic test with controlled completion order; avoid flaky wall-clock unit assertions.
6. Compare per-device distributions, frame deadlines, worst stalls, request/FFI counts, and memory behavior. Reproduce any visible glitch even when the summary metrics improve. Keep uninstrumented timing runs separate from verbose diagnostic traces when instrumentation changes performance.
7. Run the applicable [Quality Checks](../skills/quality-checks.md) for the implementation. A shared interface change also requires generation and app verification. Report every skipped scenario or platform; passing builds are not performance evidence.

Existing build and test commands live in [iOS commands](../ios/skills/development-commands.md) and [Android commands](../android/skills/development-commands.md). This document does not introduce a benchmark runner or CI job. Until a scenario has a committed harness, capture it manually with the platform tools and record the exact invocation/configuration so another engineer can repeat it.

## Regression policy and report

For changes to a primary screen or its shared data path, evaluate the affected journey. Shared Core, FFI, storage, and subscription changes need performance evidence from both apps when they affect these journeys. Before release, run the complete primary-screen matrix on the reference devices. These are execution guidelines; no recurring job is created by this document.

Treat a new freeze, reproducible scroll jump, stale-wallet result, stale selectable quote, duplicate send, growing task leak, or skipped security gate as a failure. After baselines are stable, flag a p95 latency regression greater than both 10% and 10 ms, or a missed-frame fraction increase of at least 0.5 percentage points, for repeat measurement and trace review. These are initial investigation thresholds; do not automatically excuse a regression below them or replace baselines to hide one.

Store a concise report with the change:

```text
Journey / fixture / network / cache state:
Baseline commit → candidate commit:
Platform / device / OS / build / refresh rate:
Exact commands or profiler configuration / sample count:
Metric                       Baseline       Candidate       Target
Input feedback p95:
Usable content p95:
Fresh result or ready p95:
Frame deadline misses / worst stall:
FFI calls and time / RPC count / peak concurrency:
Settled memory / remaining tasks and observers:
Trace and recording locations:
Cause and owning layer / change made:
Correctness, cancellation, failure, and security checks:
Skipped or blocked scenarios / platform gaps:
Result and remaining work:
```

## Implementation entry points

Use the [screen-service map](SERVICES.md#screen-services) for current iOS and Android consumers. Start with these owners and follow the actual callers, store adapters, and rendering path before optimizing:

- Wallet: [`GemWalletHomeService`](../core/gemstone/src/services/wallet_home/mod.rs).
- Asset details: [`GemAssetDetailsService`](../core/gemstone/src/services/assets/details.rs).
- Transactions: [`GemTransactionsService`](../core/gemstone/src/services/transactions/mod.rs) and [transaction details](../core/gemstone/src/services/transactions/details.rs).
- Confirmation: [`GemConfirmation`](../core/gemstone/src/services/confirm/confirmation.rs) and [transfer orchestration](../core/gemstone/src/services/confirm/transfer.rs).
- Swap: [screen-facing quote service](../core/gemstone/src/services/swap/quote.rs) and [swapper code map](SWAPPER.md#code-map).
