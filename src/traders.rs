

// Traders which will participate in a prediction market
// TODO: integrate as desired

#[allow(dead_code)]
pub struct Trader {
    yes_owned: u64,
    no_owned: u64
}


#[allow(dead_code)]
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