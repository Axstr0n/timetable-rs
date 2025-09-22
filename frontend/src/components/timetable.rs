use yew::prelude::*;
use chrono::{Datelike, Duration, NaiveDate, Weekday};

use crate::components::entry_box::{EntryBox, EntryBoxProps};



#[derive(Properties, PartialEq)]
pub struct TimetableProps {
    pub current_week: i32,
    pub year: i32,
    pub entries: Vec<EntryBoxProps>,
    pub work_free_days: Vec<NaiveDate>,
}


#[function_component(Timetable)]
pub fn timetable(props: &TimetableProps) -> Html {
    let days = [
        ("Ponedeljek", Weekday::Mon),
        ("Torek", Weekday::Tue),
        ("Sreda", Weekday::Wed),
        ("Četrtek", Weekday::Thu),
        ("Petek", Weekday::Fri),
    ];

    // Monday of the ISO week
    let monday = NaiveDate::from_isoywd_opt(props.year, props.current_week as u32, Weekday::Mon)
        .expect("valid ISO week date");

    // Hours 7..19
    let hours: Vec<u8> = (7..=19).collect();
    let positions_count = hours.len() * 2;
    let mut hour_nodes: Vec<Html> = Vec::with_capacity(positions_count);
    for (j, h) in hours.iter().enumerate() {
        let idx_hour = 2 * j;
        let bottom_hour = 100.0 - (idx_hour as f64) * (100.0 / positions_count as f64);
        hour_nodes.push(html! { <div class="hour" style={format!("bottom: {bottom_hour:.2}%")}><span>{ h }</span></div> });

        let idx_dash = 2 * j + 1;
        let bottom_dash = 100.0 - (idx_dash as f64) * (100.0 / positions_count as f64);
        hour_nodes.push(html! { <div class="hour-dashed" style={format!("bottom: {bottom_dash:.2}%")}></div> });
    }


    html! {
        <div id="timetable-wrapper">
            <div id="timetable">
                <div id="days">
                    { for days.iter().enumerate().map(|(i, (label, weekday))| {
                        let left = format!("{:.2}%", i as f64 * 20.0);
                        let style = format!("--day-width: 20.00%; left: {left};");

                        // compute the actual date for this day
                        let date = monday + Duration::days(weekday.num_days_from_monday() as i64);

                        // check if this date is in work-free days
                        let is_work_free = props.work_free_days.contains(&date);

                        // conditional class
                        let class = if is_work_free { "day day-work-free" } else { "day" };

                        // optional label for work-free day
                        let label_text = format!("{} - {:02}.{:02}.{}", label, date.day(), date.month(), date.year());

                        html! {
                            <div {class} style={style}>
                                { label_text }
                            </div>
                        }
                    }) }
                </div>
                <div id="hours">{ for hour_nodes }</div>
                <div id="entries">
                    { for props.entries.iter().map(|entry| html! {
                        <EntryBox
                            subject_id={entry.subject_id}
                            subject_name={entry.subject_name.clone()}
                            subject_abbr={entry.subject_abbr.clone()}
                            entry_type={entry.entry_type.clone()}
                            teachers={entry.teachers.clone()}
                            classroom={entry.classroom.clone()}
                            day={entry.day}
                            date={entry.date}
                            start_time={entry.start_time}
                            duration={entry.duration}
                            offset_x_multiplier={entry.offset_x_multiplier}
                            total_columns={entry.total_columns}
                            color={entry.color.clone()}
                        />
                    }) }
                </div>
                <div class="author-mark">
                    <div class="made-by">{ "made by" }</div>
                    <div class="author">{ "Axstr0n" }</div>
                </div>
            </div>
        </div>
    }
}

