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
mod timer;
mod role_reveal;
mod game_end;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}