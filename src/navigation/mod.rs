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
                                <A href="hurlalot_web/editor">"Editor"</A>
                            </li>
                            <li class="nav-li">
                                // But we can also use a normal class attribute like it is a normal component
                                <A href="hurlalot_web/settings">"Settings"</A>
                            </li>
                            <li class="nav-li">
                                // It also supports signals!
                                <A href="hurlalot_web/about">"About"</A>
                            </li>

                        </ul>
                    </div>
                </nav>
                <main>
                    <Routes>
                        <Route path="" view=move || view! { <Editor/> }/>
                        <Route path="hurlalot_web/editor" view=move || view! { <Editor/> }/>
                        <Route path="hurlalot_web/settings" view=move || view! { <Settings/> }/>
                        <Route path="hurlalot_web/about" view=move || view! { <About/> }/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
