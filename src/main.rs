mod app;
mod player_list;
mod player_card;
mod player;
mod use_map;
mod text_input_pop_up;
mod game_settings;
mod slider;
mod pre_game;
mod game;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}