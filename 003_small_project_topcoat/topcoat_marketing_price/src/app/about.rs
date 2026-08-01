// GET /about
//
// A regular route module: its name `about` becomes the `/about` segment.
use topcoat::{Result, router::page, view::view};

#[page]
async fn about() -> Result {
    view! {
        <h1>"About"</h1>
        <p>"This page is served at /about, derived from its module path."</p>
    }
}
