# Style

The owner's hand — there is **no `cargo fmt`** in the gate because this hand is deliberate. Read neighbouring files and match them exactly; code must be indistinguishable from the owner's.

- **Declarations breathe:** a space before `(` and spaces inside it on fn signatures — `pub fn resolve ( name: &str, depth: usize ) -> Flow<String> {`. Plain **calls do not**: `resolve(name, 2)`.
- **Breathing bodies:** one blank line after an opening `{` and one before its closing `}` on fns and impl blocks. No blank line *between* consecutive fns — the closing `}` is immediately followed by the next signature.
- **Allman else:** the `}` closes on its own line, `else {` opens on the next — never `} else {`.
- **Compact match:** small arms inline on one line, aligned by `=>`; a match that fits in five short arms never becomes twenty lines of blocks.
- **Aligned declaration groups:** consecutive `const`/field/`let` groups align their `=`/`:` columns; grouped, related lines read as a table.
- **Structure over prose:** no banners, no `// TODO` graveyards — rename and restructure until the code explains itself; clarity debt is paid at the name.
- **One-line guards welcome:** `if !path.exists() { return Ok(()); }` — early return over nesting, always.
- No one-letter names outside tight closures; no abbreviations that need decoding. Short full words: `cfg`, `paths`, `journey` — names carry the meaning comments would have.

```rust
pub fn locate ( dir: &Path, name: &str ) -> Option<PathBuf> {

    let target = Text::unprefix(name).to_lowercase();

    Dir::entries(dir)
        .into_iter()
        .filter(|entry| Text::unprefix(&Path::name_of(entry)).to_lowercase() == target)
        .next_back()

}
```

- Match the file's exact spacing before adding to it — the hand-format IS the format; there is no `cargo fmt` in the gate by design, never introduce it.
