use std::ops::Add;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextPopupProps {
    pub add_text: Callback<AttrValue>,
    pub on_abort: Callback<()>,
}

#[function_component]
pub fn TextInputPopUp(props: &TextPopupProps) -> Html {
    let input_value = use_state(|| AttrValue::from(""));

    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            input_value.set(AttrValue::from(input.value()));
        })

    };
    
    let add_button_clicked = {
        let input_value = input_value.clone();
        let add_text = props.add_text.clone();
        move |_| {
            add_text.emit(AttrValue::from((*input_value).clone()))
        }
    };
    
    let abort_button_clicked = {
        let on_abort = props.on_abort.clone();
        move |_| {
            on_abort.emit(());
        }
    };

    html! {
        <div class="text-input-popup">
            <p>{"Name für neue*r Spieler*In:"}</p>
            <input type="text" oninput={on_input}/>
            <div>
                <button onclick={add_button_clicked}>{ "Hinzufügen" }</button>
                <button onclick={abort_button_clicked}>{ "Abbrechen" }</button>
            </div>
        </div>
    }
}