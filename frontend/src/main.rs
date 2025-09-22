
pub mod components;
pub mod utils;
pub mod logic;
pub mod database;

use components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
