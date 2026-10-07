# Style

The owner's hand — deliberately **not PSR-12**, so there is **no formatter** in the gate. Existing project files are the reference; match them exactly, every line.

- `declare(strict_types=1);` at the top of **every** file, never removed — it is a runtime fail-fast switch, not decoration.
- **4-space indent, K&R braces** (opening brace on the same line).
- **Declarations and control structures breathe:** a space before `(` and spaces inside — `public function index ( Request $req ): JsonResponse {`, `if ( $cond )`, `match ( $x )`, `foreach ( $items as $item )`. **Native calls do not:** `array_merge($a, $b)`, `is_array($value)`.
- **Breathing bodies:** one blank line after a method's opening `{` and one before its `}`. **No blank line between consecutive methods** — a closing `}` is immediately followed by the next signature.
- Property/const declarations grouped at the **top** of the class, one blank line before the method block; the class itself breathes like a method.
- **Never a one-line body** — always the multi-line breathing form, even for a single statement. The one exception: a constructor that **only promotes properties** collapses to `) {}` — short form on one line, long form one param per line then `) {}`; the moment it gains a statement it reverts to the breathing form.
- One-line guard clauses welcome: `if ( !$id ) return Response::fail();`.
- Heavy use of `match`, arrow fns `fn () =>`, ternaries, `??`/`?->`, destructuring, named args where they clarify.
- Multiple same-modifier properties may share a line; align `=>` in multi-line arrays.

```php
class Cast {

    public static function string ( mixed $value ): ?string {

        if ( is_array($value) || is_object($value) ) return null;

        $value = trim((string) $value);

        return in_array($value, ['', 'null', 'undefined'], true) ? null : $value;

    }

}
```

- `namespace` then `use` lines immediately; blank line before the class. No fully-qualified class names inside code — import and use the short name; alias on collision.