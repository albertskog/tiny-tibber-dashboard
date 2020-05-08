use chrono::{
    prelude::*,
    Duration,
};
use std::thread::sleep;

mod display;
use display::DisplayController;

mod tibber;
use tibber::get_prices;


fn main(){
    let mut prices = get_prices();
    // let prices: Vec<f64> = vec!(0.05, 0.05, -0.03, 0.05, 0.5, 0.5, 0.5, 0.5, 0.05, 0.05, 0.04, 0.05, 0.5, 0.5, 0.5, 0.5, 0.05, 0.05, 0.04, 0.05, 0.5, 0.5, 0.5, 0.5);

    let mut display = DisplayController::new();
    display.bars(&prices);
    display.run();

    let mut today = Local::today();

    loop {
        if today != Local::today() {
            prices = get_prices();
            today = Local::today();
            println!("It's a new day!");
        }
        display.clear();
        display.bars(&prices);
        display.run();

        sleep(Duration::seconds(1).to_std().unwrap());
    }
}
