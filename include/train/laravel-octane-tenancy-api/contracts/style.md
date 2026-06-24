# contracts/style.md — Code style (LAW)

The code must be **indistinguishable from the owner's hand** — a reader must not be able to tell whether the
owner or an agent wrote it. There is **NO formatter** (no Pint) because this hand-style is intentionally not
PSR-12 compatible; consistency with the owner's hand matters more. The existing `app/Support/*` classes are
the **reference hand** — read and mirror them. Match it exactly, every line. Re-read every turn.

## Hard rules
- `declare(strict_types=1);` at the top of **every** PHP file. Full param & return types on every signature;
  type every property.
- **4-space indent. K&R braces** (opening brace on the same line).
- **Declarations and control structures: a space before `(` AND spaces inside it.**
  `public function index ( Request $req ): JsonResponse {`, `if ( $cond )`, `match ( $x )`,
  `foreach ( $a as $b )`, `catch ( \Throwable $e )`.
- **Native function *calls* use no inner spaces:** `array_merge($a, $b)`, `is_array($value)`. Declarations
  breathe; calls do not — match the surrounding code.
- **Breathing bodies:** one blank line right after a method's opening `{` and one right before its `}`.
- **NO blank line BETWEEN consecutive methods** — a method's closing `}` is immediately followed by the next
  method's signature on the very next line. The breathing is *inside* bodies, never *between* methods.
- **Declarations vs methods:** property/const declarations are grouped at the **top** of the class, separated
  from the method block by **ONE blank line** (a blank line may also separate distinct declaration groups). The
  class also breathes: blank line after the opening `{` and before the closing `}`.
- **NEVER a one-line function/method body** — always the multi-line breathing form, even for a single
  statement. No `function x () { return $y; }`. **The ONE exception is an empty constructor that exists only to
  promote properties** (see "Promoted constructors" below): it uses the collapsed `) {}`, never an empty
  breathing body.
- `namespace` then `use` lines immediately (no blank line between); blank line before the class.
- Multiple properties on one line may share a modifier (`protected array $scopes = [], $permissions = [];`).
  Align `=>` in multi-line array literals.
- Heavy use of `match`, arrow fns `fn() =>`, ternaries, `??`, destructuring `[$a, $b] = …`, `compact()`, and
  inline guard clauses (`if ( !$id ) return …;`).
- **Ids are UUIDv7 `string`, never `int`** — never copy `int $id` signatures.

## Canonical examples (this is the exact hand)
A Support class:
```php
<?php

declare(strict_types=1);

namespace App\Support;

class Cast {

    public static function string ( mixed $value ): ?string {

        if ( is_array($value) || is_object($value) ) return null;

        $value = trim((string) $value);

        return in_array($value, ['', 'null', 'undefined'], true) ? null : $value;

    }

}
```
Declarations on top → one blank line → methods with breathing bodies and **no** blank line between them:
```php
class HasBaseController {

    protected array $scopes = [], $permissions = [];

    public function index ( Request $req ): JsonResponse {

        return $this->service->index($req->all(), $this->scopes, $this->permissions);

    }
    public function show ( Request $req, string $id ): JsonResponse {

        return $this->service->show($id, $this->scopes, $this->permissions);

    }

}
```
A FormRequest (validation is mandatory on every write — `tolerance.md`):
```php
<?php

declare(strict_types=1);

namespace App\Http\Requests;

class CategoryRequest extends BaseRequest {

    /** @return array<string, mixed> */
    public function rules (): array {

        return [
            'name'        => ['required', 'string', 'max:120'],
            'category_id' => ['nullable', 'uuid'],
        ];

    }

}
```

## Promoted constructors
A constructor that **only promotes properties** (zero statements in the body) is written with a collapsed empty
body `) {}` — NEVER an empty breathing body `) {\n\n}`. This is the canonical shape for value objects, DTOs
(`Comments…` below: fixed-shape data is a DTO, not an array), and thin base-shell wiring.
- **Short — one line:**
```php
public function __construct ( public readonly string $email, public readonly ?string $name = null ) {}
```
- **Long — one promoted param per line, then `) {}`:**
```php
public function __construct (
    public readonly string $email,
    public readonly ?string $name = null,
) {}
```
A thin base shell follows the same rule (`public function __construct ( protected Model $model ) {}`). The
**moment the constructor has a body** (even one statement, e.g. `parent::__construct($model);`) it reverts to
the normal multi-line breathing form — the collapsed `) {}` is *only* for the genuinely empty body.

## Comments, PHPDoc, strict_types & imports
- **Zero comments — absolute.** No prose, no section banners, no `// TODO`, no `/* */`, no `#`. Never explain
  code with words; rename and restructure until it explains itself. Write a comment ONLY when the owner
  **explicitly** asks for one in this task.
- **Zero PHPDoc by default — a docblock is a gate concession, never documentation.** It is permitted ONLY when a
  static-analysis config exists in the project (`phpstan.neon`/`psalm.xml`) AND that config does **not** suppress
  the missing-type identifiers that would otherwise fail without the tag. Decision procedure, every file:
  1. **No analyzer config, OR it ignores both `missingType.iterableValue` and `missingType.generics`** (or sets
     the equivalent `checkMissingIterableValueType: false` / generics-off) → **write NO docblock at all.** The
     native type hints stand alone.
  2. **Analyzer present and does NOT ignore them** → write ONLY the minimal tag the gate fails without
     (`list<…>`, `array<K, V>`, `array{…}`, `class-string<T>`, `@template T`). Never a tag the native type
     already says (`/** @return string */` is always banned).
- **Load-bearing test (only under case 2):** add the tag, remove it, run the gate. Green without it → it was
  noise, leave it out. Red (`return.type`/`argument.type`/`missingType.*`) → it was a real contract, keep it.
  Never add a tag "for clarity".
- **Fixed-shape data is a type, not an array.** Do not return associative/fixed-key arrays as pseudo-DTOs to
  dodge a shape doc — model it as a `readonly` DTO object, an enum, or a value object so the native type system
  verifies the shape. Associative arrays are only for genuinely open/dynamic maps.
- **`declare(strict_types=1);` is mandatory and NEVER removed** — it is a runtime safety switch (fail-fast on
  type coercion at call/return boundaries, the same fail-closed spirit as the rest of the system), not
  decoration. A PHP file without it is a defect, regardless of analyzer settings.
- **No fully-qualified class names inside code.** Never write `\Illuminate\…\CursorPaginator` in a signature,
  property, return type, or body. `use` every class at the **top** of the file and reference the short name; on
  a name collision alias one with `use … as …` (`use Illuminate\Support\Str as IlluminateStr;`). The only inline
  `\` allowed is a leading global-namespace builtin call where importing is pointless (`\count`, `\is_array`) —
  and those still follow native-call spacing.

## Discipline
Match the surrounding file's exact spacing and idiom before adding to it — **read neighbours first.** No
drive-by reformatting of code you are not changing. The smallest correct diff wins (`tolerance.md`).
