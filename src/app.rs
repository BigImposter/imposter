use yew::prelude::*;

use crate::{game::{self, Game, GameProps}, game_settings::GameSettings, player_list::PlayerList, pre_game::PreGame, role_reveal::{Role, RoleReveal}, timer::Timer};

#[derive(PartialEq)]
enum AppState {
    Menu,
    Playing,
}

#[function_component]
pub fn App() -> Html {
    let game_state = use_state(|| AppState::Menu);
    let game_props = use_state(|| GameProps::default());

    let on_start_game = {
        let game_state = game_state.clone();
        let game_props_handle = game_props.clone();
        move |game_props: GameProps| {
            game_props_handle.set(game_props);
            game_state.set(AppState::Playing);
        }
    };

    let on_game_finished = {
        let game_state = game_state.clone();
        let game_props = game_props.clone();
        move |_| {
            game_state.set(AppState::Menu);
            game_props.set(GameProps::default());
        }
    };

    html! {
        // <Timer time_in_s={10} on_timer_finished={|()| {}}/>
        <>
        if *game_state == AppState::Menu {
            <PreGame {on_start_game} />
        } else if *game_state == AppState::Playing {
            <Game game_time={game_props.game_time} 
                max_imposters={game_props.max_imposters} 
                min_imposters={game_props.min_imposters} 
                players={game_props.players.clone()}
                {on_game_finished}
            />
        }
        </>
        
        
        // <RoleReveal role={Role::Imposter(AttrValue::from("Hi"))}
        // on_role_reveal_finished={|_| {}}/>
    }
}
