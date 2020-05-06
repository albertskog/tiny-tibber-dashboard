# Tiny Tibber Dashboard

This project presents you with today's electricity price from Tibber, shown on a tiny SSD1306 128 x 64 pixel display connected to a Raspberry Pi.

# Requirements

```
sudo apt install openssl-dev libsdl2-dev
```

# Usage
1. Get an API key from https://developer.tibber.com/
2. Do either of these:
    * To test the code on your computer, run `export TIBBER_API_KEY="your key here"`, then `cargo run`. A simulated display will appear in a window.
    * To run manually on Raspberry Pi, do the same on the Pi.
    * To run with Balena, set up a Pi and an app in Balena cloud, then run: `balena push your-app-name`
