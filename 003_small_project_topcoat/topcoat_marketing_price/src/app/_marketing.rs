// A group module: the leading underscore means it contributes NO URL segment,
// but layouts and layers here still wrap its descendants. So `_marketing` is
// invisible in URLs, yet its layout wraps `/pricing` (and any sibling under
// `_marketing/`). The root layout at `/` wraps this one from the outside.
use topcoat::{Result, router::layout, view::view};

#[layout]
async fn marketing_layout(slot: Result) -> Result {
    view! {
        <section class="marketing">
            <h2>"Marketing"</h2>
            (slot?)
        </section>
    }
}

mod pricing;
