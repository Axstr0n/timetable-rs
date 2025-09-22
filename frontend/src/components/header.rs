
use js_sys::Object;
use shared::definitions::{MainProgramId, Semester};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlAnchorElement, HtmlCanvasElement, HtmlElement, HtmlSelectElement};
use yew::prelude::*;

use crate::logic::bindings::html2canvas;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub main_program_options: Vec<(MainProgramId, String)>,
    pub selected_main_program: MainProgramId, // currently selected program ID
    pub on_main_program_change: Callback<MainProgramId>,

    pub semester_options: Vec<(Semester, String)>,
    pub selected_semester: Semester, // currently selected program ID
    pub on_semester_change: Callback<Semester>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {

    let on_main_program_change = {
        let callback = props.on_main_program_change.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let select = target.dyn_into::<HtmlSelectElement>().unwrap();
            let value = select.value();
            
            // Parse the MainProgramId from the string value
            if let Ok(program_id) = value.parse::<u32>() {
                callback.emit(MainProgramId(program_id));
            }
        })
    };
    let on_semester_change = {
        let callback = props.on_semester_change.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let select = target.dyn_into::<HtmlSelectElement>().unwrap();
            let value = select.value();
            
            // Parse the Semester from the string value
            if let Ok(semester_id) = value.parse::<u8>() {
                callback.emit(Semester(semester_id));
            }
        })
    };

    let onclick = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let document = web_sys::window().unwrap().document().unwrap();
            let content = document
                .get_element_by_id("timetable-wrapper")
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap();

            // temporarily set width
            content.style().set_property("width", "1100px").unwrap();

            // call html2canvas
            let options = Object::new();
            let promise = html2canvas(&content, &options.into());
            let canvas = JsFuture::from(promise).await.unwrap();
            let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();

            // get dataUrl
            let data_url = canvas.to_data_url().unwrap();

            // create download link
            let link: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();
            link.set_href(&data_url);
            link.set_download("timetable.png");
            link.click();

            // reset width
            content.style().set_property("width", "100%").unwrap();
        });
    });
    
    html! {
        <div class="header">
            <a href="https://axstr0n.github.io/Portfolio/" target="_blank" class="logo-container">
                // <img src="static/logo.png" alt="logo" />
                { "🐵" }
            </a>
            <div class="title">{ "TIMETABLE" }</div>
            <div class="download-save-discard-select">
                <button id="download-button" onclick={onclick} title="Download timetable">{ "📸" }</button>
                // <button id="discard-button" title="Reset">{ "🗑️" }</button>
                <select
                    id="main-program-select"
                    onchange={on_main_program_change}
                >
                    { for props.main_program_options.iter().map(|(program_id, name)| html! {
                        <option 
                            value={program_id.0.to_string()}
                            selected={*program_id == props.selected_main_program}
                        >
                            { name }
                        </option>
                    }) }
                </select>
                <select
                    id="semester-select"
                    onchange={on_semester_change}
                >
                    { for props.semester_options.iter().map(|(semester, name)| html! {
                        <option
                            value={semester.0.to_string()}
                            selected={*semester == props.selected_semester}
                        >
                            { name }
                        </option>
                    }) }
                </select>
            </div>
        </div>
    }
}
