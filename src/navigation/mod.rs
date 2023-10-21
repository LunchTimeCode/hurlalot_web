use leptos::*;
use leptos_router::*;

use crate::{about, editor, settings};
use about::About;
use editor::Editor;
use settings::Settings;

#[component]
pub fn XRouter() -> impl IntoView {
    view! {
        <div id="root">
            <Router>
                // <nav> and <main> will show on every rout
                <nav class="nav-wrapper">
                    <div>
                        <ul class="nav-ul">
                            <h1 class="nav-title">Hurlalot</h1>
                            <li class="nav-li">
                                <A href="editor">"Editor"</A>
                            </li>
                            <li class="nav-li">
                                // But we can also use a normal class attribute like it is a normal component
                                <A href="settings">"Settings"</A>
                            </li>
                            <li class="nav-li">
                                // It also supports signals!
                                <A href="about">"About"</A>
                            </li>

                        </ul>
                    </div>
                </nav>
                <main>
                    <Routes>
                        <Route path="editor" view=move || view! { <Editor/> }/>
                        <Route path="settings" view=move || view! { <Settings/> }/>
                        <Route path="about" view=move || view! { <About/> }/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
