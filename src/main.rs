

pub mod traders;
pub mod wiener;
pub mod draw;

use num_format::{Locale, ToFormattedString};

fn main() {

    // steps represent number of trading days
    // 252 trading days per year
    // 390 minutes in a trading day
    let steps = 252usize;

    let series = wiener::simulate_gbm(steps);


    let xmax = steps as f64;
    let ymax = 250.0;
    let _ = draw::console::chart(series, xmax, ymax);
}

#[allow(dead_code)]
fn fmt(num:u64) -> String {
    num.to_formatted_string(&Locale::fr)
}

#[allow(dead_code)]
fn z_score(x: f64, mean: f64, sd: f64) -> f64 {
    (x - mean) / sd
}










// Gaussian score dynamics
// L overall liquidity or scaling factor
// ϕ probability density function
// Φ cumulative distribution function of the normal distribution
/*
#[allow(dead_code)]
fn static_amm(x: u64, y: u64,) -> u64 {

    0
}
*/

/*


fn curve() {

}

// geometric mean market makers
// uniform AMMs for assets who prices follow geometric Brownian motion
fn uniswap() {

}

fn balancer() {

}
*/
