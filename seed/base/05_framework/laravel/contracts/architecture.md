# Architecture

- Repository pattern, layered. Build order (inner → outer): `Support → Traits → Bases → Repository → Service → Controller → Request → Middleware → Route`. Runtime flow is the reverse: `route → middleware → FormRequest → controller → service → repository → model (+ traits) → Support`. Each layer depends only inward; a controller never touches a model directly.
- **Structure is FLAT, not modular domains:** `app/Models/*`, `app/Repositories/*Repository.php`, `app/Services/*Service.php`, `app/Http/Controllers/*Controller.php`, `app/Http/Requests/*`, `app/Http/Resources/*`. One unified `XxxController` serves all panels; behaviour differs by the actor context, never by parallel controller trees.
- **The Base engine (the magic):** each layer's real logic lives in a `HasBaseXxx` trait under `app/Traits/Bases/` — `HasBaseModel`, `HasBaseRepository`, `HasBaseService`, `HasBaseController`, `HasBaseRequest`, `HasBaseResource`, `HasBaseCommand`. Each layer has a thin `BaseXxx` shell that only `use`s its trait and wires the inner layer in its constructor; concretes extend the shell and declare only differences (`fields()`, an override, relations). New shared behaviour goes in the trait, **never** in a concrete class.

```php
class BaseRepository {

    use HasBaseRepository;

    public function __construct ( protected Model $model ) {}

}
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

- **`app/Traits/` holds exactly THREE families, nothing loose:** `Bases/` (the per-layer engine traits) · `Dna/` (opt-in model capabilities: tenancy, files, search, permissions, relations, morphs, translate, logs, journey/state, … — each a folder with its `index.php` facade; a large cohesive subsystem becomes focused sibling traits composed by one facade) · `Util/` (the **static cross-layer vocabulary**: `Access::permits(...)`, `Auth::clientId()`, `Response::ok()`, `Exception`, `Tenant`, `Schema`, `Locale` — the short words service pipelines and engine traits speak; thin, stateless, callable from any layer). Every family is built on the Support DSL — traits carry behaviour, Util carries the pipeline's grammar, Support carries power.
- **Business code is pure pipeline:** a service method is a sequence of named calls into Support + the engine — no native PHP (`array_*`, `preg_*`, hand loops, `json_*`) and no raw framework calls (`DB::`, `Http::`, `Cache::` inline) inside a business method. Missing capability → add it to Support, then call it.
- Ids are **UUIDv7 `string` everywhere**, never `int`.
- **The streamlined skeleton is respected:** middleware, exceptions, and providers are configured in `bootstrap/app.php`; scheduling lives in `routes/console.php`; no resurrected Kernel classes. Work with the current skeleton, never recreate the old one.
