use leptos::*;

mod about;
mod editor;
mod navigation;
mod settings;

pub fn main() {
    mount_to_body(|| view! { <navigation::XRouter /> })
}
