use yew::prelude::*;
use chrono::{Datelike, Duration, NaiveDate};

fn monday_of_week(year: i32, week: i32) -> NaiveDate {
    // ISO week: week starts on Monday
    let jan_4 = NaiveDate::from_ymd_opt(year, 1, 4).unwrap();
    let week1_monday = jan_4 - Duration::days(jan_4.weekday().num_days_from_monday() as i64);
    week1_monday + Duration::weeks((week - 1) as i64)
}


#[derive(Properties, PartialEq)]
pub struct CalendarProps {
    pub current_week: i32,
    pub year: i32,
    pub on_previous: Callback<()>,
    pub on_next: Callback<()>,
}

#[function_component(Calendar)]
pub fn calendar(props: &CalendarProps) -> Html {
    let on_prev_click = {
        let cb = props.on_previous.clone();
        Callback::from(move |_: web_sys::MouseEvent| cb.emit(()))
    };
    let on_next_click = {
        let cb = props.on_next.clone();
        Callback::from(move |_: web_sys::MouseEvent| cb.emit(()))
    };

    let monday = monday_of_week(props.year, props.current_week);
    let friday = monday + chrono::Duration::days(4);

    html! {
        <div id="calendar-container">
            <button id="calendar-previous-button" onclick={on_prev_click}>{ "<" }</button>
            <div id="current-week">
                { format!("{} - {}", monday.format("%d.%m.%Y"), friday.format("%d.%m.%Y")) }
            </div>
            <button id="calendar-next-button" onclick={on_next_click}>{ ">" }</button>
        </div>
    }
}

