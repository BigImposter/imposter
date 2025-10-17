use yew::prelude::*;

use crate::{game::{self, Game}, game_settings::GameSettings, player_list::PlayerList, pre_game::PreGame};

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
}

#[function_component]
pub fn App() -> Html {
    let game_state = use_state(|| GameState::Menu);

    let on_start_game = {
        let game_state = game_state.clone();
        move |_| {
            game_state.set(GameState::Playing);
        }
    };

    html! {
        <>
        if *game_state == GameState::Menu {
            <PreGame {on_start_game} />
        } else if *game_state == GameState::Playing {
            <Game />
        }
        </>
    }
}
