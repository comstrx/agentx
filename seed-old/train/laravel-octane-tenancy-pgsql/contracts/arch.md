# contracts/arch.md — Architecture (LAW)

Where things live and how a request flows. The structural law. Pair with `design.md` (how to think). Both
are re-read every turn. Code here is the **exact hand** — copy its spacing (`style.md`).

## 1 — Layers
Repository pattern, layered. Two views of one stack:
- **Build / dependency order (inner → outer):**
  `support → traits → bases → repository → service → controller → request → middleware → route`
- **Runtime request flow (outer → inner):**
  `route → middleware → request (validation) → controller → service → repository → model (+ traits) → support`

Each layer depends only on layers **inner** to it. A controller never touches a model directly — it goes
through its service → repository. Support has **zero** business logic and depends on nothing in `app` except
sibling Support. Traits sit on Support; bases sit on traits.

## 2 — Code structure is FLAT (not modular domains)
All classes live one level deep in their layer folder: `app/Models/*`, `app/Repositories/*Repository.php`,
`app/Services/*Service.php`, `app/Http/Controllers/*Controller.php`, `app/Http/Requests/*`,
`app/Http/Resources/*`. A single unified `XxxController` serves all panels; behaviour differs by the active
panel role read from `Context` (§6).

## 3 — The Base engine (CORE PATTERN — this IS the magic)
The real logic of each layer lives in a `HasBaseXxx` **trait** under `app/Traits/Bases/`. Each layer has a
thin `BaseXxx` **shell class** that does nothing but `use` its trait. Concrete classes **extend** the shell
and stay almost empty — declaring only what is unique (`fields()`, an override). **Put new shared behaviour
in the trait, NEVER in a concrete class.** Engine traits, one per layer: `HasBaseModel`, `HasBaseRepository`,
`HasBaseService`, `HasBaseController`, `HasBaseRequest`, `HasBaseResource`, `HasBaseCommand`.

Canonical skeletons (ids are **UUIDv7 `string`**, never `int`):

```php
// app/Repositories/BaseRepository.php — thin shell
namespace App\Repositories;
use App\Traits\Bases\HasBaseRepository;
use Illuminate\Database\Eloquent\Model;

class BaseRepository {

    use HasBaseRepository;

    public function __construct ( protected Model $model ) {}

}
```
```php
// app/Repositories/CategoryRepository.php — concrete = declaration only (fields() + overrides)
namespace App\Repositories;
use App\Models\Category;

class CategoryRepository extends BaseRepository {

    public function __construct ( Category $model ) {

        parent::__construct($model);

    }
    public function fields ( array $data = [] ): array {

        return [
            'name'        => $data['name'] ?? null,
            'category_id' => $data['category_id'] ?? null,
        ];

    }

}
```
```php
// app/Models/Category.php — bare noun; DECLARES relation methods + casts + fillable; the engine derives the rest
namespace App\Models;
use App\Traits\Bases\HasBaseModel;
use App\Traits\Dna\HasTenant;
use App\Traits\Dna\HasRelations;
use Illuminate\Database\Eloquent\Model;

class Category extends Model {

    use HasBaseModel, HasTenant, HasRelations;

    protected $fillable = ['name', 'category_id', 'description'];

}
```

`BaseService`, the base `Controller` (`abstract`), `BaseRequest`, `BaseResource`, `BaseCommand` follow the
same shape: a thin shell that `use`s its `HasBaseXxx` trait and wires the inner layer in the constructor. A
concrete class that grows real logic is a smell — push it down into the engine.

## 4 — `app/Traits/` — exactly two sub-folders, nothing loose at root
- `app/Traits/Bases/` — the `HasBaseXxx` engine traits (§3): the reusable logic of every layer.
- `app/Traits/Dna/` — opt-in model **DNA**: a capability a model gains by `use`-ing the trait (`HasTenant`,
  `HasRoles`, `HasPermissions`, `HasFiles`, `HasSearch`, `HasCache`, `HasRelations`, `HasState` — illustrative;
  add what a system needs in the owner's style, `naming.md`). A large, cohesive DNA subsystem gets its OWN
  sub-folder of focused traits under `Dna/` — `Dna/Permissions/` (the multi-level RBAC ladder,
  `skills/rbac-permissions.md`) and `Dna/Social/` (engagement traits: `HasLikes`/`HasComments`/`HasReviews`/…,
  `skills/social-engagements.md`) — composed by a facade trait (`HasPermissions`, `HasSocial`).

Every trait in both folders is built **on top of the Support DSL** — it calls `App\Support\…` and never
re-implements native/infra work. Traits carry layer/model **behaviour**; Support carries the std-lib **power**.

## 5 — `app/Support/` layout & canonical domain map
First level = **folders only**, never loose files. Each folder's `index.php` is the public facade
(`App\Support\<Name>`); internal PascalCase siblings (`namespace App\Support\<Name>`) hold the pieces. Even a
one-file helper becomes `folder/index.php`. Support is **native/infrastructure ONLY — ZERO business logic**.
`†` = swappable adapter (a `Driver` interface + concrete driver + manager in `index.php`; swap the backend =
ONE new Driver file — `design.md` §6). This map is the **contract for naming/shape**, NOT a build list —
files are created on demand.

```
app/Support/
├── arr/         Arr        Dot Shape Filter Map Group Sort Tree
├── cache/   †   Cache      Driver RedisDriver Key Tag Entry Scope     full DSL + indexed (tag) invalidation
├── cast/        Cast       Scalar Collection Enum
├── context/     Context    Tenant Panel User Scope Meta               Octane-safe role/tenant/super tag
├── database/    Database   Uuid Transaction Rls Query Schema Column Keyset
├── date/        Date       Clock Range Format Parse
├── event/   †   Event      Driver RedisDriver Payload Outbox          publish(event,payload,key); Redis default
├── file/        File       Path Name Mime Size Hash Stream
├── http/        Http       Client Request Response Header Retry        outbound; SSRF guard via net/Ip
├── json/        Json       Encode Decode Path Shape Merge
├── lock/    †   Lock       Driver RedisDriver Mutex                    distributed lock (serves idempotency)
├── log/         Log        Context Channel Entry Redact                Redact = never log secrets
├── mail/        Mail       Mailer Message Address                      always queued, Mailgun API transport
├── net/         Net        Ip Url Domain Host Port                     Domain = tenant subdomain resolution
├── num/         Num        Money Percent Range Format Random           Money = integer minor-units math only
├── parse/       Parse      Csv Query Boolean Number Locale
├── queue/   †   Queue      Driver Dispatch Payload Tenant Retry        Tenant = stamp/restore ctx across jobs
├── request/     Request    Input Header Fingerprint Idempotency Tenant
├── response/    Response   Envelope Failure Pagination Meta            uniform success/fail envelope (§7)
├── security/    Security   Token Hash Signature Secret Sanitize        wrappers only — no DIY crypto
├── storage/ †   Storage    Driver S3Driver ObjectKey Upload TemporaryUrl
├── str/         Str        Casing Slug Clean Matches Random Template
├── throttle/ †  Throttle   Driver RedisDriver Limit                    per-plan rate limiting
└── validate/    Validate   Rule Shape Field Type Message               predicates + Laravel Rule objects
```

Rules: no class name may be a reserved keyword (`Boolean` not `Bool`, `Casing` not `Case`, `Matches` not
`Match`, `cache/Tag` not `Index`). Support facades intentionally shadow Illuminate (`Str`, `Arr`, `Cache`,
`Date`, `Log`, `Mail`, `Queue`, `Storage`, `Http`, `Request`, `Response`, `Context`) — **never alias-import
both ours and Illuminate's in one file**. `cache·lock·throttle·queue·event·storage` are the `†` adapters.
`context` is the single source of truth for the active role/tenant/super tag; `database/Rls` and
`queue/Tenant` read from it.

## 6 — Role / tenant "tag" via `Context` (Octane-safe — non-negotiable)
The active panel role + `tenant_id` + super flag live in request-scoped Laravel `Context`, wrapped by
`App\Support\Context`. The panel middleware **sets** the tag; the base controller/service **reads** it to
compute role-specific scopes/permissions. Roles are many-to-many; the panel route group declares the expected
role and the middleware verifies membership. `App\Support\Context` is the **only** accessor:

```
Context::tenantId(): ?string     // null for super (cross-tenant)
Context::role(): string
Context::panel(): string         // super|admin|vendor|affiliate|delivery|client|guest
Context::isSuper(): bool
Context::userId(): ?string
Context::set(panel, role, tenantId, userId): void   // middleware only
Context::forget(): void          // reset on Octane RequestTerminated
```

**NEVER** use long-lived singletons, static properties, request-bound global helpers, or container bindings
for per-request state (Octane bleeds them across requests). Reset tenant-scoped state on `RequestTerminated`.

## 7 — Response envelope (one envelope, never a second)
All API output flows through `BaseResource` / `App\Support\Response` into a **uniform** envelope:
- `success → { status: true, data, …extra }`
- `fail → { status: false, message, errors }`

Surface: `Response::success($data, $status, $extra)` · `::message($text)` · `::fail($errors, $status, $msg)` ·
`::error($key, $msg, $status)` · `::noContent()`.

## 8 — Routes (explicit reusable blocks, `route:cache`-safe)
Panels: `routes/apis/<panel>.php`, included by `routes/api.php`, prefix `/v1`, per-panel name + middleware.
Repeated shapes are **reusable named functions in `routes/apis/shared.php`**, invoked explicitly per panel —
NOT `glob()`/reflection auto-registration, NOT route closures (closures break `route:cache`). Every handler
is a `'method'` string on a `->controller(...)` group. Guard the block definitions with `function_exists` so
`route:cache` rebuilds cleanly. Provide several blocks (`resource()`, `engagements()`, `account()`, … — clear
names, implementer's choice).

```php
// routes/apis/shared.php — route:cache-safe (string handlers only)
use Illuminate\Support\Facades\Route;

if ( !function_exists('resource') ) {

    function resource ( string $name, string $controller ): void {

        Route::prefix($name)->name("$name.")->controller($controller)->group(function () use ( $name ) {

            Route::middleware("has:view_$name")->get('', 'index')->name('index');
            Route::middleware("has:add_$name")->post('', 'store')->name('store');

            Route::prefix('{id}')->whereUuid('id')->middleware("has:view_$name")->group(function () {

                Route::get('', 'show')->name('show');
                Route::get('{relation}', 'related')->name('related');
                Route::put('{column?}', 'update')->name('update')->middleware("has:edit_$name");

            });

        });

    }

}
```

The standard uniform action set every `resource()` exposes (all gated): `index` · `statistics`
(`has:allow_statistics`) · `store` (`has:add_*`) · `deleteMany`/`delete` (`has:delete_*`) · `show` ·
`related`/`showRelated` (nested, derived) · `update` (`has:edit_*`) · file actions (`has:edit_*`).
**Nested relations are real controller actions** that resolve `{relation}` against the model's derived
relations and fail-closed (404 on unknown) — never a route closure.

**API contract = the `routes/collections/` mirror, NOT OpenAPI.** The engine's dynamic surface doesn't fit doc
generators, so `routes/collections/<panel>.json` mirrors `routes/apis/<panel>.php` **1:1**: every route carries
its method, URL, required headers (`Authorization: Bearer …`, `Accept`, `Locale`, and `Idempotency-Key` on
writes), request body, and a saved response example. **Any API change updates its collection entry in the SAME
change** so the frontend sees it immediately. The mirror is mandatory, not optional.

## 9 — Database conventions
- **UUIDv7** primary keys everywhere (`support/database/Uuid`); ids are `string` in every signature.
- **`tenant_id`** on every tenant-owned table; composite uniques `unique(tenant_id, …)`; hot indexes lead with
  `tenant_id`. **Every business unique includes `tenant_id`** — notably `users` is `unique(email, tenant_id)`,
  NEVER `email` alone (same email registers per tenant). Migrations are **central, reversible, additive-first**
  — no destructive drop in the same release as code that still reads the column.
- The full canonical schema — roles hierarchy (`is_super`/`is_supervisor`), `products` single-table
  polymorphism (`type`/`subtype`/`product_id`), `prices`+`product_prices`, `locations` closure, `domains`/
  `zones` — is the law in **`data.md`**.
- **RLS** is defense-in-depth: transaction-local `set_config('app.tenant_id', ?, true)` (never session `SET`),
  the app connects as a **non-owner** role, tables `FORCE ROW LEVEL SECURITY`. The Eloquent global scope
  (`HasTenant`) is the **primary** isolation; RLS catches what code forgets.
- Money: **double-entry ledger, integer minor units** (never floats); idempotency keys on financial endpoints.

## 10 — Multi-tenant & Octane rules (correctness, not gold-plating)
`HasTenant` global scope is **fail-closed** and primary; RLS is defense-in-depth. No per-request state
in singletons/statics — the tag lives in `Context`, reset on `RequestTerminated`. Queued jobs carry
`tenant_id`, restore tenant context at start, reset after (`support/queue/Tenant`). Redis cache/lock/throttle
keys are tenant-namespaced. `super` is the only cross-tenant role, via an **audited** `withoutTenancy()`.
