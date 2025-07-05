

pub mod traders;
pub mod wiener;
pub mod draw;
// pub mod ansi;

use std::io;
use std::io::Write;
use num_format::{Locale, ToFormattedString};

// #[derive(Clone)]
pub struct Chart {
    regime: Regime,
    data: Vec<(f64, f64)>,
}

#[derive(Clone)]
pub struct Regime {
    steps: usize, // length of market period
    drift: f64, // 𝜇, risk-free rate ~0.05
    volatility: f64, // σ, standard deviation
}

impl Chart {
    fn reset_data(&mut self) {
        self.data = vec![(0f64, 100f64)];
    }
    fn gen_data(&mut self, duration: usize) {
        let entry = self.last_entry();
        let simulation = wiener::simulate_gbm(self.regime.clone(), duration, entry);

        self.data.append(&mut simulation.clone());
    }
    /*
    fn buy(&mut self) {
        println!("buy");
    }
    fn sell(&mut self) {
        println!("sell");
    }
     */
    fn current_price_rounded(&mut self) -> f64 {
        let (_, value) = self.data.last().unwrap();
        (value * 100.0).round() / 100.0
    }
    fn last_entry(&mut self) -> (f64, f64) {
        let entry = self.data.last().unwrap();
        *entry
    }
    fn points(&mut self) -> usize {
        self.data.len()
    }
}

// Steps represent number of trading days
// 252 trading days per year
// Factors: 1, 2, 3, 4, 6, 7, 9, 12, 14, 18, 21, 28, 36, 42, 63, 84, 126, 252

// 390 minutes in a trading day
// Factors: 1, 2, 3, 5, 6, 10, 13, 15, 26, 30, 39, 65, 78, 130, 195, 390

// For financial applications:
// Daily stock returns might have variance around 0.0001 to 0.01 (std dev 1-10%)
// Annual volatility is often 10-30% (variance 0.01 to 0.09)
fn main() {

    
    let regime = Regime {
        steps: 252usize, 
        drift: 0.5f64,
        volatility: 0.5f64,
    };

    let mut chart = Chart{
        regime,
        data: vec![(0f64, 100f64)],
    };

    chart.gen_data(99usize);

    draw(&chart);

    setup_prompt_area(&mut chart);

    // run prompt
    loop {
        let input = prompt(">");
        
        if !handle_input(&input, &mut chart) {
            break;
        }
    }
}

fn prompt(prompt_text: &str) -> String {

    print!("{}", prompt_text);
    io::stdout().flush().unwrap(); // immediately print carrot
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    print!("\x1B[1A"); // Move cursor up one line
    print!("\x1B[2K"); // Clear entire current line
    
    input.trim().to_string()
}

fn draw(chart: &Chart) {

    let Chart { regime, data, .. } = chart;

    let _ = draw::console::chart(data.clone(), regime.steps);
    
}

/*
\x1B[2J\x1B[1;1H - Clear screen
\x1B[A - Move cursor up one line
\x1B[B - Move cursor down one line
\x1B[2K - Clear entire current line
\x1B[0J - Clear from cursor to end of screen
\x1B[s - Save cursor position
\x1B[u - Restore cursor position
*/

fn setup_prompt_area(chart: &mut Chart) {
    // Reserve space for status area (3 lines) + prompt line

    let price = chart.current_price_rounded();
    let points = chart.points();

    println!("Current price: {:?}", price);
    println!("Data points: {:?}", points);
    println!(".");
    println!();
    print!("\x1B[1A"); 
    io::stdout().flush().unwrap();
}

pub fn handle_input(command: &str, chart: &mut Chart) -> bool {
    // Split command into parts for parsing
    let parts: Vec<&str> = command.split_whitespace().collect();
    
    match parts.as_slice() {
        ["help"] | ["h"] => {
            print!("\x1B[3A");  // Move up to update content
            println!("  run <integer>");
            println!("  buy | sell <amount>");
            println!("  quit, q");
        },
        ["quit"] | ["q"] => {
            println!("Goodbye!");
            return false; // Signal to exit the loop
        },
        ["clear"] | ["c"] => {
            print!("\x1B[2J\x1B[1;1H"); // Clear screen
            chart.reset_data();
            chart.gen_data(99usize);
            draw(chart);
            setup_prompt_area(chart);
        },
        ["run", time_str] | ["r", time_str] => {
            match time_str.parse::<usize>() {
                Ok(time) => {
                    print!("\x1B[2J\x1B[1;1H"); // Clear screen
                    chart.gen_data(time);
                    draw(chart);
                    setup_prompt_area(chart);
                },
                Err(_) => {
                    print!("\x1B[s");   // Save cursor position
                    print!("\x1B[3A");  // Move up to update content
                    print!("\x1B[2K\rInvalid amount for RUN: '{}'", time_str);
                    print!("\x1B[B\x1B[2K\r.");
                    print!("\x1B[B\x1B[2K\r.");
                    print!("\x1B[u");   // Restore cursor position
                    io::stdout().flush().unwrap();
                }
            }
        }
        ["buy", amount_str] => {
            match amount_str.parse::<i32>() {
                Ok(amount) => {
                    print!("\x1B[s");   // Save cursor position
                    print!("\x1B[3A");  // Move up to update content
                    print!("\x1B[2K\rBought {} units", amount);
                    print!("\x1B[B\x1B[2K\r.");
                    print!("\x1B[B\x1B[2K\r.");
                    print!("\x1B[u");   // Restore cursor position
                    io::stdout().flush().unwrap();
                    
                    // Add your buy logic here
                    // chart.buy(amount);
                },
                Err(_) => {
                    print!("\x1B[s");   // Save cursor position
                    print!("\x1B[3A");  // Move up to update content
                    print!("\x1B[2K\rInvalid amount for BUY: '{}'", amount_str);
                    print!("\x1B[B\x1B[2K\r.");
                    print!("\x1B[B\x1B[2K\r.");
                    print!("\x1B[u");   // Restore cursor position
                    io::stdout().flush().unwrap();
                }
            }
        },
        ["sell", amount_str] => {
            match amount_str.parse::<i32>() {
                Ok(amount) => {
                    print!("\x1B[s");   // Save cursor position
                    print!("\x1B[3A");  // Move up to update content
                    print!("\x1B[2K\rSold {} units", amount);
                    print!("\x1B[B\x1B[2K\r.");
                    print!("\x1B[B\x1B[2K\r.");
                    print!("\x1B[u");   // Restore cursor position
                    io::stdout().flush().unwrap();
                    
                    // Add your sell logic here
                    // chart.sell(amount);
                },
                Err(_) => {
                    print!("\x1B[s");   // Save cursor position
                    print!("\x1B[3A");  // Move up to update content
                    print!("\x1B[2K\rInvalid amount for SELL: '{}'", amount_str);
                    print!("\x1B[B\x1B[2K\r.");
                    print!("\x1B[B\x1B[2K\r.");
                    print!("\x1B[u");   // Restore cursor position
                    io::stdout().flush().unwrap();
                }
            }
        },
        [] => {
            // Do nothing for empty input
        },
        _ => {
            print!("\x1B[s");   // Save cursor position
            print!("\x1B[3A");  // Move up to update content
            print!("\x1B[2K\rUnknown command: '{}'. Type 'help' for available commands.", command);
            print!("\x1B[B\x1B[2K\r.");
            print!("\x1B[B\x1B[2K\r.");
            print!("\x1B[u");   // Restore cursor position
            io::stdout().flush().unwrap();
        }
    }
    true // Continue the loop
}

#[allow(dead_code)]
fn fmt(num:u64) -> String {
    num.to_formatted_string(&Locale::fr)
}

