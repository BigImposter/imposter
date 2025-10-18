use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TimerProps {
    pub on_timer_finished: Callback<()>,
    pub time_in_s: u32,
}

const ONE_SECOND: u32 = 1_000;

#[function_component]
pub fn Timer(props: &TimerProps) -> Html {
    let ms_current = use_state(|| props.time_in_s * 1000);
    
    let _timout_handle= {
        let ms_current = ms_current.clone();
        let timer_finished = props.on_timer_finished.clone();
        gloo::timers::callback::Interval::new(ONE_SECOND, move || {
            ms_current.set(*ms_current - ONE_SECOND);
            if *ms_current <= 0 {
                timer_finished.emit(());
            }
        })
        .forget()
    };
    let timer_finished = props.on_timer_finished.clone();
    html! {
        <div class="timer">
            <div>{*ms_current / ONE_SECOND}</div>
            <button onclick={move |_| {timer_finished.emit(())}}>{"Abbrechen"}</button>
        </div>
    }
}