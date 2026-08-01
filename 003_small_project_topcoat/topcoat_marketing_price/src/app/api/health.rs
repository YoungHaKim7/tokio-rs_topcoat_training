// GET /api/health
//
// An API route always declares its method(s) first. Returning `&'static str`
// (without wrapping in `Json`) serves it as a plain text body.
use topcoat::{Result, router::route};

#[route(GET)]
async fn health() -> Result<&'static str> {
    Ok("ok")
}
