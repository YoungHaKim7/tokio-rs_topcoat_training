# htmx doc
- https://htmx.org/docs/

# Result


```bash


```


- Fixed. `cargo check` passes cleanly.

## What was wrong

In topcoat, `<script>` is **not** a raw-text element like in HTML — the `view!` macro parses its children as ordinary view nodes (string literals, elements, `(expr)`, control flow). So the bare line `htmx.on(...)` was read as a node and rejected with `expected view node`. A second problem had also crept in: a stray nested `view!{ … }` (with no macro name resolution) plus an `<h2>` wrapping a `<form>`.

## What I changed (`src/main.rs`)

1. **Imported `Unescaped`** from `topcoat::view` — the type topcoat provides for trusted, verbatim markup.
2. **Wrapped the JS body in `(Unescaped::new_unchecked(r##" … "##))`** so it's emitted as-is.
   - Used `r##"…"##` (two hashes) because the JS contains `"#` (in `"#form"`), which would prematurely close a `r#"…"#` raw string.
3. **Removed the broken scaffolding** around it: the nested `view!{ }`, the `<h2>...</h2>` wrapper (a block-level form inside a heading is invalid), and the duplicate `topcoat::dev::script()` (already loaded once in the layout `<head>`).

## Why `Unescaped` instead of a plain string literal

A bare `"…"` text node gets HTML-escaped (`<`→`&lt;`, `>`→`&gt;`, `&`→`&amp;`). That happens to round-trip for this snippet, but JS routinely contains `=>` and comparisons (`i < len`) — escaping would silently corrupt the script. `Unescaped` is the correct tool for a trusted `<script>` body.

Your `topcoat dev` server should now pick up the change and rebuild green.
