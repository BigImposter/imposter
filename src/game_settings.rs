use yew::prelude::*;

use crate::slider::Slider;

#[derive(PartialEq, Properties)]
pub struct GameSettingsProps {
    pub player_list_length: u32,
    pub max_imposter_value: i32,
    pub min_imposter_value: i32,
    pub time_value: i32,
    pub on_max_imposter_value_change: Callback<i32>,
    pub on_min_imposter_value_change: Callback<i32>,
    pub on_time_value_change: Callback<i32>,
}

#[function_component]
pub fn GameSettings(props: &GameSettingsProps) -> Html {
    html! {
        <div id="game-settings">
            <Slider title={"Maximale Imposterzahl:"} 
                    status_message={|x| {format!{"Max Imposter: {}", x}}}
                    on_value_update={props.on_max_imposter_value_change.clone()}
                    step={1} min={0} max={props.player_list_length as i32} value={props.max_imposter_value}
            />
            <Slider title={"Minimale Imposterzahl:"}
                    on_value_update={props.on_min_imposter_value_change.clone()}
                    status_message={|x| {format!{"Min Imposter: {}", x}}}
                    step={1} min={0} max={props.player_list_length as i32} value={props.min_imposter_value}

            />
            <Slider min={60} max={480} step={60} value={props.time_value}
                    title={"Ratezeit:"}
                    on_value_update={props.on_time_value_change.clone()}
                    status_message={|x| {format!("{}", x / 60)}}
            />
        </div>
    }
}