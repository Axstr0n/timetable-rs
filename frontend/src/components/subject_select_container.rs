use shared::definitions::{Subject, SubjectId};
use yew::prelude::*;
use std::collections::HashMap;

use crate::{logic::visibility::SubjectsVisibilityMap, utils::Color};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupType {
    Tutorial,
    Lab,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SubjectContainerProps {
    pub subjects: Vec<Subject>,
    pub subjects_visibility: SubjectsVisibilityMap,
    pub subject_colors: HashMap<SubjectId, Color>,
    pub on_toggle_subject: Callback<SubjectId>,
    pub on_toggle_group: Callback<(SubjectId, GroupType, u32)>, 
}


#[function_component(SubjectContainer)]
pub fn subject_container(props: &SubjectContainerProps) -> Html {
    html! {
        <div id="subjects-container">
            { for props.subjects.iter().map(|subject| {
                let Some(visibility) = props.subjects_visibility.get(&subject.id) else {
                    return html! {};
                };
                if !visibility.can_show {
                    return html! {};
                }

                let bg_color = props.subject_colors
                    .get(&subject.id)
                    .cloned()
                    .unwrap();

                // Whole subject container
                html! {
                    <div class="subject-select-container" style={format!("background-color: {}", bg_color.css())}>
                        <div class="title">{ &subject.name }</div>

                        // Tutorial groups
                        <div class="exercises-container">
                            <div class="label">{ "VP" }</div>
                            <div class="buttons">
                                { for visibility.tutorial_groups.iter().map(|(group_id, active)| {
                                    let id = subject.id;
                                    let gid = *group_id;
                                    let on_toggle = props.on_toggle_group.clone();
                                    let on_click = Callback::from(move |_| on_toggle.emit((id, GroupType::Tutorial, gid)));
                                    html! {
                                        <button
                                            id={format!("button-toggle-{}-VP{}", subject.abbr, group_id)}
                                            class={classes!("exercise-button", if *active { "active-button" } else { "" })}
                                            onclick={on_click}
                                        >
                                            { format!("S{group_id}") }
                                        </button>
                                    }
                                }) }
                            </div>
                        </div>

                        // Lab groups
                        <div class="exercises-container">
                            <div class="label">{ "VL" }</div>
                            <div class="buttons">
                                { for visibility.lab_groups.iter().map(|(group_id, active)| {
                                    let id = subject.id;
                                    let gid = *group_id;
                                    let on_toggle = props.on_toggle_group.clone();
                                    let on_click = Callback::from(move |_| on_toggle.emit((id, GroupType::Lab, gid)));
                                    html! {
                                        <button
                                            id={format!("button-toggle-{}-VL{}", subject.abbr, group_id)}
                                            class={classes!("exercise-button", if *active { "active-button" } else { "" })}
                                            onclick={on_click}
                                        >
                                            { format!("S{group_id}") }
                                        </button>
                                    }
                                }) }
                            </div>
                        </div>
                    </div>
                }
            }) }
        </div>
    }
}
