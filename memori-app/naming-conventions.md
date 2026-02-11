# SvelteKit Naming Conventions

Practical, consistent naming rules for this project.

## 1) Project / Repo / Package

- Repo name: `kebab-case`
  - Examples: `acme-portal`, `billing-dashboard`, `docs-site`
- NPM package name (`package.json > name`): `kebab-case` or scoped `@scope/kebab-case`
  - Examples: `acme-portal`, `@acme/billing-dashboard`
- App title (UI/branding): normal words
  - Example: `Acme Portal`

## 2) Folders and Paths

- Folders: `kebab-case` (or single word where possible)
  - Examples: `src/lib`, `src/routes`, `src/lib/components`, `src/lib/server`, `src/lib/utils`
- Keep purpose folders consistent:
  - `src/lib/components` -> shared UI components
  - `src/lib/features/<feature>` -> feature modules
  - `src/lib/server` -> server-only code (DB, auth, services)
  - `src/lib/utils` -> pure helpers
  - `src/lib/stores` -> Svelte stores
  - `src/lib/types` -> shared types

## 3) SvelteKit Routes

- Route folders: `kebab-case`
  - Example: `src/routes/account-settings/`
- Route groups (do not affect URL): `(group-name)`
  - Example: `src/routes/(app)/dashboard/`
- Params:
  - `[id]` (single) -> `src/routes/users/[id]/+page.svelte`
  - `[...slug]` (rest) -> `src/routes/docs/[...slug]/+page.svelte`

### Route Files

Use SvelteKit special filenames exactly. Common files:

- `+page.svelte`, `+page.ts`, `+page.server.ts`
- `+layout.svelte`, `+layout.ts`, `+layout.server.ts`
- `+error.svelte`
- `+server.ts`

Note:
- `.js` variants are also valid when not using TypeScript.

### Endpoint Naming

- Prefer resource nouns in paths (project convention):
  - Good: `/api/users`, `/api/users/[id]`
  - Avoid: `/api/getUsers`, `/api/deleteUser`
- `/api/*` is recommended for organization, not required by SvelteKit.
- Put logic in `+server.ts` and export the HTTP handlers you need:
  - `GET`, `POST`, `PUT`, `PATCH`, `DELETE`, `OPTIONS`, `HEAD`

## 4) Components

- Component filenames: `PascalCase.svelte`
  - Examples: `UserCard.svelte`, `ConfirmDialog.svelte`, `NavBar.svelte`
- Co-locate component-specific files when helpful:
  - `UserCard.svelte`
  - `UserCard.test.ts`
  - `UserCard.stories.ts`
- One component per file unless it is a tiny internal helper.

## 5) Modules, Utilities, and TypeScript Files

- Non-component TS files: `kebab-case.ts` (chosen convention)
  - Examples: `date-format.ts`, `auth-guards.ts`, `http-client.ts`
- Svelte runes modules should follow this too:
  - Examples: `session-state.svelte.ts`, `dashboard-model.svelte.ts`

## 6) Variables, Functions, Classes, and Types

- Variables/functions: `camelCase`
  - `getUserById()`, `isLoading`, `handleSubmit()`
- Classes/types/interfaces: `PascalCase`
  - `HttpClient`, `UserProfile`, `ApiError`
- True constants: `UPPER_SNAKE_CASE`
  - `DEFAULT_PAGE_SIZE`, `API_TIMEOUT_MS`

## 7) Stores

- Store files: `kebab-case.ts`
  - Example: `src/lib/stores/session-store.ts`
- Exported store names: `camelCase`
  - `sessionStore`
- Derived stores should read like values:
  - `isAuthenticated`

## 8) CSS / Styling

- Global styles: simple filenames like `app.css`, `variables.css`, `themes.css`
- If writing custom class names, use `kebab-case`
  - `.user-card`, `.nav-item`, `.is-active`
- State class prefixes are encouraged:
  - `.is-loading`, `.is-open`, `.has-error`

Note:
- Utility-class systems (Tailwind/UnoCSS) can reduce the need for custom class naming rules.

## 9) Suggested Baseline Structure

```text
src/
  lib/
    components/
      UserCard.svelte
      ConfirmDialog.svelte
    features/
      billing/
        components/
        services/
    server/
      db.ts
      auth.ts
    stores/
      session-store.ts
    utils/
      date-format.ts
    types/
      user.ts
  routes/
    (app)/
      dashboard/
        +page.svelte
        +page.ts
      account-settings/
        +page.svelte
    api/
      users/
        +server.ts
      users/[id]/
        +server.ts
  app.css
```

## 10) Consistency Rule

If a file or folder does not match this convention, prefer renaming while touching nearby code, and avoid introducing mixed styles in new files.
