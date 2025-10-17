use yew::prelude::*;

use crate::{game_settings::GameSettings, player_list::PlayerList};

#[function_component]
pub fn App() -> Html {

    html! {
        <>
        <PlayerList />
        <GameSettings />
        </>
    }
}
