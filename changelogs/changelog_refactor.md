# Changelog

## 2026-02-20

### `memori-app` location route platform guard

- Updated `memori/memori-app/src/routes/location/+page.svelte` to gate geolocation behavior to mobile platforms only.
- Added a derived status value that reports `not-available` when the app is not running on mobile.
- Skips permission checks and `watchPosition` startup on non-mobile platforms.
- Added explicit UI feedback message: `Location status is not available on this platform.`
- Kept watch cleanup via the `onMount` teardown return (`stopWatching`) for active mobile sessions.

### `memori-app` location state/service cleanup

- Extracted location permission + position sync into `memori/memori-app/src/lib/services/location-service.ts`.
- Consolidated ownership so `memori/memori-app/src/routes/device/+page.svelte` requests/refreshes location and updates shared `appState`.
- Added initialization behavior that uses `lastKnownLocation` as fallback, then updates it when a fresh position is available.
- Simplified `device/+page.svelte` handlers by removing extra wrappers and small helper indirections for clearer action-by-action flow.
- Restored location test UI in `memori/memori-app/src/routes/location/+page.svelte`:
  - status display
  - lat/long display
  - explicit `Enable Location` button (hidden when status is `not-available`)
- Removed the redirect button to `/device` so the location route remains a standalone testing page.

### `memori-app` route simplification and permission UX polish

- Updated `memori/memori-app/src/routes/location/+page.svelte` to check permission state before showing the enable action.
- Button visibility is now gated to prompt states only (`prompt` / `prompt-with-rationale`), and hidden for `granted`, `denied`, and `not-available`.
- Restored denied-state guidance text directing users to Settings > Privacy > Location.
- Added explicit fallback messaging when rendering last-known coordinates without live permission.
- Simplified route state logic by inlining one-off values and restoring derived wrappers only where reused multiple times.
- Updated home navigation in `memori/memori-app/src/routes/+page.svelte` from `/test` to `/widgets`.

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
  - `memori/memori-app/src/routes/+page.svelte` now acts as a simple navigation hub.
- Moved feature flows out of root page:
  - login flow to `memori/memori-app/src/routes/login/+page.svelte`
  - device flow to `memori/memori-app/src/routes/device/+page.svelte`
- Updated `memori/memori-app/src/routes/+layout.svelte` nav links to include `Login` and `Device`.

### Command handling cleanup in Svelte routes

- Standardized frontend command calls on `tryCmd(...)` and typed `commands.*` usage.
- Added shared command handling helper in `memori/memori-app/src/routes/device/+page.svelte` to reduce duplicated success/error logic.
- `memori/memori-app/src/routes/login/+page.svelte` now uses explicit state for hydration + action status (`pendingAction`, `statusMessage`, `errorMessage`).

### Widget/layout client mapping updates

- Updated `memori-app/src/lib/widget-utils.ts` slot naming to camelCase keys (`topLeft`, `bottomRight`, etc.) to match exported Rust/specta shapes.

### iOS generated project artifacts

- Updated generated Apple project files:
  - `memori-app/src-tauri/gen/apple/memori-app.xcodeproj/project.pbxproj`
  - `memori-app/src-tauri/gen/apple/memori-app_iOS/Info.plist`
  - `memori-app/src-tauri/gen/apple/memori-app_iOS/memori-app_iOS.entitlements`
