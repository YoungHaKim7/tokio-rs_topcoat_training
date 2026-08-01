# Result


```bash
❯ topcoat dev

  topcoat dev server
  watching for file changes...
  press r to reload

  ready on http://127.0.0.1:3000


```

✅ **Build fixed.** `cargo build --release` finished clean (exit 0, 13.2s) — all 239 crates compiled, including the build script's Tailwind scan, `topcoat v0.5.0`, and your `tailwind` crate.

`cargo r --release` will now work — it'll start the dev server at `http://127.0.0.1:3000`.

### What was wrong
The `[build-dependencies]` line had been edited down to:
```toml
topcoat = { features = ["tailwind"] }
```
A dependency with **no source** — no `version`, `path`, `git`, or `workspace`. Cargo rejects that at manifest-parse time, which is the exact `dependency (topcoat) specified without providing a local path, Git repository, version, or workspace dependency to use` error. Your `[dependencies]` line was already fine — unlike what the upstream workspace comment implies, the `topcoat` facade **is** published on crates.io (`topcoat = "0.5.0"`).

### What changed
- `[build-dependencies] topcoat` → given the crates.io source with `default-features = false` (build script only needs the Tailwind scanner):
  ```toml
  topcoat = { version = "0.5.0", default-features = false, features = ["tailwind"] }
  ```
- Deleted the stale 58 KB `Cargo.lock` (it was generated against the monorepo's path deps); cargo regenerated it from crates.io.

The crate is now fully standalone — no topcoat monorepo checkout needed, which fits your numbered `001_tutorials/` training layout.

One note for the rest of your training crates: topcoat pins **rust-version 1.95** and **edition 2024** — your toolchain (1.96.1) covers both.

