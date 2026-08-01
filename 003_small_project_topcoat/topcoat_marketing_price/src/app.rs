// Root module of the route tree.
//
// `module_router!()` is called here, so this module maps to `/`. Every
// module-derived `#[page]`, `#[layout]`, `#[layer]`, and `#[route]` reachable
// from here through `mod` declarations is registered automatically. The route
// tree mirrors the file tree under `src/app/`.
use topcoat::{
    Result,
    router::{Router, RouterBuilderDiscoverExt, layout, page},
    view::view,
};

/// Builds the application router from the module tree.
///
/// `module_router!()` collects every module-derived handler (the ones below);
/// `.discover()` additionally pulls in anything Topcoat collects at link time,
/// such as explicit-path handlers, fonts, and assets.
pub fn router() -> Router {
    topcoat::router::module_router!().discover().build()
}

// The root layout wraps every page in the HTML document shell. Its URL is the
// module path `/`, so it applies to all descendants. The rendered inner page is
// passed as `slot` and embedded with `?`.
#[layout]
async fn root_layout(slot: Result) -> Result {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"Topcoat Marketing Price"</title>
            </head>
            <body>
                <nav>
                    <a href="/">"Home"</a>
                    " "
                    <a href="/about">"About"</a>
                    " "
                    <a href="/pricing">"Pricing"</a>
                    " "
                    <a href="/posts">"Posts"</a>
                </nav>
                <main>
                    (slot?)
                </main>
            </body>
        </html>
    }
}

// GET /
#[page]
async fn home() -> Result {
    view! {
        <h1>"Welcome"</h1>
        <p>"A small Topcoat app exploring module-based routing."</p>
    }
}

// Child route modules. Each contributes one URL segment (or none, for groups).
mod about;
mod _marketing;
mod posts;
mod api;
