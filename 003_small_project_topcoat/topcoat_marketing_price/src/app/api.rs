// The `api` module serves `/api/*`. It also hosts a `#[layer]` that wraps every
// handler under `/api`, here just logging response statuses.
//
// Rust requires this file (or `api/mod.rs`) to declare `mod health;` so that
// `api/health.rs` is compiled and reachable from the route tree.
use topcoat::{
    Result,
    context::CxBuilder,
    router::{Body, Next, Response, layer},
};

// A layer wraps handlers whose path begins with `/api`. Call `next.run` to
// invoke the inner handler; the return value becomes the response.
#[layer]
async fn api_log(cx: &mut CxBuilder, body: Body, next: Next<'_>) -> Result<Response> {
    let response = next.run(cx, body).await?;
    println!("[api] response status: {}", response.status());
    Ok(response)
}

mod health;
