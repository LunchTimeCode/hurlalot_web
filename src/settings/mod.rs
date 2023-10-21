use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <p>"This is Settings."</p>

        <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark/>
    }
}
