// GET /posts/{post_id}
//
// `#[path_param]` turns this module's segment into a dynamic parameter. The
// segment name comes from the struct (`PostId` -> `{post_id}`), not from the
// file name, so `id.rs` still produces `/posts/{post_id}`. The inner `u64` is
// parsed via `FromStr`; a value that fails to parse responds `400 Bad Request`.
use topcoat::{
    Result,
    context::Cx,
    router::{page, path_param},
    view::view,
};

#[path_param(error = bad_request)]
struct PostId(u64);

#[page]
async fn post(cx: &Cx) -> Result {
    // `path_param::<PostId>(cx)` returns `Result<&u64, _>`: a reference to the
    // parsed inner value. `?` propagates a parse failure as the declared 400.
    let post_id = path_param::<PostId>(cx)?;
    view! {
        <h1>"Post " (post_id)</h1>
        <p><a href="/posts">"Back to posts"</a></p>
    }
}
