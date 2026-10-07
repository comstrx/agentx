# Speed

The 8-hours-a-day bar — an operator lives in this panel; every saved second is payroll.

- **Keyboard-first is the power law:** global command palette (go to resource, run action, search everything) · list navigation without the mouse (j/k or arrows, enter to open, shortcuts on row actions) · form flow that tabs sanely and submits from the keyboard · shortcuts discoverable (a `?` overlay) — a panel operated 8 hours by mouse alone is ergonomic debt billed daily.
- **Latency budgets for the working set:** list interactions (filter, sort, page) paint feedback instantly and settle < 500ms · row → detail feels immediate (prefetch on hover/intent, render from the list's cache while fresh data loads) · saves acknowledge < 1s or go optimistic — the operator's rhythm breaks at the second second.
- **Saved views are the operator's muscle memory:** filter + sort + column sets saved per user per resource, shareable to the team, with a default view per role — rebuilding the same filter every morning is a designed-in time tax.
- **Bulk operations are engineered, not looped:** select-across-pages (the filter IS the selection), server-side bulk endpoints with per-item outcome reporting (succeeded/refused/failed lists), progress for the long ones, undo where the domain allows — forty deletes as forty requests with forty toasts is a DoS of the operator's attention.
- **Optimistic where cheap, receipted where not:** toggles/status flips apply instantly with rollback; money and state-machine transitions show their real progress — the split mirrors the domain's own risk map.
- **Inline editing for the single-field fix:** click-to-edit on cells where permissions allow, validated the same as forms, saved on blur/enter — opening a full form to fix a typo is four clicks that should be one.
- **Virtualize the dense:** tables past the threshold render windowed rows (the table engine's job, free for every resource); column sets stay user-controlled so wide tables scroll less — density is the panel's nature; jank at density is the defect.
- **Prefetch the working rhythm:** the next page on scroll intent, the detail on row hover, the related tabs' first page when a detail opens — the operator's next click is statistically known; the panel is already holding it.
- **Duplicate-and-edit beats blank forms:** creating the fifth similar record starts from a copy of the fourth (minus uniques) — data entry work has patterns; the panel that notices them wins the operator's loyalty.
- Measure the panel like a product: time-to-first-interaction per screen, action latency p95, palette usage, saved-view adoption — the operator experience regresses silently unless these numbers are watched.
