mod analyze;
mod fileedit;

use fileedit::FileEditor;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Editor() -> impl IntoView {
    view! {
        <div class="">
            <Tabs mount=Mount::Once>
                <Tab name="tab-1" label="File 1".into_view()>
                    <FileEditor/>
                </Tab>
                <Tab name="tab-2" label="File 2".into_view()>
                    <FileEditor/>
                </Tab>

            </Tabs>

        </div>
    }
}
