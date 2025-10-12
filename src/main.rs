mod app;
mod player_list;
mod player_card;
mod player;
mod use_map;
mod text_input_pop_up;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}