# Style

- **ESM only:** `import`/`export`, no `require`, no mixed module systems. One default export for a module's main citizen, named exports for the rest — never both shapes for the same thing.
- **`const` first, `let` on real reassignment, `var` never.** A `let` that is never reassigned is a lie about intent.
- **Early returns over nesting:** guard clauses at the top, the happy path flows flat — `if (!id) return null;`. Three levels of indentation is a refactor signal.
- **`async/await` over then-chains;** every promise is awaited, returned, or deliberately voided with a reason — floating promises are silent failure factories. Errors handled at the boundary that can act on them, not caught-and-logged at every level.
- **Small pure functions as the unit:** one job, explicit inputs, returned outputs; side effects live at named edges. Arrow functions for expressions and callbacks, `function` declarations where hoisting or `this` matters.
- **Destructuring at the boundary:** `const { id, name } = payload` unpacks once at the top; deep property chains repeated through a body mean the destructuring is missing. `?.` and `??` are the null grammar — `||` for defaults is a bug on falsy values.
- **Modern grammar is the default:** template literals over concatenation, spread over `Object.assign`, `for…of` over index loops where the index is unused, object shorthand, optional catch binding.
- **Names and structure explain:** an unclear expression is renamed or extracted, never annotated around.
- Strict equality always (`===`); implicit coercion never carries logic.
- Consistent file naming per project convention; one module = one concern; barrel files only where the folder is a real public surface.
- Match the project's lint config exactly — the gate is the style's enforcement, never bypassed with disable-comments.
