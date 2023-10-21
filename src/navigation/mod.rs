use leptos::*;
use leptos_router::*;

use crate::{about, editor, settings};
use about::About;
use editor::Editor;
use settings::Settings;

const TITLE: &str = "text-xl text-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700";
const NAVT: &str = "block mt-1 mb-2 text-white active:underline focus:underline";
const NAVB: &str = "bg-gray-900 w-full z-20 top-1 left-1 border-b border-gray-200 border-gray-600";
const NAVLIST: &str = "flex flex-col p-4 md:p-0 mt-4 font-medium border border-gray-100 bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700";

#[component]
pub fn XRouter() -> impl IntoView {
    view! {
        <div id="root">
            <Router>
                // <nav> and <main> will show on every rout
                <nav class=NAVB>
                    <div class=NAVLIST>
                        <p class=TITLE>Hurlalot</p>
                        <ul class=NAVLIST>
                            <li class="">
                                <A href="editor" class=NAVT>
                                    "Editor"
                                </A>
                            </li>
                            <li class="">
                                // But we can also use a normal class attribute like it is a normal component
                                <A href="settings" class=NAVT>
                                    "Settings"
                                </A>
                            </li>
                            <li class="">
                                // It also supports signals!
                                <A href="about" class=move || NAVT>
                                    "About"
                                </A>
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
