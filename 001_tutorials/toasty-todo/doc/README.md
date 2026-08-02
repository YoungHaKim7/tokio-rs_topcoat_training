# fix
- Compiles cleanly. 

## What changed

The tutorial targets **topcoat 0.5.0**, where the `path_param!(name: T, ...)` *function-like macro* was replaced with a `#[path_param]` **attribute** on a single-field tuple struct.

**Before** (old macro API):
```rust
path_param!(todo_id: u64, error = bad_request);
```

**After** (current attribute API):
```rust
#[path_param(error = bad_request)]
struct TodoId(u64);
```

This single change fixed all three errors:

1. **`path_param` macro not found** — `path_param` is now an *attribute*, not a `!`-invoked macro. The import on line 10 was already correct; only the call site was using the obsolete form.
2. **`TodoId` not found (×2)** — the old `path_param!` macro used to generate the `TodoId` type behind the scenes. In the new API, you write the struct yourself, and the attribute generates the `PathParam` impl + the `{todo_id}` route segment for it.

The handler bodies (`path_param::<TodoId>(cx)?`) needed no change — in topcoat 0.5.0 the same `path_param` name resolves to the attribute in `#[...]` position and to the free function in call position (separate namespaces), exactly as the [crate's own docs](https://docs.rs/topcoat/0.5.0/topcoat/router/attr.path_param.html) show.

Note that the name mapping still holds: `TodoId` → snake_case `todo_id` → matches the `{todo_id}` segments in `/todos/{todo_id}/toggle` and `/todos/{todo_id}/delete`.
