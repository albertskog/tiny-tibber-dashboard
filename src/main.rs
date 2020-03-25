mod display;

use display::DisplayController;


fn main() -> Result<(), core::convert::Infallible> {
    let mut display = DisplayController::new();
    display.draw("hello");
    display.run();
    Ok(())
}