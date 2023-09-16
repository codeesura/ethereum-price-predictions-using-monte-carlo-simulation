#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate plotters;

use rand::distributions::{Distribution, Uniform};
use std::collections::VecDeque;
use plotters::prelude::*;

#[derive(Deserialize)]
struct EthereumData {
    prices: Vec<Vec<f64>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.coingecko.com/api/v3/coins/ethereum/market_chart?vs_currency=usd&days=1095&interval=daily";
    let response: EthereumData = reqwest::get(url).await?.json().await?;
    let past_prices: VecDeque<f64> = response.prices.iter().map(|x| x[1]).collect();

    let mut returns = Vec::new();
    for i in 1..past_prices.len() {
        let daily_return = (past_prices[i] - past_prices[i - 1]) / past_prices[i - 1];
        returns.push(daily_return);
    }

    let simulations = 1000;
    let days = 365;
    let mut all_simulated_prices: Vec<VecDeque<f64>> = Vec::new();

    for _ in 0..simulations {
        let mut simulated_prices = VecDeque::from(vec![*past_prices.back().unwrap()]);
        for _ in 0..days {
            let last_price = *simulated_prices.back().unwrap();
            let random_index = Uniform::from(0..returns.len()).sample(&mut rand::thread_rng());
            let new_price = last_price * (1.0 + returns[random_index]);
            simulated_prices.push_back(new_price);
        }
        all_simulated_prices.push(simulated_prices);
    }

    let starting_price = *past_prices.front().unwrap();
    let y_axis_start = starting_price * 0.9;

    let root = BitMapBackend::new("ethereum_predictions.png", (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let median_prices: Vec<f64> = (0..365).map(|day| {
        let mut day_prices: Vec<f64> = all_simulated_prices.iter().map(|prices| prices[day]).collect();
        day_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        day_prices[day_prices.len() / 2]
    }).collect();

    let max_median_price = median_prices.iter().cloned().fold(0./0., f64::max);
    let mut chart = ChartBuilder::on(&root)
        .caption("Ethereum Price Predictions", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0..365i32, y_axis_start..max_median_price * 1.1)?;

    chart.configure_mesh().y_labels(10).draw()?;

    chart.draw_series(LineSeries::new(median_prices.iter().enumerate().map(|(day, &price)| (day as i32, price)), &RED))?;

    Ok(())
}
