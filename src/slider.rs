use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SliderProps {
    pub title: AttrValue,
    pub min: i32,
    pub max: i32,
    pub value: i32,
    pub step: i32,
    pub status_message: Callback<i32, String>,
    pub on_value_update: Callback<i32>,
}

#[function_component]
pub fn Slider(props: &SliderProps) -> Html {

    let oninput = {
        let on_value_update = props.on_value_update.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            let value = input.value().parse::<i32>().expect("While setting slider use handle: input not a i32 integer.");
            on_value_update.emit(value);
        })
    };

    html! {
        <div class="slider">
            <h3>{ props.title.clone() }</h3>
            <input
                type="range"
                min={props.min.to_string()}
                max={props.max.to_string()}
                step={props.step.to_string()}
                value={props.value.to_string()}
                oninput={oninput}
            />
            <p>{ props.status_message.emit(props.value) }</p>
        </div>
    }
}
