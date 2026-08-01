// GET /posts
//
// This module serves `/posts` AND is the parent of the dynamic `id` module,
// which serves `/posts/{post_id}`.
use topcoat::{Result, router::page, view::view};

#[page]
async fn posts() -> Result {
    view! {
        <h1>"Posts"</h1>
        <p>"Browse all posts, or open one directly, e.g. " <a href="/posts/42">"/posts/42"</a> "."</p>
    }
}

mod id;
