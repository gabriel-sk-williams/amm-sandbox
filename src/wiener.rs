
use std::f64::consts::E;
use std::f64::consts::PI;
use rand_distr::{Normal, Distribution};

// For financial applications:
// Daily stock returns might have variance around 0.0001 to 0.01 (std dev 1-10%)
// Annual volatility is often 10-30% (variance 0.01 to 0.09)
pub fn simulate_gbm(steps: usize, drift: f64, volatility: f64) -> Vec<(f64, f64)> {

    let dt = 1.0 / steps as f64;
    
    let mut current_price = 100.0;

    let mut path = Vec::with_capacity(steps + 1);
    path.push((0.0, current_price));

    let normal = Normal::new(0.0, dt.sqrt()).unwrap();
    
    for time in 0..steps {
        let dw = normal.sample(&mut rand::rng());
        let price = geometric_brownian_motion(current_price, drift, volatility, dt, dw);
        current_price = price;
        path.push((time as f64, price));
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
    let normal = Normal::new(0.0, dt.sqrt()).unwrap(); // N(0, sqrt(dt))

    println!("{:?} / {:?} = {:?}", total_time, steps, dt);
    println!("{:?}{:?}", dt, normal);

    let mut w_path = Vec::with_capacity(steps + 1);
    w_path.push(0.0); // W(0) = 0

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