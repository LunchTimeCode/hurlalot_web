use leptonic::prelude::*;
use leptos::*;
mod editor;

use editor::Editor;

pub fn main() {
    mount_to_body(|| {
        view! {
            <Root default_theme=LeptonicTheme::default()>
                <div class="nav-wrapper">
                    <ul class="nav-ul">
                        <h1 class="nav-title">Hurlalot</h1>
                    </ul>
                </div>

                <Editor/>
            </Root>
        }
    })
}
