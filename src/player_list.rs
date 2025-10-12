use std::collections::HashMap;

use uuid::Uuid;

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::player_card::PlayerCard;
use crate::player::Player;
use crate::text_input_pop_up::TextInputPopUp;

#[function_component]
pub fn PlayerList() -> Html {
    let players = use_map(HashMap::from([
        (Uuid::new_v4(), Player::new(AttrValue::from("Brady Haran"))),
        (Uuid::new_v4(), Player::new(AttrValue::from("Tom Scott"))),
        (Uuid::new_v4(), Player::new(AttrValue::from("CPG Grey"))),
    ]));
    let is_open_name_input = use_state(|| false);

    let on_player_delete: Callback<Uuid> = {
        let players = players.clone();
        Callback::from(move |uu_id| {
            players.remove(&uu_id);
        })
    };

    let add_player_button_clicked = {
        let is_open_name_input = is_open_name_input.clone();
        move |_| {
            is_open_name_input.set(true);
        }
    };

    let add_player = {
        let players = players.clone();
        let is_open_name_input = is_open_name_input.clone();
        move | name: AttrValue | {
            players.insert(Uuid::new_v4(), Player::new(name));
            is_open_name_input.set(false);
        }
    };

    html! {
        <>
        <div id="player-list">
        {
            players.current().iter().map(|(&key, player)| {
                html! { 
                    <PlayerCard name={player.name.clone()} on_minus_button={on_player_delete.clone()} uuid={key}/> 
                }
            }).collect::<Html>()
        }
        <div class="player-card" id="add-new-player" onclick={add_player_button_clicked}>
            <p>{ "+" }</p>
        </div>
        </div>
        <TextInputPopUp is_open={*is_open_name_input} add_text={add_player}/>
        </>
    }
}
