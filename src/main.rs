

pub mod traders;
pub mod wiener;
pub mod draw;
// pub mod ansi;

use std::io;
use std::io::Write;
use num_format::{Locale, ToFormattedString};

// steps represent number of trading days
// 252 trading days per year
// 390 minutes in a trading day

fn main() {
    
    let mut data = wiener::simulate_gbm(252usize);
    draw_chart();

    // run prompt
    loop {
        let input = prompt(">");
        
        if !handle_input(&input, &mut data) {
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

    print!("\x1B[1A");
    print!("\x1B[2K");
    
    input.trim().to_string()
}

fn draw_chart() {
    let steps = 252usize;
    let data = wiener::simulate_gbm(steps);
    let xmax = steps as f64;
    let ymax = 250.0;
    let _ = draw::console::chart(data, xmax, ymax);
    println!(".");
    println!(".");
    println!(".");
}

/*
\x1B[2J\x1B[1;1H - Clear screen
\x1B[1A - Move cursor up one line
\x1B[2K - Clear entire current line
\x1B[0J - Clear from cursor to end of screen
\x1B[s - Save cursor position
\x1B[u - Restore cursor position
*/

pub fn handle_input(command: &str, _data: &mut Vec<(f64, f64)>) -> bool {
    match command {
        "draw" | "d" => {
            print!("drawing")
        }
        "help" | "h" => {
            println!("Available commands:");
            println!("  help, h - Show this help");
            println!("  quit, q - Exit the program");
            println!("  echo <text> - Echo back the text");
            println!("  clear - Clear the screen");
        },
        "quit" | "q" => {
            println!("Goodbye!");
            return false; // Signal to exit the loop
        },
        "clear" => {
            print!("\x1B[s");        // Save current cursor position
            print!("\x1B[1A");       // Move up to update content        
            print!("\x1B[2K");       // Clear that line
            print!("\x1B[1G");       // Move to beginning
            print!("Updated text");  // Print new content
            print!("\x1B[1A");       // Move up to update content        
            print!("\x1B[2K");       // Clear that line
            print!("\x1B[1G");       // Move to beginning
            print!("updated text");  // Print new content
            print!("\x1B[1A");       // Move up to update content        
            print!("\x1B[2K");       // Clear that line
            print!("\x1B[1G");       // Move to beginning
            print!("Updated header");  // Print new content
            print!("\x1B[u");
            io::stdout().flush().unwrap();
        },
        "reset" | "r" => {
            print!("\x1B[2J\x1B[1;1H"); // Clear screen
            draw_chart();
        }
        "" => {
            // Do nothing for empty input
        },
        _ => {
            println!("Unknown command: '{}'. Type 'help' for available commands.", command);
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
