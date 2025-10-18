use yew::prelude::*;

#[derive(PartialEq)]
pub enum Role {
    Guesser(AttrValue),
    Imposter(AttrValue)
}

impl Role {}

#[derive(PartialEq, Properties)]
pub struct RoleRevealProps {
    pub role: Role,
    pub on_role_reveal_finished: Callback<()>,
}

#[function_component]
pub fn RoleReveal(props: &RoleRevealProps) -> Html {
    html! {
        <div class="role-reveal-container">
            <div class="covered-div">{"Hi"}</div>
            <div class="movable-div"></div>
        </div>
    }
}