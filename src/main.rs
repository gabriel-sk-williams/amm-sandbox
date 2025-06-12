use num_format::{Locale, ToFormattedString};


#[derive(Debug)]
pub struct Trader {
    yes_owned: u64,
    no_owned: u64
}


fn main() {
    println!("pm_amm");

    let k: u64 = 2_500_000_000;
    let yes_amount: u64 = 1800;
    let no_amount: u64 = 200;

    let yes_price: u64 = 50;
    let no_price: u64 = 50;

    // xy=k, x and y each represent total value of one token
    // let x = yes_amount * yes_price;
    // let y = no_amount * no_price;
    // let k = x * y;

    println!("{}", fmt(k));

    let traders = create_traders();

    println!("{:?}", traders);

    let (x, y) = get_prices(yes_amount, no_amount, k);

    println!("{}", fmt(x));
    println!("{}", fmt(y));

}

fn fmt(num:u64) -> String {
    num.to_formatted_string(&Locale::fr)
}

fn create_traders(/*num:u64*/) -> Vec<Trader> {

    let trader_one = Trader {
        yes_owned: 500,
        no_owned: 0,
    };

    let trader_two = Trader {
        yes_owned: 0,
        no_owned: 500,
    };

    let bruh = vec![trader_one, trader_two];

    bruh
}

fn get_prices(yes_amount: u64, no_amount: u64, k: u64) -> (u64, u64) {

    // x + y must equal 100;

    let x = 80;
    let y = 20;

    return (x, y)
}

/*
fn uniswap() {

}

fn curve() {

}

fn balancer() {

}
*/
