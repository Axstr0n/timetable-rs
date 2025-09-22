use std::collections::HashMap;

use shared::definitions::SubjectId;
use yew::prelude::*;

use crate::{logic::visibility::SubjectsVisibilityMap, utils::Color};


#[derive(Properties, PartialEq)]
pub struct SubjectAbbrProps {
    pub subjects_visibility: SubjectsVisibilityMap, // SubjectId -> SubjectVisibility
    pub subjects: Vec<(SubjectId, String)>,
    pub subject_colors: HashMap<SubjectId, Color>,
    pub on_toggle: Callback<SubjectId>,
}

#[function_component(SubjectAbbrContainer)]
pub fn subject_abbr_container(props: &SubjectAbbrProps) -> Html {

    html! {
        <div id="subjects-abbreviation-container">
            { for props.subjects.iter().map(|(subject_id, subject_abbr)| {
                let visibility = &props.subjects_visibility[subject_id];
                let color = props.subject_colors.get(subject_id).unwrap();

                let on_click: Callback<MouseEvent> = {
                    let subject_id = *subject_id;
                    let on_toggle = props.on_toggle.clone();
                    Callback::from(move |_: MouseEvent| on_toggle.emit(subject_id))
                };

                html! {
                    <button
                        id={format!("button-toggle-{subject_abbr}")}
                        class={classes!(
                            "abbreviation-button",
                            if visibility.can_show { "active-button" } else { "" }
                        )}
                        onclick={on_click}
                        style={format!("background-color: {}", color.css())}
                    >
                        { &subject_abbr }
                    </button>
                }
            }) }
        </div>
    }
}

