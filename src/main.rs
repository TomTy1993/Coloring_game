mod logic;
mod ui_components;

use ui_components::main_game_component::MainGameComponent;

fn main() {
    yew::start_app::<MainGameComponent>();
}
