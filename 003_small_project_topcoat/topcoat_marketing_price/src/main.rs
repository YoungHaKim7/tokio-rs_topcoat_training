// The binary entry point. The entire route tree lives under the `app` module,
// which builds the router from its module structure via `module_router!()`.
mod app;

#[tokio::main]
async fn main() {
    // `app::router()` returns the fully-built `Router`; `topcoat::start` binds a
    // listener (host/port from the environment) and serves it.
    topcoat::start(app::router())
        .await
        .expect("topcoat server failed");
}
