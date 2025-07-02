

pub mod traders;
pub mod wiener;
pub mod draw;
// pub mod ansi;

use std::io;
use std::io::Write;
use num_format::{Locale, ToFormattedString};

// #[derive(Clone)]
pub struct Chart {
    steps: usize,
    drift: f64, // 𝜇, risk-free rate ~0.05
    volatility: f64, // σ, standard deviation
    xmax: f64,
    ymax: f64,
    data: Vec<(f64, f64)>,
}

impl Chart {
    fn gen_data(&mut self) {
        self.data = wiener::simulate_gbm(self.steps, self.drift, self.volatility);
    }
}

// Steps represent number of trading days
// 252 trading days per year
// Factors: 1, 2, 3, 4, 6, 7, 9, 12, 14, 18, 21, 28, 36, 42, 63, 84, 126, 252

// 390 minutes in a trading day
// Factors: 1, 2, 3, 5, 6, 10, 13, 15, 26, 30, 39, 65, 78, 130, 195, 390

fn main() {

    let steps = 252usize;

    let mut chart = Chart{
        steps,
        drift: 0.5f64,
        volatility: 0.5f64,
        xmax: steps as f64,
        ymax: 250.0,
        data: vec![],
    };

    chart.gen_data();

    draw(&chart);

    setup_prompt_area();

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

    let Chart { xmax, ymax, data, .. } = chart;

    let _ = draw::console::chart(data.clone(), *xmax, *ymax);
    
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

fn setup_prompt_area() {
    // Reserve space for status area (3 lines) + prompt line
    println!("Status: Ready");
    println!("Data points: 0");
    println!("Last action: None");
    println!();
    print!("\x1B[1A"); 
    io::stdout().flush().unwrap();
}

pub fn handle_input(command: &str, chart: &mut Chart) -> bool {
    match command {
        "draw" | "d" => {
            print!("drawing")
        }
        "help" | "h" => {
            print!("\x1B[3A");  // Move up to update content   
            println!("  help, h - Show this help");
            println!("  quit, q - Exit the program");
            println!("  echo <text> - Echo back the text");
        },
        "quit" | "q" => {
            println!("Goodbye!");
            return false; // Signal to exit the loop
        },
        "clear" => {
            print!("\x1B[s");   // Save cursor position

            print!("\x1B[3A");  // Move up to update content        
            print!("\x1B[2K\rStatus: Active");
            print!("\x1B[B\x1B[2K\rData points:"); // {}", data.len());
            print!("\x1B[B\x1B[2K\rLast action:"); // {}", last_action);

            print!("\x1B[u");   // Restore cursor position
            io::stdout().flush().unwrap();
        },
        "reset" | "r" => {
            print!("\x1B[2J\x1B[1;1H"); // Clear screen
            chart.gen_data();
            draw(chart);
            setup_prompt_area();
        }
        "" => {
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

pub fn clear_screen() {
    println!("clearing screen");
    
}

pub fn refresh_chart() {
    println!("refreshing");
    println!("\x1B[2K");
    
}

pub fn quit_program() {
    println!("Goodbye!");
    std::process::exit(0);
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

#[allow(dead_code)]
fn fmt(num:u64) -> String {
    num.to_formatted_string(&Locale::fr)
}

#[allow(dead_code)]
fn z_score(x: f64, mean: f64, sd: f64) -> f64 {
    (x - mean) / sd
}
