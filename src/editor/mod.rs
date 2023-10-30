mod analyze;
mod fileedit;

use fileedit::FileEditor;
use leptos::*;

#[component]
pub fn Editor() -> impl IntoView {
    view! { <FileEditor/> }
}
