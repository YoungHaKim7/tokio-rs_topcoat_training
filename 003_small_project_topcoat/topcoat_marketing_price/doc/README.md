Done. The module-based route tree is built, compiles, and serves correctly.

## What I built

Your requested tree, mapped to routes:

```
src/
├── main.rs                          → topcoat::start(app::router())
└── app.rs                           → /  (root #[layout] <html> shell + #[page] home + mod decls)
    └── app/
        ├── about.rs                 → /about
        ├── _marketing.rs            → group: #[layout], no URL segment
        │   └── _marketing/
        │       └── pricing.rs       → /pricing   (wrapped by marketing + root layouts)
        ├── posts.rs                 → /posts
        │   └── posts/
        │       └── id.rs            → /posts/{post_id}   (#[path_param] PostId(u64))
        └── api.rs                   → /api/* (#[layer] logging)
            └── api/
                └── health.rs        → GET /api/health
```

## Two notes

**1. I added `app/api.rs`** (your tree only showed `api/health.rs`). This is required by Rust's module system — `module_router!` does not scan the filesystem; every module must be reached through `mod` declarations. So `app.rs` needs `mod api;`, and `api.rs` needs `mod health;` for `api/health.rs` to compile at all. I used it to host a small `#[layer]` that logs API responses (the canonical pattern from the docs), which is why you saw `[api] response status: 200 OK` in the smoke test.

**2. The `{post_id}` name comes from the struct, not the file.** The file is `id.rs`, but `#[path_param] struct PostId(u64)` makes the segment `{post_id}` (snake-cased struct name). That's why `/posts/not-a-number` returns **400** — the declared `error = bad_request` fires on the parse failure.

## How it works

- `module_router!()` is called in `app.rs` (the route root → `/`) and auto-registers every reachable `#[page]`/`#[layout]`/`#[layer]`/`#[route]` with no explicit path.
- Underscore-prefixed `_marketing` is a **group**: contributes no URL segment, but its layout still wraps descendants.
- `.discover()` on the builder is there to also pick up any future explicit-path handlers, fonts, and assets.

## Run it

```fish
cargo run    # serves on 127.0.0.1:3000 (override with HOST/PORT env)
```

The IDE em-dash squiggles were false positives from rust-analyzer not expanding the `view!` proc-macro — `cargo build` is the source of truth and it's clean. I used ASCII hyphens anyway to keep the editor quiet.
