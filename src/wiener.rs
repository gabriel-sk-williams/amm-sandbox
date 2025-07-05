
use std::f64::consts::E;
use std::f64::consts::PI;
use rand_distr::{Normal, Distribution};
use crate::Regime;


// simulate <duration> steps from last timestamp
pub fn simulate_gbm(regime: Regime, duration: usize, last_entry: (f64, f64)) -> Vec<(f64, f64)> {

    let Regime { steps, drift, volatility } = regime;
    let (last_timestamp, last_price) = last_entry;
    
    let dt = 1.0 / steps as f64;
    let normal = Normal::new(0.0, dt.sqrt()).unwrap();
    
    let mut current_price = last_price;

    let mut path = Vec::with_capacity(duration);

    let next_timestamp = last_timestamp as usize + 1;
    let endpoint = duration + next_timestamp;

    for timestamp in next_timestamp..endpoint {
        let dw = normal.sample(&mut rand::rng());
        let next_price = geometric_brownian_motion(current_price, drift, volatility, dt, dw);
        current_price = next_price;
        path.push((timestamp as f64, next_price));
    }

    path
}

pub fn geometric_brownian_motion(
    prev_price: f64,
    mu: f64, // 𝜇 drift
    sigma: f64, // σ volatility
    dt:f64,
    dw: f64,
) -> f64 {
    prev_price * ( (mu - 0.5 * sigma.powi(2)) * dt + sigma * dw ).exp()
}

#[allow(dead_code)]
pub fn wiener_process(total_time: f64, steps: usize) -> Vec<f64> {
    let dt = total_time / steps as f64;
    let normal = Normal::new(0.0, dt.sqrt()).unwrap();

    println!("{:?} / {:?} = {:?}", total_time, steps, dt);
    println!("{:?}{:?}", dt, normal);

    let mut w_path = Vec::with_capacity(steps + 1);
    w_path.push(0.0);

    for i in 0..steps {
        let increment = normal.sample(&mut rand::rng());
        let next = w_path[i] + increment;
        w_path.push(next);
    }

    w_path
}

//wiener::wiener_curve(19, 1);
#[allow(dead_code)]
pub fn wiener_curve(range: i64, variance: u64) {
    let lower_bound = 0 - range;
    let upper_bound = 0 + range;

    for x in lower_bound..upper_bound+1 {
        let val = wiener_density(x as f64, variance as f64);
        println!("{}", val);
    }
}

// X∼N(μ,σ^2)
// The random variable 𝑋 is distributed as (~) a normal distribution 
// with mean 𝜇 variance 𝜎^2

// variance is standard deviation squared
// max density 0.3989 at 1 variance
// max density 0.0266 at 225 variance
#[allow(dead_code)]
pub fn wiener_density(x: f64, variance: f64) -> f64 {

    let exo = x.powf(2.0) / (2.0 * variance);
    let e_raised:f64 = E.powf(-exo);

    let normalizing_constant = 2.0 * PI * variance;
    let denominator = normalizing_constant.sqrt();

    1.0 / denominator * e_raised
}

// 0.0 -> 0.39894
#[allow(dead_code)]
pub fn probability_density(x:f64) -> f64 {
    let exo = x.powf(2.0) / 2.0;
    let e_raised:f64 = E.powf(-exo);

    let normalizing_constant = 2.0 * PI;
    let denominator = normalizing_constant.sqrt();

    1.0 / denominator * e_raised
}

#[allow(dead_code)]
fn z_score(x: f64, mean: f64, sd: f64) -> f64 {
    (x - mean) / sd
}