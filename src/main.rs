mod display;
use display::DisplayController;

mod tibber;
use tibber::get_prices;

fn main() -> Result<(), core::convert::Infallible> {
    let prices = get_prices().unwrap();

    let mut display = DisplayController::new();
    display.draw("00      06      12      18     00");
    display.bars(&prices);
    display.run();
    Ok(())
}