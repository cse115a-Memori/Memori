# Changelog

## 2026-02-19

### Monorepo tooling

- Added `just ext` in `justfile` to optionally route `CARGO_TARGET_DIR` to `/Volumes/MemoriTarget/memori-tauri-target` when an external drive is mounted.

### `memori-app` Rust backend refactor

- Split `src-tauri/src/lib.rs` into focused modules for long-term maintainability:
  - `src-tauri/src/commands/connection.rs`
  - `src-tauri/src/commands/data.rs`
  - `src-tauri/src/commands/mod.rs`
  - `src-tauri/src/state.rs`
  - `src-tauri/src/simulator.rs`
- Kept command registration centralized in `src-tauri/src/lib.rs` and preserved specta export wiring for shared frontend types.
- Updated `src-tauri/src/oauth.rs` `UserInfo` to derive `Type` and use:
  - `#[serde(rename_all = "camelCase")]`
  - `#[specta(rename_all = "camelCase")]`

### `memori-ui` type naming alignment for FE

- Updated layout/widget serialization + TS export shape consistency:
  - `memori-ui/src/layout.rs`
    - `#[serde(rename_all_fields = "camelCase")]` on `MemoriLayout` (fields become camelCase).
    - Per-variant `#[specta(rename_all = "camelCase")]` so variant names remain PascalCase while fields export camelCase.
  - `memori-ui/src/widgets/mod.rs`
    - Added both serde + specta camelCase rename attrs to `MemoriWidget`.

### Frontend tauri bindings + IPC normalization

- Updated generated `memori-app/src/lib/tauri/bindings.ts` for camelCase field names:
  - `UserInfo.accessToken`
  - `MemoriWidget.remoteUpdateFrequency` / `localUpdateFrequency`
  - `MemoriLayout` nested fields (`topLeft`, `rightTop`, etc.)
- Added `memori-app/src/lib/tauri/ipc.ts` with:
  - `AppError` alias (`string`)
  - `toCmdError(...)`
  - `tryCmd(...) -> ResultAsync<T, AppError>`
- Exported IPC helpers via `memori-app/src/lib/tauri/index.ts`.

### Auth service modernization

- Reworked `memori-app/src/lib/services/auth.ts` to use `RuneStore` (`@tauri-store/svelte`) instead of raw plugin-store file primitives.
- Tightened store lifecycle:
  - `autoStart: false`
  - `saveOnChange: true`
  - concurrency-safe `startPromise` guard
  - retry-safe start behavior (promise reset on failed start)
  - explicit `hooks.error` rethrow
- Switched auth types to specta-exported `UserInfo`.
- Replaced legacy raw `invoke` login path with typed `commands.loginWithProvider(...)` wrapped in `tryCmd(...)`.

### Route extraction and page cleanup

- Kept root page minimal:
  - `memori-app/src/routes/+page.svelte` now acts as a simple navigation hub.
- Moved feature flows out of root page:
  - login flow to `memori-app/src/routes/login/+page.svelte`
  - device flow to `memori-app/src/routes/device/+page.svelte`
- Updated `memori-app/src/routes/+layout.svelte` nav links to include `Login` and `Device`.

### Command handling cleanup in Svelte routes

- Standardized frontend command calls on `tryCmd(...)` and typed `commands.*` usage.
- Added shared command handling helper in `memori-app/src/routes/device/+page.svelte` to reduce duplicated success/error logic.
- `memori-app/src/routes/login/+page.svelte` now uses explicit state for hydration + action status (`pendingAction`, `statusMessage`, `errorMessage`).

### Widget/layout client mapping updates

- Updated `memori-app/src/lib/widget-utils.ts` slot naming to camelCase keys (`topLeft`, `bottomRight`, etc.) to match exported Rust/specta shapes.

### iOS generated project artifacts

- Updated generated Apple project files:
  - `memori-app/src-tauri/gen/apple/memori-app.xcodeproj/project.pbxproj`
  - `memori-app/src-tauri/gen/apple/memori-app_iOS/Info.plist`
  - `memori-app/src-tauri/gen/apple/memori-app_iOS/memori-app_iOS.entitlements`
