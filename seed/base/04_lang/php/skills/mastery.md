# Mastery

Modern PHP written like a language, not a legacy.

- **Pipelines over loops:** collections chain `filter → map → groupBy → sortBy → values` in one readable flow; a hand `foreach` accumulating into a temp array is a pipeline that hasn't been written yet. Reach for `array_*`/collection methods before writing state.
- **`match` is the decision engine:** expression-shaped, exhaustive, no fallthrough — every closed decision becomes a `match` returning a value; `switch` is dead vocabulary.
- **Constructor promotion + `readonly` + named args** make value objects one-screen affairs: promote, type, freeze; construct with named args when the call site gains clarity.
- **Generators for volume:** `yield` streams big datasets with flat memory — chunked table walks, file line readers, cursor iterations; a function returning a million-element array is a generator that hasn't been written yet.
- **First-class callable syntax** (`$this->method(...)`, `Str::slug(...)`) turns methods into pipeline citizens without closure noise.
- **Late static binding is the engine's fuel:** `static::` inside base traits resolves to the concrete class — the mechanism that lets one engine trait serve every child with zero configuration.
- **`__call`/`__get` as deliberate dispatch**, never accident: dynamic surfaces (nested relation dispatch, derived accessors) are backed by a discovered, cached map that fails loudly on unknown names — magic with a printed inventory.
- **Null-safety as grammar:** `?->` chains through optional relations, `??` supplies the default, `??=` initializes lazily — three tokens that erase a screen of `isset` archaeology.
- **Attributes over convention-guessing** where the ecosystem supports them: declarative metadata read by reflection once, cached, never re-scanned per request.
- **Property hooks and asymmetric visibility retire the accessor boilerplate** (current-major PHP): `public string $slug { get => Str::slug($this->name); }` replaces getter methods; `private(set) string $status` exposes reading while guarding writes to the class — value objects shrink to their declarations; verify the floor in `composer.json` before reaching for either.
- Performance instincts: opcache assumptions (no `eval`, stable classes), preloading-friendly bootstrapping, reflection results cached in statics safely, `str_*` over regex where a literal suffices.
