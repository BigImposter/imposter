use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameProps {

}

#[function_component]
pub fn Game(props: &GameProps) -> Html {
    html! {
        {"Spiel"}
    }
}