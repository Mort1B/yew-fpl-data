use reqwasm::http::Request;
use yew::prelude::*;

mod common;

#[function_component(CurrentComponent)]
fn current_component(props: &common::CurrentComponentProps) -> Html {
    let common::CurrentComponentProps { current } = props;
    html! {
        <div class="current">
            <div class="pointbench">{"points on bench: \t"} {current.points_on_bench}</div>
            <div class="bank">{"money in bank: \t"}{current.bank}</div>
            <div class="teamvalue">{"team value: \t"}{current.value as f64 / 10.0}</div>
            <div class="transfersandcost">{"transfers: "}{current.event_transfers}{"    transfer cost: "}{current.event_transfers_cost}</div>
            <div class="rank">{"rank: \t"}{current.overall_rank}</div>
            <div class="points">{"points: \t"}{current.points}</div>
            <div class="totpoints">{"total points: \t"}{current.total_points}</div>
        </div>
    }
}

#[function_component(App)]
fn app_component() -> Html {
    let player = use_state(|| None);
    let player_clone = player.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let player_data = format!("https://fantasy.premierleague.com/api/entry/657266/history/");

        let fetched_data: common::Root = Request::get(&player_data)
            // .mode(reqwasm::http::RequestMode::SameOrigin)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        player.set(Some(fetched_data));
    });

    html!({
        match (*player_clone).as_ref() {
            Some(f) => f
                .current
                .iter()
                .map(|value| {
                    html! {
                        <>
                        <div class="asd">
                        <CurrentComponent current={value.clone()}/>
                        </div>
                        </>
                    }
                })
                .collect(),
            None => html! {
                <>
                    <div> {String::from("error")}</div>
                </>
            },
        }
    })
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
