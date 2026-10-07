# Types

- **Full types on every signature:** parameters, return, properties — no naked `mixed` where a real type exists, no untyped property ever. `strict_types=1` makes these contracts enforced at runtime; treat a coercion failure as the bug it caught.
- **Fixed-shape data is a type, not an array.** A function passing associative arrays with known keys is smuggling an object — model it as a `readonly` DTO with promoted constructor, an enum, or a value object so the type system verifies the shape. Associative arrays are only for genuinely open/dynamic maps.
- **Enums for every closed set:** statuses, kinds, scopes, actions — backed enums with methods where behaviour belongs to the set. A string constant pretending to be an enum invites the typo the enum makes impossible.
- **A docblock never substitutes a type.** Whatever the run's documentation policy, a tag standing in for a real signature type is banned (`/** @return string */` over `: string`); the analyser-demanded shape tags (`list<…>`, `array<K,V>`, `array{…}`, `@template`) are the one structural exception. The load-bearing test: remove the tag, run the gate — green means it was noise, leave it out.
- **Nullability is deliberate:** `?Type` means "absence is a valid business state" — not "I didn't decide". A method that cannot return null must not declare it; the caller's `??` chains reveal where nullability leaked.
- `readonly` by default for value-shaped classes; mutation is the exception with a named reason.
- **First-class callables and closures are typed** through `Closure` signatures the analyser can verify; `callable` strings are legacy.
- Casting is centralized: input coercion goes through one cast utility (string|int|bool|array normalizers), never sprinkled `(int)` guesses at call sites.
- The analyser at max level with zero suppression is the floor: no `@phpstan-ignore`, no baseline growth — fix the cause.
