use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    // primitives::Line,
    style::{TextStyle},
};
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};

pub struct DisplayController {
    display: SimulatorDisplay<BinaryColor>,
    window: Window,
}

impl DisplayController {
    pub fn new() -> DisplayController {

        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledWhite)
            .build();

        DisplayController {
            display: SimulatorDisplay::new(Size::new(128, 64)),
            window: Window::new("Hello World", &output_settings),
        }
    }

    pub fn draw(&mut self, text: &str){
        Text::new(text, Point::new(5, 10))
            .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
            .draw(&mut self.display)
            .unwrap();
    }

    pub fn run(&mut self) {
        self.window.show_static(&self.display);
    }
}

// fn main() -> Result<(), core::convert::Infallible> {
//     let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

//     let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

//     Line::new(Point::new(64, 64), Point::new(0, 64))
//         .into_styled(line_style)
//         .draw(&mut display)?;

//     Line::new(Point::new(64, 64), Point::new(80, 80))
//         .into_styled(line_style)
//         .draw(&mut display)?;

//     Text::new("Hello World!", Point::new(5, 50))
//         .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
//         .draw(&mut display)?;

//     let output_settings = OutputSettingsBuilder::new()
//         .theme(BinaryColorTheme::OledWhite)
//         .build();
//     Window::new("Hello World", &output_settings).show_static(&display);

//     Ok(())
// }