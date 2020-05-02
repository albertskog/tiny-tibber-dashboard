mod display;
use display::DisplayController;

mod tibber;
use tibber::get_prices;

#[cfg(target_arch = "arm")]
use std::thread::sleep;
#[cfg(target_arch = "arm")]
use std::time::Duration;

fn main(){
    let prices = get_prices();
    // let prices: Vec<f64> = vec!(0.05, 0.05, -0.03, 0.05, 0.5, 0.5, 0.5, 0.5, 0.05, 0.05, 0.04, 0.05, 0.5, 0.5, 0.5, 0.5, 0.05, 0.05, 0.04, 0.05, 0.5, 0.5, 0.5, 0.5);

    let mut display = DisplayController::new();
    display.bars(&prices);
    display.run();

    #[cfg(target_arch = "arm")]
    loop {
        sleep(Duration::from_millis(33));
    }
}