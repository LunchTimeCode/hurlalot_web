use leptonic::prelude::*;
use leptos::*;
mod about;
mod editor;
mod navigation;
mod settings;

pub fn main() {
    mount_to_body(|| {
        view! {
            <Root default_theme=LeptonicTheme::default()>
                <navigation::XRouter></navigation::XRouter>
            </Root>
        }
    })
}
