use embedded_graphics::{
    fonts::{Font6x6, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitive_style,
    primitives::Rectangle,
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
        Text::new(text, Point::new(12, 64 - 5))
            .into_styled(TextStyle::new(Font6x6, BinaryColor::On))
            .draw(&mut self.display)
            .unwrap();
    }

    pub fn bars(&mut self, prices: &Vec<f64>) {
        let bar_width: i32 = 3;
        let bar_spacing: i32 = 1;
        let mut start_x: i32 = 12;
        let end_y: i32 = 64 - 8;

        let bar_style = primitive_style!(
            stroke_color = BinaryColor::On,
            fill_color = BinaryColor::On,
            stroke_width = bar_width as u32
        );

        for price in prices {
            let end_x = start_x + bar_width - 1;
            let start_y = end_y - (*price * 100.0).round() as i32;

            println!("start x: {}, y: {} end x: {}, y: {}", start_x, start_y, end_x, end_y);

            Rectangle::new(Point::new(start_x, start_y), Point::new(end_x, end_y))
                .into_styled(bar_style)
                .draw(&mut self.display)
                .unwrap();

            start_x = end_x + bar_spacing + 1;
        }

        
    }

    pub fn run(&mut self) {
        self.window.show_static(&self.display);
    }
}