# Routes

- **Per-panel files:** `routes/apis/<panel>.php` included by `routes/api.php`, prefix `/v1`, per-panel name prefix + middleware group. Panel spelling is fixed and identical across route prefix, role, and permissions.
- **Reusable named blocks in `routes/apis/shared.php`**, invoked explicitly per panel — NOT `glob()`/reflection auto-registration. Guard each block with `function_exists` so `route:cache` rebuilds cleanly.
- **`route:cache`-safe is law:** every handler is a `'method'` string on a `->controller(...)` group — route closures are forbidden (they break the cache).

```php
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

- The uniform action set every `resource()` exposes (all permission-gated): `index · statistics · store · show · related · update · delete/deleteMany · file actions · download`. **Nested relations are real controller actions** resolving `{relation}` against the model's derived map, fail-closed (404 on unknown) — never a closure.
- **Blocks take opt-outs, never grow forks:** the block signature carries an `$except` list (`resource('carts', except: ['statistics', 'files'])`) checked by one `$can` closure per action — a panel exposing a subset declares the subtraction inline; copying the block to delete two lines is the fork that rots. Companion blocks follow the same shape: `related(['comments', 'reviews'])` registers the nested sub-trees per relation, permission-gated per relation name.
- **Two shared files, two jobs:** `blocks.php` = the reusable block FUNCTIONS (`function_exists`-guarded); `shared.php` = concrete route groups every panel mounts as-is (account, tokens, notifications) — a concrete group repeated across panel files belongs in shared; a shape repeated across resources belongs in blocks.
- **Permissions derive from the segment:** `view_/add_/edit_/delete_<resource>` via `has:<permission>` middleware; declare only cross-cutting flags (`allow_statistics`, `allow_downloads`, …). Route names mirror the segment per panel: `admin.products.index`.
- **The API contract = the `routes/collections/` mirror, not OpenAPI:** `routes/collections/<panel>.json` mirrors its routes file 1:1, Postman-compatible (v2.1 schema) so the frontend team imports it directly — collection variables (`base_url`, `token`, `tenant_domain`, `locale`), an info block documenting auth/tenant/locale headers and the envelope, items grouped per resource, each request carrying method, URL, headers, body, the ROUTE NAME in its description, and a saved response example. **Any API change updates its collection entry in the SAME change.** The mirror is mandatory — and GENERATED: a collection builder command emits it from the route table + request metadata; regenerate in the change, never hand-edit the JSON.
- **Middleware stacks are named vocabulary, not repeated lists:** one utility composes the canonical stacks — `auth(role)` returning the `[auth:sanctum, role:<x>, has:allow_logins, throttle, idempotent, verified]`-shaped array, `tenant()`, role-aware `rate()` budgets (an admin's throttle ceiling ≠ a client's) — panels attach the named stack; a route group hand-assembling five middleware is how one route ships without `verified`.
