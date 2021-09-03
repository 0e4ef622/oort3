pub mod api;
pub mod game;
pub mod ui;
pub mod worker_api;

use chrono::NaiveDateTime;
use game::Game;
use monaco::sys::editor::{IEditorMinimapOptions, IStandaloneEditorConstructionOptions};
use monaco::{
    api::CodeEditorOptions, sys::editor::BuiltinTheme, yew::CodeEditor, yew::CodeEditorLink,
};
use oort_simulator::scenario;
use rbtag::{BuildDateTime, BuildInfo};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::services::render::{RenderService, RenderTask};

#[derive(BuildDateTime, BuildInfo)]
struct BuildTag;

pub fn version() -> String {
    let build_time = NaiveDateTime::from_timestamp(
        BuildTag {}
            .get_build_timestamp()
            .parse::<i64>()
            .unwrap_or(0),
        0,
    );

    let commit = BuildTag {}.get_build_commit();

    if commit.contains("dirty") {
        commit.to_string()
    } else {
        format!("{} {}", build_time.format("%Y%m%d.%H%M%S"), commit)
    }
}

fn make_editor_options() -> CodeEditorOptions {
    let initial_text = "\
// Welcome to Oort.
// Select a scenario from the list in the top-right of the page.
// If you're new, start with 'tutorial01'.";
    CodeEditorOptions::default()
        .with_language("rust".to_owned())
        .with_value(initial_text.to_owned())
        .with_builtin_theme(BuiltinTheme::VsDark)
}

fn make_real_editor_options() -> IStandaloneEditorConstructionOptions {
    let options = make_editor_options().to_sys_options();
    options.set_automatic_layout(Some(true));
    let minimap: IEditorMinimapOptions = js_sys::Object::new().unchecked_into();
    minimap.set_enabled(Some(false));
    options.set_minimap(Some(&minimap));
    options
}

enum Msg {
    Render,
    SelectScenario(String),
    EditorCreated(CodeEditorLink),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    render_task: RenderTask,
    game: Game,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::SelectScenario("welcome".to_string()));
        let link2 = link.clone();
        let render_task = RenderService::request_animation_frame(Callback::from(move |_| {
            link2.send_message(Msg::Render)
        }));
        let game = game::create();
        Self {
            link,
            render_task,
            game,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render => {
                self.game.render();
                let link2 = self.link.clone();
                self.render_task =
                    RenderService::request_animation_frame(Callback::from(move |_| {
                        link2.send_message(Msg::Render)
                    }));
                false
            }
            Msg::SelectScenario(scenario_name) => {
                self.game.start(&scenario_name, "");
                false
            }
            Msg::EditorCreated(link) => {
                link.with_editor(|editor| {
                    editor.as_ref().update_options(&make_real_editor_options());
                });
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        fn render_option(name: String) -> Html {
            html! { <option name={name.clone()}>{name}</option> }
        }

        let select_scenario_cb = self.link.callback(|data: ChangeData| match data {
            ChangeData::Select(elem) => Msg::SelectScenario(elem.value()),
            _ => unreachable!(),
        });

        let editor_options = Rc::new(make_editor_options());
        let editor_created_cb = self.link.callback(Msg::EditorCreated);

        html! {
        <>
            <canvas id="glcanvas" tabindex="1"></canvas>
            <div id="editor">
                <CodeEditor options=editor_options on_editor_created=editor_created_cb />
            </div>
            <div id="status"></div>
            <div id="toolbar">
                <div class="toolbar-elem title">{ "Oort" }</div>
                <div class="toolbar-elem right">
                    <select name="scenario" id="scenario" onchange=select_scenario_cb>
                        { for scenario::list().iter().cloned().map(render_option) }
                    </select>
                </div>
                <div class="toolbar-elem right"><a id="doc_link" href="#">{ "Documentation" }</a></div>
                <div class="toolbar-elem right"><a href="http://github.com/rlane/oort3" target="_none">{ "GitHub" }</a></div>
                <div class="toolbar-elem right"><a href="https://trello.com/b/PLQYouu8" target="_none">{ "Trello" }</a></div>
                <div class="toolbar-elem right"><a href="https://discord.gg/vYyu9EhkKH" target="_none">{ "Discord" }</a></div>
                <div id="username" class="toolbar-elem right" title="Your username"></div>
            </div>
            <div id="overlay">
                <div id="splash-overlay" class="inner-overlay"></div>
                <div id="doc-overlay" class="inner-overlay">
                    <h1>{ "Quick Reference" }</h1>
                    { "Press Escape to close. File bugs on " }<a href="http://github.com/rlane/oort3/issues" target="_none">{ "GitHub" }</a>{ "." }<br />

                    <h2>{ "Basics" }</h2>
                    { "Select a scenario from the list in the top-right of the page." }<br/>
                    { "Press Ctrl-Enter in the editor to run the scenario with a new version of your code." }<br/>
                    { "The game calls your <code>tick()</code> function 60 times per second." }
                </div>
                <div id="mission-complete-overlay" class="inner-overlay">
                </div>
            </div>
        </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
