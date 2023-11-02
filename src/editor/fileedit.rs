use super::cmpx_editor::CmpxEditor;
use crate::editor::analyze::parse;
use leptonic::prelude::Button;
use leptonic::prelude::Col;
use leptonic::prelude::Drawer;
use leptonic::prelude::DrawerSide;
use leptonic::prelude::Grid;
use leptonic::prelude::LeptonicTheme;
use leptonic::prelude::Row;
use leptonic::prelude::Size;
use leptonic::prelude::Stack;
use leptonic::prelude::TextInput;
use leptonic::prelude::ThemeToggle;
use leptos::leptos_dom::helpers::IntervalHandle;
use leptos::*;
use web_time::Duration;

#[component]
pub fn FileEditor() -> impl IntoView {
    let (shown, set_shown) = create_signal(false);

    let (api_url, set_api_url) = create_signal("https://hurlalot.shuttleapp.rs".to_string());

    let (content, set_content) = create_signal("".to_string());
    let (content_to_parse, set_content_to_parse) = create_signal("".to_string());

    let (time_over, set_time_over) = create_signal(false);

    let (result, set_result) = create_signal("".to_string());

    let _ = create_resource(
        move || content_to_parse.get(),
        // every time `count` changes, this will run
        move |value| async move {
            let url = api_url.get_untracked();
            logging::log!("loading data from API");
            let parse_res = parse(&value, &format!("{}/api/parse", &url)).await;
            if let Some(res) = parse_res {
                set_result.set(res)
            } else {
                set_result.set("not able to reach server".to_string())
            };
            "nothing".to_string()
        },
    );

    let (interval, _) = create_signal(100);
    let (debounce, _) = create_signal(200);

    use_interval(debounce, move || set_time_over.set(true));

    use_interval(interval, move || {
        if time_over.get() {
            set_content_to_parse.set(content.get());

            set_time_over.set(false)
        }
    });

    view! {
        <Drawer
            side=DrawerSide::Right
            shown=shown
            style="padding: 0.5em; height: 19.5em; overflow: scroll; position: absolute; top: 0; right: 0; background-color: var(--brand-color); border-left: 1px solid gray;"
        >
            <Stack spacing=Size::Em(0.5)>
                <div class="code-editor-box">
                    <TextInput get=api_url set=set_api_url/>

                    <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark/>
                </div>

            </Stack>
        </Drawer>
        <div class="code-editor-box">
            <Stack spacing=Size::Em(0.6)>

                <Grid spacing=Size::Em(0.6) style="">

                    <Row>
                        // editor region
                        <Col md=6 sm=1 xs=1>
                            <CmpxEditor content=content set_content=set_content/>
                        </Col>
                        // editor region end

                        // output region start
                        <Col md=0 sm=1 xs=1>

                            <Grid spacing=Size::Em(0.6) style="">
                                <Row>
                                    <Col md=0 sm=1 xs=1>
                                        <p>{result}</p>
                                    </Col>
                                </Row>

                            </Grid>

                        </Col>
                    // output region end
                    </Row>

                    // settings region start
                    <Row>
                        <Col md=3 sm=4 xs=6>
                            <Button on_click=move |_| {
                                set_shown.set(!shown.get())
                            }>Editor Settings</Button>
                        </Col>
                    </Row>
                </Grid>
            // settings region end
            </Stack>

        </div>
    }
}

pub fn use_interval<T, F>(interval_millis: T, f: F)
where
    F: Fn() + Clone + 'static,
    T: Into<MaybeSignal<u64>> + 'static,
{
    let interval_millis = interval_millis.into();
    create_effect(move |prev_handle: Option<IntervalHandle>| {
        // effects get their previous return value as an argument
        // each time the effect runs, it will return the interval handle
        // so if we have a previous one, we cancel it
        if let Some(prev_handle) = prev_handle {
            prev_handle.clear();
        };

        // here, we return the handle
        set_interval_with_handle(
            f.clone(),
            // this is the only reactive access, so this effect will only
            // re-run when the interval changes
            Duration::from_millis(interval_millis.get()),
        )
        .expect("could not create interval")
    });
}
