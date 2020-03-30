mod display;
use display::DisplayController;

mod tibber;
use tibber::get_prices;

fn main() -> Result<(), core::convert::Infallible> {
    let prices = get_prices().unwrap();
    // let prices: Vec<f64> = vec!(0.05, 0.05, 0.04, 0.05, 0.5, 0.5, 0.5, 0.5, 0.05, 0.05, 0.04, 0.05, 0.5, 0.5, 0.5, 0.5, 0.05, 0.05, 0.04, 0.05, 0.5, 0.5, 0.5, 0.5);

    let mut display = DisplayController::new();
    display.bars(&prices);
    display.run();
    Ok(())
}