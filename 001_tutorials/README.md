# learn topcoat
- https://github.com/tokio-rs/topcoat#learn-topcoat

# Getting started
- https://github.com/tokio-rs/topcoat/blob/main/crates/topcoat/docs/getting_started.md

# 해외 유튜브 영상
- [260731) Rust의 새로운 풀스택 프레임워크, Topcoat에 대해 알아야 할 7가지_자동 더빙 Francesco Ciulla](https://youtu.be/gvo9KzADsjQ?si=xMGk_x7AEaWmsdAA)

# tailwind `Cargo.toml` patterns

- `Cargo.toml`

```toml
[dependencies]
tokio = { version = "1.53.1", features = ["rt-multi-thread", "macros"] }
topcoat = { version = "0.5.0", features = ["tailwind"] }


[build-dependencies]
# The build script only needs the Tailwind scanner, so disable the heavy
# default feature set. (cargo cannot combine `workspace = true` with
# `default-features = false`, so this entry is spelled out with a version.)
topcoat = { version = "0.5.0", default-features = false, features = ["tailwind"] }
```

# lint pattern

- `Cargo.toml`

```toml
[lints.rust]
unsafe_code = "deny"

[lints.rustdoc]
broken_intra_doc_links = "deny" 

[lints.clippy]
mod_module_files = "deny"
pedantic = { level = "warn", priority = -1 }
too_many_lines = "allow"

[profile.dev]
debug = true
```

