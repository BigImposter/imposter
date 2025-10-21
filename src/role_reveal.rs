use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Role {
    Guesser(AttrValue),
    Imposter(AttrValue)
}

impl Role {}

#[derive(PartialEq, Properties)]
pub struct RoleRevealProps {
    pub role: Role,
    pub player_name: AttrValue,
    pub on_role_reveal_finished: Callback<()>,
}

#[function_component]
pub fn RoleReveal(props: &RoleRevealProps) -> Html {
    let is_dragging = use_state(|| false);
    let offset_y = use_state(|| 0);
    let start_y = use_state(|| 0);
    let reveal_button = use_state(|| false);
    let role = props.role.clone();

    let onmousedown = {
        let is_dragging = is_dragging.clone();
        let start_y = start_y.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            is_dragging.set(true);
            start_y.set(e.client_y());
        }
    };

    let onmousemove = {
        let start_y = start_y.clone();
        let offset_y = offset_y.clone();
        let is_dragging = is_dragging.clone();
        let reveal_button = reveal_button.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            if *is_dragging {
                let delta = (*start_y - e.client_y()).clamp(-20, 90);
                if delta >= 55 {
                    reveal_button.set(true);
                }
                offset_y.set(delta); // 55 => reveal button
            }
        }
    };

    let onmouseup = {
        let start_y = start_y.clone();
        let offset_y = offset_y.clone();
        let is_dragging = is_dragging.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            start_y.set(0);
            offset_y.set(0);
            is_dragging.set(false);
        }
    };

    let on_next_clicked = {
        let next = props.on_role_reveal_finished.clone();
        let reveal_button = reveal_button.clone();
        move |_| {
            reveal_button.set(false);
            next.emit(());
        }
    };

    html! {
        <div class="role-reveal">
            <div class="player-card">{props.player_name.clone()}</div>
            <div class="role-reveal-container">
                <div class="movable-div"
                    style={format!("bottom: {}px;", *offset_y)}
                    {onmousedown}
                    {onmousemove}
                    {onmouseup}
                ></div>
                <div class="covered-div">
                {
                    match role {
                        Role::Guesser(word) => html! {word},
                        Role::Imposter(tipp) => {
                            html!{
                                <>
                                <div>
                                {"Imposter"}
                                </div>
                                <br />
                                <div>
                                {tipp}
                                </div>
                                </>
                            }
                        },
                    } 
                }
                </div>
            </div>
            if *reveal_button {
                <button onclick={on_next_clicked} id="normal-button">{"Weitergeben"}</button>
            }
        </div>

    }
}