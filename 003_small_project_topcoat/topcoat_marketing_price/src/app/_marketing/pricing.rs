// GET /pricing
//
// The `_marketing` group contributes no segment, so this module's name `pricing`
// becomes a top-level `/pricing` segment. It is wrapped by both the marketing
// layout and the root layout.
use topcoat::{Result, router::page, view::view};

#[page]
async fn pricing() -> Result {
    view! {
        <h1>"Pricing"</h1>
        <ul>
            <li><strong>"Starter"</strong>" - $0/mo"</li>
            <li><strong>"Pro"</strong>" - $19/mo"</li>
            <li><strong>"Team"</strong>" - $49/mo"</li>
        </ul>
    }
}
