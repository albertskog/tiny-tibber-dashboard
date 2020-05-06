# Tiny Tibber Dashboard

This project presents you with today's electricity price from Tibber, shown on a tiny SSD1306 128 x 64 pixel display connected to a Raspberry Pi.

## Requirements

``` shell
sudo apt install openssl-dev libsdl2-dev
```

## Usage

Get an API key from <https://developer.tibber.com/>,
then do either of these:

* To test the code on your computer, run

    ``` shell
    export TIBBER_API_KEY="your key here"`
    ```

    then `cargo run`. A simulated display will appear in a window.
* To run manually on Raspberry Pi, do the same on the Pi with an SSD1306 display connected to the I2C GPIO pins.
* To run with Balena:

  * Set up a Pi and an app in Balena cloud
  * Cross compile with `cross`:

    ``` shell
    cross build --target armv7-unknown-linux-gnueabihf
    ```

  * Deploy: `balena push <app name>`

    Or, if using local-mode:

    ``` shell
    balena push <device ip> --env TIBBER_API_TOKEN=$TIBBER_API_TOKEN
    ```
