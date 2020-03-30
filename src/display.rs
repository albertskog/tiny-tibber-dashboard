use embedded_graphics::{
    fonts::{Font6x6, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitive_style,
    primitives::{Line, Rectangle},
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

    // pub fn draw(&mut self, text: &str){
    //     Text::new(text, Point::new(12, 64 - 5))
    //         .into_styled(TextStyle::new(Font6x6, BinaryColor::On))
    //         .draw(&mut self.display)
    //         .unwrap();
    // }

    pub fn bars(&mut self, prices: &Vec<f64>) {
        let bar_width: i32 = 4;
        let bar_spacing: i32 = 1;
        let mut start_x: i32 = 6;
        let end_y: i32 = 64 - 8;

        let bar_style = primitive_style!(
            stroke_color = BinaryColor::On,
            fill_color = BinaryColor::On,
            stroke_width = bar_width as u32
        );

        let mut hour = 0;
        let num_labels = 6;

        for price in prices {
            // Add X axis label
            if hour % (24 / num_labels) == 0 {
                self.add_time_label(format!("{}", hour), start_x);
            }
            hour = hour + 1;

            // Draw bar
            let end_x = start_x + bar_width - 1;
            let start_y = end_y - (*price * 100.0).round() as i32;

            println!("start x: {}, y: {} end x: {}, y: {}", start_x, start_y, end_x, end_y);

            Rectangle::new(Point::new(start_x, start_y), Point::new(end_x, end_y))
                .into_styled(bar_style)
                .draw(&mut self.display)
                .unwrap();

            start_x = end_x + bar_spacing + 1;
        }

        // Add final X label
        self.add_time_label(format!("{}", 0), start_x);

        // Add Y labels
        let price_max = prices.iter().cloned().fold(0./0., f64::max);
        self.add_price_label(price_max);

        let price_min = prices.iter().cloned().fold(0./0., f64::min);
        self.add_price_label(price_min);

    }

    pub fn run(&mut self) {
        self.window.show_static(&self.display);
    }

    fn add_time_label(&mut self, text: String, x_position: i32) {
        let time_position_y = 57;
        let tick_offset = - 1;
        let label_offset = text.len() as i32 * 6 / 2 - 1;

        Line::new(Point::new(x_position + tick_offset, time_position_y), Point::new(x_position + tick_offset, time_position_y - 1))
            .into_styled(primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1))
            .draw(&mut self.display)
            .unwrap();

        Text::new(&text, Point::new(x_position - label_offset, time_position_y + 2))
            .into_styled(TextStyle::new(Font6x6, BinaryColor::On))
            .draw(&mut self.display)
            .unwrap();
    }

    fn add_price_label(&mut self, price: f64) {
        let text = format!("{}", (price * 100.0).round() as i32);
        let x_position = 0;
        let end_y: i32 = 64 - 8;
        let y_position = end_y - (price * 100.0).round() as i32;
        let line_offset = text.len() as i32 * 5;

        Line::new(Point::new(x_position + line_offset, y_position), Point::new(x_position + line_offset + 1, y_position))
            .into_styled(primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1))
            .draw(&mut self.display)
            .unwrap();

        Text::new(&text, Point::new(x_position, y_position - 2))
            .into_styled(TextStyle::new(Font6x6, BinaryColor::On))
            .draw(&mut self.display)
            .unwrap();
    }

}
