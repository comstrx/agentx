# Types

- **`strict: true` always** — plus `noUncheckedIndexedAccess`; the compiler options are the contract, never loosened per file.
- **`any` is banned.** Unknown input is `unknown`, narrowed by real checks before use. `as` casts only at genuine boundaries (deserialization edges) with the reason visible; `!` non-null only when the guarantee is in the surrounding lines; `@ts-ignore`/`@ts-expect-error` never ships to silence a real error — fix the cause.
- **Parse, don't assert, at the boundary:** external data (API responses, env, files, user input) enters through a validator that produces the typed shape; interior code never re-checks what the boundary guaranteed. One schema is the single source — types derive from it, never maintained in parallel.
- **Discriminated unions are the state model:** `{ status: 'loading' } | { status: 'ready'; data: T } | { status: 'failed'; error: E }` — impossible states unrepresentable, `switch` exhaustive with a `never` check so a new variant breaks every place that should care.
- **Derive, never duplicate:** `keyof`, indexed access, `ReturnType`, `Parameters`, mapped types pull shapes from the source of truth; a hand-copied interface of an existing shape is drift scheduled.
- **`interface` for public object contracts, `type` for unions/compositions/functions;** `readonly` by default on value shapes; `as const` for literal vocabularies with the union derived from it.
- **Generics earn their letters:** a type parameter must appear in at least two positions (constrain input ↔ output) or it is noise; constraints (`extends`) over conditional gymnastics; let inference work — annotate boundaries, infer locals.
- **`satisfies` for config shapes:** validates against the contract while preserving the narrow literal types the call sites want.
- Enums as `as const` objects + derived unions (erasable, tree-shakeable) unless the project already standardized on `enum`.
- Return types explicit on the public surface (they are the API); inferred on internals (they are plumbing).
