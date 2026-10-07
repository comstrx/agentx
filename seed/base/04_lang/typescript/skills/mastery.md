# Mastery

Type-level engineering that pays rent at every call site.

- **Design APIs so inference does the typing:** builders and factories that thread generics through, so consumers write zero annotations and still get narrow types — the test of a good generic API is an untyped-looking call site with a fully-typed result.
- **Template literal types make strings safe:** route paths, permission keys (`view_${Resource}`), event names, CSS tokens — the compiler catches the typo that used to be a production 404.
- **Mapped + conditional types derive whole surfaces:** from one model map, derive the filter shape, the write shape, the response shape — the type-level mirror of "declare once, materialize everywhere". Keep the gymnastics in one types module; call sites see clean names.
- **Function overloads (or generic conditionals) encode real signatures:** a `get(key)` that returns `string | undefined` but `get(key, fallback)` that returns `string` — the API tells the truth per call shape.
- **Branded types for dangerous primitives:** `type TenantId = string & { __brand: 'tenant' }` stops an id from crossing into the wrong parameter — nominal safety where structural typing is too forgiving (ids, money, sanitized strings).
- **Exhaustiveness as a habit:** every union `switch` ends in a `never` assertion; adding a variant becomes a compile-error tour of exactly the places that must react.
- **Type-only imports** (`import type`) keep runtime bundles honest; erasable-syntax-only mindset — types are a compile-time tool, never a runtime dependency.
- **Assertion functions and type guards centralize narrowing:** `assertPresent(x)`, `isRecord(x)` — written once, reused everywhere, instead of scattered `as` casts.
- Read compiler errors bottom-up (the last "because" is the cause); when a type gets clever enough to need explanation, extract and name its parts — named intermediate types are the comments the codebase allows.
