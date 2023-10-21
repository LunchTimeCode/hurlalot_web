use leptos::*;

use crate::editor::analyze::analyze;

#[component]
pub fn FileEditor() -> impl IntoView {
    let (content, set_content) = create_signal("".to_string());
    let (chars, set_chars) = create_signal("".to_string());

    create_effect(move |_| {
        // immediately prints "Value: 0" and subscribes to `a`
        set_chars.set(analyze(&content.get()))
    });

    view! {
        <textarea
            class="code-editor"
            type="text"
            on:input=move |ev| {
                set_content.set(event_target_value(&ev));
            }

            prop:value=content.get()
            rows=40
            style="margin: 10px; width: 80%"
        ></textarea>

        <p>chars with spaces:</p>
        <p>{chars}</p>
    }
}
