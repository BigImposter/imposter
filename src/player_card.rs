use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlayerCardProps {
    pub name: AttrValue,
    pub on_minus_button: Callback<Uuid>,
    pub uuid: Uuid,
}

#[function_component]
pub fn PlayerCard(props: &PlayerCardProps) -> Html {
    let onclick = {
        let on_minus_button = props.on_minus_button.clone();
        let uuid = props.uuid.clone();
        move |_| {
            on_minus_button.emit(uuid);
        }
    };
    html! { 
        <div class="player-card">
            {&props.name}
            <button class="circle-button" {onclick}>{"-"}</button>
        </div> 
    }
}