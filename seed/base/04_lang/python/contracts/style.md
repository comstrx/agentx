# Style

The owner's hand — the same hand as every other language in the stack, expressed in Python. Match neighbouring files exactly.

- **Declarations breathe:** a space before `(` and spaces inside on `def` signatures — `def resolve ( self, path: str, depth: int = 0 ):`. Plain calls do not: `resolve(path, 2)`.
- **Breathing bodies:** one blank line right after a `def` line before its first statement; methods separated by a single blank line; classes by two.
- **One-line guards welcome:** `if not value: return ''` — early return over nesting, always.
- **Aligned assignment groups:** consecutive related assignments align their `=` column; grouped configuration reads as a table:

```python
class File(Checker, Editor):

    def __init__ ( self ):

        self.compressor = Compressor()
        self.encryptor  = Encryptor()
        self.manager    = Manager
        self.watcher    = Watcher

    def name ( self, path: str ) -> str:

        return os.path.basename(path)

    def extension ( self, path: str ) -> str:

        return os.path.splitext(path)[1].lstrip('.')
```

- **Facade classes compose mixins:** a domain's public class inherits its focused capability classes (`File(Checker, Editor)`) and wires its helpers in `__init__` — the shell-and-engine shape, Pythonic.
- **Imports grouped on shared lines by kinship:** `import os, re, time, shutil` for the stdlib cluster; `from typing import Any, Callable`; relative imports for siblings. Import block is compact, top-of-file, no mid-file imports.
- **Prose never substitutes structure:** no docstrings as decoration — names, types, and shape carry the meaning; an unclear unit is renamed or split, never annotated around.
- Decorators express cross-cutting behaviour (`@Handler.skip`, `@cached`) — declared at the definition, never wrapped at call sites.
- f-strings only for interpolation; `snake_case` functions, `PascalCase` classes, `SCREAMING_SNAKE` constants — no Hungarian, no abbreviation archaeology.
