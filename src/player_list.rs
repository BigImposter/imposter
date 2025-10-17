use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

use yew::prelude::*;

use crate::player_card::PlayerCard;
use crate::player::Player;
use crate::text_input_pop_up::TextInputPopUp;

#[derive(PartialEq, Properties)]
pub struct PlayerListProps {
    pub on_player_delete: Callback<Uuid>,
    pub on_add_player: Callback<AttrValue>,
    pub players_vec: Vec<(Uuid, Player)>,
}

#[function_component]
pub fn PlayerList(props: &PlayerListProps) -> Html {
    let is_open_name_input = use_state(|| false);

    let add_player_button_clicked = {
        let is_open_name_input = is_open_name_input.clone();
        move |_| {
            is_open_name_input.set(true);
        }
    };

    let add_player = {
        let is_open_name_input = is_open_name_input.clone();
        let on_add_player = props.on_add_player.clone();
        move | name: AttrValue | {
            on_add_player.emit(name);
            is_open_name_input.set(false);
        }
    };

    let on_add_player_abort = {
        let is_open_name_input = is_open_name_input.clone();
        move |_| {
            is_open_name_input.set(false);
        }
    };

    html! {
        <>
        <div id="player-list">
            {
                props.players_vec.iter().map(|(key, player)| {
                    html! { 
                        <PlayerCard name={player.name.clone()} on_minus_button={props.on_player_delete.clone()} uuid={*key}/> 
                    }
                }).collect::<Html>()
            }
        <div class="player-card" id="add-new-player" onclick={add_player_button_clicked}>
            <p>{ "+" }</p>
        </div>
        </div>
        if *is_open_name_input {
            <TextInputPopUp add_text={add_player} on_abort={on_add_player_abort}/>
        }
        </>
    }
}
