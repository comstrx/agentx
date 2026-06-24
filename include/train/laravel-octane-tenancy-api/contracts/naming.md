# contracts/naming.md — Naming (LAW)

Names are **clear, concise nouns** — never verbose, self-describing file names. A reader grasps a file's role
from a short name, not a sentence. Re-read every turn.

## Classes & files
- **PSR-4 (`App\ → app/`):** file name = class name, one class per file (`CategoryService.php` →
  `class CategoryService`).
- **Layer suffixes:** `XxxService`, `XxxRepository`, `XxxController`, `XxxRequest`, `XxxResource`. Models are
  the bare noun (`Product`, `Order`, `Wallet`).
- **Base shells:** `BaseRepository`, `BaseService`, `BaseRequest`, `BaseResource`, `BaseCommand`; the base
  controller is `Controller` (Laravel convention, `abstract`). Each only `use`s its engine trait.
- **Engine traits:** `HasBaseXxx` (`HasBaseModel`, `HasBaseRepository`, `HasBaseService`, `HasBaseController`,
  `HasBaseRequest`, `HasBaseResource`, `HasBaseCommand`) in `app/Traits/Bases/`.
- **DNA traits:** `HasXxx` (`HasRoles`, `HasPermissions`, `HasFiles`, `HasSearch`, `HasCache`, `HasRelations`,
  `HasState`, `HasTenant`, …) in `app/Traits/Dna/`. Names illustrative — add what a system needs in this
  style; never invent a parallel convention.

## The `index.php` folder convention (Support & Traits)
First level under `app/Support/` and `app/Traits/` is **folders only**, never loose files. Each folder's
`index.php` is the public facade: `support/str/index.php` → `namespace App\Support; class Str` → called as
`App\Support\Str::x()`. Internal pieces are PascalCase sibling files: `support/str/Casing.php` →
`namespace App\Support\Str; class Casing`. These load via **classmap** (not PSR-4) with clean class names —
**run `composer dump-autoload` after adding a new support/trait file** (classmap is static; the class will not
resolve until you do). See `tools.md`.

## Reserved-keyword avoidance (mandatory)
No class name may be a reserved keyword: **`Boolean`** (not `Bool`), **`Casing`** (not `Case`), **`Matches`**
(not `Match`), **`cache/Tag`** (not `Index`, which collides with `index.php`). Apply the same care to any new
piece.

## Facade shadowing
Support facades intentionally shadow Illuminate equivalents: `Str`, `Arr`, `Cache`, `Date`, `Log`, `Mail`,
`Queue`, `Storage`, `Http`, `Request`, `Response`, `Context`. Prefer ours. When a file genuinely needs both,
**import both and alias the Illuminate one with `as`** (`use Illuminate\Support\Str as IlluminateStr;`) — never
fully-qualify it inline (`style.md`: no FQCN in code).

## Routes & resources
- Resource route segment = **plural snake_case** of the resource (`products`, `commission_rules`), following
  global REST naming — clear and predictable.
- Route names mirror the segment (`products.index`, `products.show`), prefixed per panel
  (`admin.products.index`). Panel prefix + name per file: `/v1/<panel>/…`, names `<panel>.<resource>.<action>`.
- Reusable route blocks live in `routes/apis/shared.php` as clearly-named functions (`resource()`,
  `engagements()`, `account()`, … — your choice). Each registers a sub-tree with **string** handlers only
  (`route:cache`-safe, `arch.md` §8).
- Spelling of any actor/role is **fixed and consistent** across route prefix, role, and permissions — pick one
  spelling and never vary it.

## Permissions — derived from the resource name
Pattern **`<verb>_<resource>`** with the four CRUD verbs, plus cross-cutting flags. The engine derives the
four from the resource segment; declare only the cross-cutting and special ones.

| Kind | Examples |
|------|----------|
| CRUD (derived from `<resource>`) | `view_products`, `add_products`, `edit_products`, `delete_products` |
| Cross-cutting (engine-wide flags) | `allow_statistics`, `allow_downloads`, `allow_comments`, … |
| Permission management | `view_permissions`, `edit_permissions` |

Permissions are **derived where possible** (`design.md` §3), not hand-maintained per endpoint. `has:<permission>`
middleware enforces them; authorization is server-authoritative (`tolerance.md`).

## Database
- Tables: plural snake_case (`order_items`, `commission_rules`). Pivots: alphabetical singular pair
  (`product_tag`) unless a domain name is clearer.
- Columns: snake_case. Foreign keys `<singular>_id` (`vendor_id`, `wallet_id`). **`tenant_id`** on every
  tenant-owned table. Composite uniques `unique(tenant_id, …)`; hot indexes lead with `tenant_id` (`arch.md` §9).

## General
Methods/properties: clear verbs/nouns, no Hungarian, no redundant prefixes. Booleans read as predicates
(`isBlank`, `hasColumn`, `active`). Do not encode the whole purpose into a name; let the short name + its
layer/folder carry the meaning.
