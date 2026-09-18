# Development Commands

Use the iOS `justfile` for every build and test. All xcodebuild recipes share one set of build settings (configuration, simulator, Gemstone linker flags, compilation cache), so moving between `build-package`, `build`, and `test` reuses the modules the previous command compiled. A raw `xcodebuild` call with different settings recompiles most of the app on the next `just` command; when you need a variation, start from the command `just -n <recipe>` prints.

## Build and Test

```bash
just install                           # first-time setup
just clean                             # clean DerivedData and build artifacts
just check GemstonePrimitives          # compile one UIKit-free package with SwiftPM
just check-test Primitives             # run one UIKit- and Gemstone-free package's tests with SwiftPM
just build-package Assets              # build one package or feature scheme
just build                             # build the app
just test AssetsTests                  # build what one test target needs and run it
just test                              # run the whole unit test plan
just build-for-testing                 # build every test target once
just test-without-building AssetsTests # re-run tests without building; omit the target for the whole plan
just test-ui                           # run the UI test plan on a reset simulator
```

The test recipes boot the simulator first. A targeted run executes serially on that simulator; the whole plan runs in parallel clones.

## Pick the cheapest command that can fail

Measured on a warm tree:

| command | warm | use it for |
|---|---|---|
| `just check <Package>` | 1-2s | does this package still compile — the default while editing a platform-independent package |
| `just check-test <Package>` | 2s | that package's tests, same limits as above |
| `just build-package <Package>` | 3-5s | a UI package or feature compiles |
| `just test <TestTarget>` | 8-11s | one test target, including Gemstone-dependent ones |
| `just build` | 8s | the app links, before a commit |
| `just test` | ~55s | the whole suite, before a commit |

SwiftPM builds for the host, so the two `check` recipes only cover packages that import neither UIKit nor SwiftUI, and among those only ones that do not link Gemstone can run their tests — the static library is built for the simulator. Everything with a UI or a Gemstone dependency goes through `just build-package`, `just build`, and `just test <TestTarget>`.

`swift build` reports every error in the package; `xcodebuild` stops at the first failing target, so a compile-fix loop driven by `just build` costs one build per error batch. `just test` builds what it needs, so a `just build` before it only adds a second build.

## Core Changes

The app links Core through `libgemstone.a` and the generated `Gemstone.swift`/`GemstoneFFI.h`, so run `just generate-stone` after any Core change before building or testing iOS. It builds the simulator library, generates the bindings from that library, and rewrites a binding file only when its content changed, so an unchanged FFI surface causes no Swift recompile. It takes about 3s when Core is unchanged and about 20s after a `gemstone` edit.

`just generate-stone release` (or `BUILD_MODE=release`) builds the simulator and device libraries in release. `GEMSTONE_IOS_TARGETS` overrides the target list, for example `GEMSTONE_IOS_TARGETS=aarch64-apple-ios` for a debug build on a device.

## Generation and Localization

```bash
just generate               # run all generation steps
just generate-models        # regenerate model types from Rust
just generate-stone         # build the iOS Core library and its UniFFI sources
just localize               # regenerate .xcstrings catalogs and typed Localized accessors
```

From the repo root, use `just generate-stone` and `just run-ios` as the default Gemstone/iOS flow. The optional `GemStone` Xcode scheme runs the same generation before the normal app build, for the device or simulator it targets.

## SwiftUI Iteration

For presentation-only SwiftUI work, build the owning package and avoid full app builds for each visual adjustment:

```bash
just build-package Assets
just build-package Components
just build-package PrimitivesComponents
```

For ViewModel or display-model behavior, pair the package build with the narrowest matching test target:

```bash
just test AssetsTests
just test LockManagerTests
```

Use `just build` when the change touches app composition, navigation wiring, generated bindings, or code that cannot be validated by a package build.

## Additional Utilities

```bash
just spm-resolve-all
```

## Command Rules

- Use `just` commands for builds and tests, not `xcrun swift test` or ad hoc `xcodebuild` invocations
- Build logs live under `build/DerivedData`
- `just install` installs `swiftformat`, `swiftlint`, and `xcbeautify`
