use yew::prelude::*;

use crate::{game::{self, Game, GameProps}, game_settings::GameSettings, player_list::PlayerList, pre_game::PreGame};

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
}

#[function_component]
pub fn App() -> Html {
    let game_state = use_state(|| GameState::Menu);
    let game_props = use_state(|| GameProps::default());

    let on_start_game = {
        let game_state = game_state.clone();
        let game_props_handle = game_props.clone();
        move |game_props: GameProps| {
            game_props_handle.set(game_props);
            game_state.set(GameState::Playing);
        }
    };

    html! {
        <>
        if *game_state == GameState::Menu {
            <PreGame {on_start_game} />
        } else if *game_state == GameState::Playing {
            <Game game_time={game_props.game_time} 
            max_imposters={game_props.max_imposters} 
            min_imposters={game_props.min_imposters} 
            players={game_props.players.clone()}/>
        }
        </>
    }
}
