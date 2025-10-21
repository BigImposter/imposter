use web_sys::console::time;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TimerProps {
    pub on_timer_finished: Callback<()>,
    pub time_in_s: u32,
    pub starting_player_name: AttrValue,
}

const ONE_SECOND: u32 = 1_000;

#[function_component]
pub fn Timer(props: &TimerProps) -> Html {
    let time_start = use_state(|| js_sys::Date::new_0());
    let time_until = use_state(|| 0);
    
    let _timout_handle= {
        let time_start = time_start.clone();
        let timer_finished = props.on_timer_finished.clone();
        let time_until = time_until.clone();
        let time_in_s = props.time_in_s.clone();
        gloo::timers::callback::Interval::new(ONE_SECOND, move || {
            let delta = js_sys::Date::new_0().get_seconds() - (*time_start).get_seconds();
            time_until.set(time_in_s - delta);
            if delta <= 0 {
                timer_finished.emit(());
            }
        })
        .forget()
    };
    let timer_finished = props.on_timer_finished.clone();
    html! {
        <div class="timer">
            <p>{format!("{} fängt an!", props.starting_player_name)}</p>
            <div>{format!("{:02}:{:02}", *time_until / 60, *time_until % 60)}</div>
            <button id="normal-button" 
                onclick={
                    move |_| {
                        timer_finished.emit(())
                    }
                }
            >
                {"Abbrechen"}
            </button>
        </div>
    }
}