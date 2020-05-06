use timer;
use chrono::{
    prelude::*,
    Duration,
};

mod display;
use display::DisplayController;

mod tibber;
use tibber::get_prices;

#[cfg(target_arch = "arm")]
use std::thread::sleep;

fn main(){
    let prices = get_prices();
    // let prices: Vec<f64> = vec!(0.05, 0.05, -0.03, 0.05, 0.5, 0.5, 0.5, 0.5, 0.05, 0.05, 0.04, 0.05, 0.5, 0.5, 0.5, 0.5, 0.05, 0.05, 0.04, 0.05, 0.5, 0.5, 0.5, 0.5);

    let mut display = DisplayController::new();
    display.bars(&prices);
    display.run();

    let timer = timer::Timer::new();
    let refresh_time = (Local::now() + Duration::days(1)).date().and_hms(0, 0, 0);
    timer.schedule(refresh_time, Some(Duration::days(1)), move || {
        display.clear();
        display.bars(&prices);
        display.run();
    });

    #[cfg(target_arch = "arm")]
    loop {
        sleep(Duration::seconds(1).to_std().unwrap());
    }
}
