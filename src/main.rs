use std::{
    ops::{Add, Sub},
    rc::Rc,
};

use chrono::{Duration, Utc};
use reqwasm::http::Request;
use yew::prelude::*;
use yew_chart::{
    axis::{Axis, Orientation, Scale},
    linear_axis_scale::LinearScale,
    series::{self, Labeller, Series, Tooltipper, Type},
    time_axis_scale::TimeScale,
};

mod common;

const WIDTH: f32 = 533.0;
const HEIGHT: f32 = 300.0;
const MARGIN: f32 = 50.0;
const TICK_LENGTH: f32 = 10.0;

#[function_component(CurrentComponent)]
fn current_component(props: &common::CurrentComponentProps) -> Html {
    let common::CurrentComponentProps { current } = props;
    html! {
        <div class="current">
            <div class="pointbench">{"points on bench: \t"} {current.points_on_bench}</div>
            <div class="bank">{"money in bank: \t"}{current.bank as f64 / 10.0}</div>
            <div class="teamvalue">{"team value: \t"}{current.value as f64 / 10.0}</div>
            <div class="transfersandcost">{"transfers: "}{current.event_transfers}{"    transfer cost: "}{current.event_transfers_cost}</div>
            <div class="rank">{"rank: \t"}{current.overall_rank}</div>
            <div class="points">{"points: \t"}{current.points}</div>
            <div class="totpoints">{"total points: \t"}{current.total_points}</div>
        </div>
    }
}

#[function_component(ChartComponent)]
fn chart_component() -> Html {
    let end_date = Utc::now();
    let start_date = end_date.sub(Duration::days(4));
    let timespan = start_date..end_date;

    let circle_text_labeller = Rc::from(series::circle_text_label("Label")) as Rc<dyn Labeller>;

    let data_set = Rc::new(vec![
        (start_date.timestamp_millis(), 1.0, None),
        (
            start_date.add(Duration::days(1)).timestamp_millis(),
            4.0,
            None,
        ),
        (
            start_date.add(Duration::days(2)).timestamp_millis(),
            3.0,
            None,
        ),
        (
            start_date.add(Duration::days(3)).timestamp_millis(),
            2.0,
            None,
        ),
        (
            start_date.add(Duration::days(4)).timestamp_millis(),
            5.0,
            Some(circle_text_labeller),
        ),
    ]);

    let h_scale = Rc::new(TimeScale::new(timespan, Duration::days(1))) as Rc<dyn Scale<Scalar = _>>;
    let v_scale = Rc::new(LinearScale::new(0.0..5.0, 1.0)) as Rc<dyn Scale<Scalar = _>>;

    let tooltip = Rc::from(series::y_tooltip()) as Rc<dyn Tooltipper<_, _>>;

    html! {
            <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
                <Series<i64, f32>
                    series_type={Type::Line}
                    name="some-series"
                    data={data_set}
                    horizontal_scale={Rc::clone(&h_scale)}
                    horizontal_scale_step={Duration::days(2).num_milliseconds()}
                    tooltipper={Rc::clone(&tooltip)}
                    vertical_scale={Rc::clone(&v_scale)}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                <Axis<f32>
                    name="some-y-axis"
                    orientation={Orientation::Left}
                    scale={Rc::clone(&v_scale)}
                    x1={MARGIN} y1={MARGIN} xy2={HEIGHT - MARGIN}
                    tick_len={TICK_LENGTH}
                    title={"Some Y thing".to_string()} />

                <Axis<i64>
                    name="some-x-axis"
                    orientation={Orientation::Bottom}
                    scale={Rc::clone(&h_scale)}
                    x1={MARGIN} y1={HEIGHT - MARGIN} xy2={WIDTH - MARGIN}
                    tick_len={TICK_LENGTH}
                    title={"Some X thing".to_string()} />

            </svg>
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

#[function_component(Render)]
pub fn render() -> Html {
    html! {
        <>
        <App />
        <ChartComponent />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Render>();
}
