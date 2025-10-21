use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameEndProps {
    pub imposters: Vec<AttrValue>,
    pub word: AttrValue,
    pub on_new_game: Callback<()>,
}

#[function_component]
pub fn GameEnd(props: &GameEndProps) -> Html {
    let is_reveal = use_state(|| false);

    let onclick = {
        let is_reveal = is_reveal.clone();
        move |_| {
            is_reveal.set(true);
        }
    };

    let newgame = {
        let on_new_game = props.on_new_game.clone();
        move |_| {on_new_game.emit(())}
    };
    html! {
            <div class="game-end">
            if *is_reveal {
                <p>{format!("Das Wort war: {}", props.word)}</p>
                <p>{"Der/Die Imposter waren:"}</p>
                {
                    props.imposters.iter().map(|imposter| 
                        html! {
                            <p>{imposter}</p>
                        }
                    ).collect::<Html>()
                }
                <button id="normal-button" onclick={newgame}>{"Neues Spiel"}</button>
            }  else {
                <button id="normal-button" {onclick}>{"Wort und Imposter aufdecken"}</button>
            }
            </div>
    }
}