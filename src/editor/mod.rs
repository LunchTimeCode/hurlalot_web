mod analyze;
mod fileedit;
mod cmpx_editor;

use fileedit::FileEditor;
use leptos::*;

#[component]
pub fn Editor() -> impl IntoView {
    view! { <FileEditor/> }
}
