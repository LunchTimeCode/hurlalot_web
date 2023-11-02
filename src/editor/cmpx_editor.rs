use leptos::*;

#[component]
pub fn CmpxEditor(content: ReadSignal<String>, set_content: WriteSignal<String>) -> impl IntoView {
    view! {
        <textarea
            class="code-editor"
            type="text"
            on:input=move |ev| {
                set_content.set(event_target_value(&ev));
            }

            prop:value=content.get()
            rows=40
            style="margin: 10px;"
        ></textarea>
    }
}
