use std::collections::{HashMap};

use uuid::Uuid;
use yew::prelude::*;
use crate::game::GameProps;
use crate::use_map::{use_map, UseMapHandle};

use crate::player::Player;
use crate::{game_settings::GameSettings, player_list::PlayerList};


const MIN_IMPOSTER_VALUE_INITIAL: i32 = 1;
const MAX_IMPOSTER_VALUE_INITIAL: i32 = 1;
const TIME_VALUE_INITIAL: i32 = 180;

#[derive(Properties, PartialEq)]
pub struct PreGameProps {
    pub on_start_game: Callback<GameProps>,
}

#[function_component]
pub fn PreGame(props: &PreGameProps) -> Html {
    let players: UseMapHandle<Uuid, Player> = use_map(HashMap::from([(Uuid::new_v4(), Player::new(AttrValue::from("Noco")))]));

    let on_add_player = {
        let players = players.clone();
        move |name| {
            players.insert(Uuid::new_v4(), Player::new(name));
        }
    };

    let on_player_delete = {
        let players = players.clone();
        move |uuid: Uuid| {
            players.remove(&uuid);
        }
    };

    let players_length = players.current().len();
    let players_vec = players.to_vec();

    let max_imposter_value = use_state(|| MAX_IMPOSTER_VALUE_INITIAL);
    let min_imposter_value = use_state(|| MIN_IMPOSTER_VALUE_INITIAL);
    let time_value = use_state(|| TIME_VALUE_INITIAL);

    let on_time_value_change = {
        let time_value = time_value.clone();
        move |count| {
            time_value.set(count);
        }
    };

    let on_max_imposter_value_change = {
        let max_imposter_value = max_imposter_value.clone();
        let min_imposter_value = min_imposter_value.clone();
        move |new_value: i32| {
            let new_value = new_value.clamp(*min_imposter_value, players_length as i32);
            max_imposter_value.set(new_value);
        }
    };

    let on_min_imposter_value_change = {
        let min_imposter_value = min_imposter_value.clone();
        let max_imposter_value = max_imposter_value.clone();
        move |new_value: i32| {
            let new_value = new_value.clamp(0, *max_imposter_value);
            min_imposter_value.set(new_value);
        }
    };

    let onclick = {
        let on_start_game = props.on_start_game.clone();
        let max_imposters= max_imposter_value.clone();
        let min_imposters= min_imposter_value.clone();
        let game_time = time_value.clone();
        let players = players.clone();
        move |_: MouseEvent| {
            on_start_game.emit(GameProps {
                max_imposters: *max_imposters, 
                min_imposters: *min_imposters, 
                game_time: *game_time, 
                players: players.to_vec(),
                on_game_finished: Callback::from(|_| {}),
            });
        }
    };

    

    html! {
        <div class="pre-game">
        <PlayerList {on_add_player} {on_player_delete} {players_vec}/>
        <GameSettings player_list_length={players_length as u32} time_value={*time_value} max_imposter_value={*max_imposter_value} min_imposter_value={*min_imposter_value}
        {on_time_value_change} {on_max_imposter_value_change} {on_min_imposter_value_change}/>
        <button {onclick} id="normal-button" align-self="flex-end">{"Start Game"}</button>
        </div>
    }
}