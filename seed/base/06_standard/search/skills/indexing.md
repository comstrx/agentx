# Indexing

The craft of keeping a fast index honest.

- **Design the document from the queries backwards:** list the real searches the product runs (prefix lookups, faceted lists, full-text) — the document shape, analyzers, and facets derive from that list; indexing everything "in case" bloats the index and the reindex time both.
- **One indexable contract per entity:** the model declares its searchable projection (`toSearchable()`-shaped: fields, facets, scope keys) in ONE place — the queue consumer, the rebuild command, and the initial import all serialize through it; two serializers drift by Thursday.
- **Versioned updates kill the race:** index writes carry the entity's version/updated-at; the engine rejects older-than-current — an out-of-order queue pair must not leave yesterday's title winning. Deletes index as tombstones through the same path.
- **Rebuild without downtime:** build into a new index, alias-swap atomically, drop the old — the alias is the only name the app knows. Rebuilds are chunked by keyset, resumable, and rate-limited so the primary store never notices.
- **Facets and filters are engine-side:** counts and buckets computed in the engine per scoped query — never fetched-then-counted in application memory; a facet the engine cannot compute cheap is a document-shape problem.
- **Autocomplete is its own shape:** edge-ngram/completion fields tuned for prefix speed, small payloads, aggressive caching — bolting suggestions onto the full-text query is why search boxes lag.
- **Relevance iterates on evidence:** capture real query logs + click-through, keep a golden set of query→expected-top-results as a regression suite, tune boosts against it — relevance changes without a measurement are vibes with a deploy.
- **Language matters:** analyzers per locale (stemming, stopwords), and Arabic-shaped concerns handled deliberately — normalization of hamza/teh-marbuta forms, diacritics stripping — tested with native queries, not transliterated guesses.
- Monitor the seams: queue lag, engine errors, index/source count drift on a schedule — silent index rot is found by customers otherwise.
