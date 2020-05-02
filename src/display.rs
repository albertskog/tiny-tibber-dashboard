use embedded_graphics::{
    fonts::{Font6x6, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitive_style,
    primitives::{Line, Rectangle},
    style::{TextStyle},
};

#[cfg(not(target_arch = "arm"))]
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};

#[cfg(target_arch = "arm")]
use ssd1306::{
    prelude::*,
    Builder,
    interface::i2c::I2cInterface,
};


// Screen size
//const X_MAX: i32 = 127;
const Y_MAX: i32 = 63;

const TEXT_HEIGHT: i32 = 6;

// Time labels
const NUM_TIME_LABELS: usize = 6;
const X_TIME_MARGIN: i32 = 9;
const Y_TIME_MARGIN: i32 = TEXT_HEIGHT + 1;
const Y_TIME_TICK_START: i32 = Y_MAX - Y_TIME_MARGIN - 1;
const Y_TIME_TICK_END: i32 = Y_TIME_TICK_START + 2;
const Y_TIME_TEXT_POSITION: i32 = Y_TIME_TICK_START + 4;

// Price labels
const X_PRICE_TEXT_POSITION: i32 = 0;
const Y_PRICE_TEXT_OFFSET: i32 = -TEXT_HEIGHT / 2 + 1;
const X_PRICE_TICK_LENGTH: i32 = 3;

// Bars
const BAR_WIDTH: i32 = 4;
const BAR_SPACING: i32 = 1;
const Y_BAR_MAX: i32 = Y_MAX - Y_TIME_MARGIN;

#[cfg(target_arch = "arm")]
use linux_embedded_hal::I2cdev;

#[cfg(not(target_arch = "arm"))]
pub struct DisplayController {
    display: SimulatorDisplay<BinaryColor>,
    window: Window,
    y_zero_position: i32,
}

#[cfg(target_arch = "arm")]
pub struct DisplayController {
    display: GraphicsMode<I2cInterface<I2cdev>>,
    y_zero_position: i32,
}

impl DisplayController {
    #[cfg(not(target_arch = "arm"))]
    pub fn new() -> DisplayController {

        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledWhite)
            .build();

        DisplayController {
            display: SimulatorDisplay::new(Size::new(128, 64)),
            window: Window::new("Hello World", &output_settings),
            y_zero_position: Y_MAX - Y_TIME_MARGIN,
        }
    }

    #[cfg(target_arch = "arm")]
    pub fn new() -> DisplayController {
        let i2c = I2cdev::new("/dev/i2c-1").unwrap();
        let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
        
        display.init().expect("Failed to initialize the display");
        display.clear();
        display.flush().unwrap();

        DisplayController {
            display: display,
            y_zero_position: Y_MAX - Y_TIME_MARGIN,
        }
    }

    #[cfg(not(target_arch = "arm"))]
    pub fn run(&mut self) {
        self.window.show_static(&self.display);
    }

    #[cfg(target_arch = "arm")]
    pub fn run(&mut self) {
        self.display.flush().unwrap();
    }

    pub fn bars(&mut self, prices: &Vec<f64>) {
        let price_max = prices.iter().cloned().fold(0./0., f64::max);
        let price_min = prices.iter().cloned().fold(0./0., f64::min);

        let y_zero_offset = if price_min < 0.0 {
            (price_min * 100.0).round() as i32
        }
        else {
            0
        };

        self.y_zero_position = Y_BAR_MAX + y_zero_offset;

        let mut cursor = Point::new(X_TIME_MARGIN, self.y_zero_position);

        for (hour, price) in prices.iter().enumerate() {
            // Add X axis label
            if hour > 0 && hour % (24 / NUM_TIME_LABELS) == 0 {
                self.draw_time_label(cursor, format!("{}", hour));
            }

            // Add bar
            let bar_height = (*price * 100.0).round() as i32;
            self.draw_bar(cursor, bar_height);
            cursor += Point::new(BAR_WIDTH + BAR_SPACING, 0);
        }

        // Add Y labels
        self.draw_price_label(price_max);
        self.draw_price_label(price_min);
    }

    fn draw_time_label(&mut self, cursor: Point, text: String) {
        let tick_start = Point::new(cursor.x - 1, Y_TIME_TICK_START);
        let tick_end = Point::new(cursor.x - 1, Y_TIME_TICK_END);

        Line::new(tick_start, tick_end)
            .into_styled(primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1))
            .draw(&mut self.display)
            .unwrap();

        let x_label_offset = -1 * (text.len() as i32 * 6) / 2 + 1;
        let label_position = Point::new(cursor.x + x_label_offset, Y_TIME_TEXT_POSITION);

        Text::new(&text, label_position)
            .into_styled(TextStyle::new(Font6x6, BinaryColor::On))
            .draw(&mut self.display)
            .unwrap();
    }

    fn draw_bar(&mut self, cursor: Point, height: i32) {
        let top_left = if height > 0 {
            cursor + Point::new(0, -height)
        }
        else {
            cursor
        };

        let bottom_right = if height > 0 {
            cursor + Point::new(BAR_WIDTH - 1, 0)
        }
        else {
            cursor + Point::new(BAR_WIDTH - 1, -height)
        };

        Rectangle::new(top_left, bottom_right)
            .into_styled(primitive_style!(fill_color = BinaryColor::On))
            .draw(&mut self.display)
            .unwrap();
    }

    fn draw_price_label(&mut self, price: f64) {
        let text = format!("{}", (price * 100.0).round() as i32);
        let y_price_offset = (price * 100.0).round() as i32;
        let x_tick_offset = text.len() as i32 * 5;
        let y_tick_position = self.y_zero_position - y_price_offset;

        let tick_start = Point::new(x_tick_offset, y_tick_position);
        let tick_end = Point::new(x_tick_offset + X_PRICE_TICK_LENGTH, y_tick_position);

        Line::new(tick_start, tick_end)
            .into_styled(primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1))
            .draw(&mut self.display)
            .unwrap();

        
        let y_text_position = y_tick_position + Y_PRICE_TEXT_OFFSET;

        Text::new(&text, Point::new(X_PRICE_TEXT_POSITION, y_text_position))
            .into_styled(TextStyle::new(Font6x6, BinaryColor::On))
            .draw(&mut self.display)
            .unwrap();
    }
}

