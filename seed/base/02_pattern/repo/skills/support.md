# Support

Growing the project's private std-lib — the layer every other layer stands on.

- One **folder per domain** (str, arr, cast, date, num, json, parse, file, http, cache, lock, queue, event, storage, log, mail, net, security, validate, context, database, request, response, throttle). First level is folders only, never loose files — even a one-file helper becomes a folder with an entry facade.
- Each folder exposes **one facade** as its public surface; internal pieces are focused sibling classes behind it. Callers touch the facade only.
- Support is **native/infrastructure power exclusively — zero business logic.** No business noun ever appears here; the moment one does, the code belongs in a higher layer.
- Infrastructure domains that could ever change backend are **adapters**: facade + `Driver` interface + concrete drivers (cache, lock, throttle, queue, event, storage, payments, search, mail, ai). Swap = one new driver file + config. Keep the neutral interface even with one backend.
- Grow **on demand**: a domain earns a new method when a feature needs it — built once, named as a clear noun/verb, added at the facade or as an internal piece. Never build ahead of need; never let the needing feature inline the capability instead.
- Naming discipline: short clear nouns, no reserved keywords as class names, consistent verb vocabulary across domains (`get/set/has/is/make/parse/format`). The facade name may deliberately shadow a framework equivalent — prefer ours; alias the framework's on genuine collision.
- Safety domains are wrappers, not inventions: security wraps vetted crypto primitives, log redacts secrets structurally, http guards outbound calls (SSRF, timeouts, retries). Support is where such guarantees are enforced once for everyone.
- The health test: support reads like a standard library manual — any engineer can find the capability by guessing the domain and the name. If a capability is hard to find, it is misnamed or misplaced.
