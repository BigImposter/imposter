use uuid::Uuid;
use yew::prelude::*;

use crate::player::Player;

#[derive(Properties, PartialEq, Default)]
pub struct GameProps {
    pub min_imposters: i32,
    pub max_imposters: i32,
    pub game_time: i32,
    pub players: Vec<(Uuid, Player)>
}

#[function_component]
pub fn Game(props: &GameProps) -> Html {
    html! {
        <>
        {"Min imposters:"}{props.min_imposters}
        <br />
        {"Max imposters:"}{props.max_imposters}
        <br />
        {"Game Time:"}{props.game_time}
        <br />
        {"Player Count:"}{props.players.len()}
        </>
    }
}