use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub current: Vec<Current>,
    pub past: Vec<Past>,
    pub chips: Vec<Chip>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    pub event: i64,
    pub points: i64,
    #[serde(rename = "total_points")]
    pub total_points: i64,
    pub rank: i64,
    #[serde(rename = "rank_sort")]
    pub rank_sort: i64,
    #[serde(rename = "overall_rank")]
    pub overall_rank: i64,
    pub bank: i64,
    pub value: i64,
    #[serde(rename = "event_transfers")]
    pub event_transfers: i64,
    #[serde(rename = "event_transfers_cost")]
    pub event_transfers_cost: i64,
    #[serde(rename = "points_on_bench")]
    pub points_on_bench: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Past {
    #[serde(rename = "season_name")]
    pub season_name: String,
    #[serde(rename = "total_points")]
    pub total_points: i64,
    pub rank: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chip {
    pub name: String,
    pub time: String,
    pub event: i64,
}

#[derive(PartialEq, Properties)]
pub struct CurrentComponentProps {
    pub current: Current,
}

#[function_component(CurrentComponent)]
fn period_component(props: &CurrentComponentProps) -> Html {
    let CurrentComponentProps { current } = props;
    html! {
        <>
        <div class="current">
            <div class="pointbench">{"points on bench"} {current.points_on_bench}</div>
            <div class="transfersandcost"><p>{current.event_transfers}</p><p>{current.event_transfers_cost}</p></div>
            <div class="teamvalue">{current.value}</div>
            <div class="bank">{current.bank}</div>
            <div class="rank">{current.overall_rank}</div>
            <div class="points">{current.points}</div>
            <div class="totpoints">{current.total_points}</div>
        </div>
        </>
    }
}

#[function_component(App)]
fn app_component() -> Html {
    let player = use_state(|| None);
    let player_clone = player.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let player_data = format!("https://fantasy.premierleague.com/api/entry/657266/history/");

        let fetched_data: Root = Request::get(&player_data)
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
        match player_clone.as_ref() {
            Some(f) => f
                .current
                .iter()
                .map(|value| {
                    html! {
                        <CurrentComponent current={value.clone()}/>
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
