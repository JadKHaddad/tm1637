#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputOpenDrain, Pull};
use esp_hal::prelude::*;
use tm1637_embedded_hal::mappings::{DigitBits, LoCharBits, UpCharBits};
use tm1637_embedded_hal::options::{ScrollDirection, ScrollStyle};
use tm1637_embedded_hal::{Brightness, TM1637Builder};

const DELAY_MS: u32 = 2000;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let delay = Delay::new();
    let clk = Output::new(peripherals.GPIO4.degrade(), Level::Low);
    // We use OutputOpenDrain that implements `OutputPin` and `InputPin` because the `ack` feature is enabled.
    let dio = OutputOpenDrain::new(peripherals.GPIO19.degrade(), Level::Low, Pull::Up);

    // Create a TM1637 instance with 4 digits.
    let mut tm = TM1637Builder::new(clk, dio, delay)
        // Set the brightness to level 3.
        .brightness(Brightness::L3)
        // Set the delay between each bit to 100us. Experiment with this value to find the best value for your display.
        .delay_us(100)
        .build_blocking::<4>();

    // Initialize the display.
    // Clear the display and set the initial brightness.
    tm.init().ok();

    // Display the number `1234` starting from the first position on the display.
    let bytes = [0b00000110, 0b01011011, 0b01001111, 0b01100110];
    tm.display_slice(0, &bytes).ok();

    delay.delay_millis(DELAY_MS);

    // Use the `mappings` module to display the sequence `AC2b`.
    let bytes = [
        UpCharBits::UpA as u8,
        UpCharBits::UpC as u8,
        DigitBits::Two as u8,
        LoCharBits::LoB as u8,
    ];
    tm.display_slice(0, &bytes).ok();

    // Use a High-Level API to display what ever you want.

    // Display the string `Err` at the first position.
    tm.options().position(0).str("Err ").display().ok();

    delay.delay_millis(DELAY_MS);

    // Using the High-Level API you can concatenate multiple bytes. let's display a calculated temperature value.
    let temprature = 15;
    tm.options().u8_2(temprature).str(" c").display().ok();

    delay.delay_millis(DELAY_MS);

    // Clear the display.
    tm.clear().ok();

    // Create a timer that counts from -99 to 99.
    for i in -99..100 {
        // `r_i16_4` translates an `i16` to it's 4 digit representation aligning the bytes to the right.
        tm.options().r_i16_4(i).display().ok();

        delay.delay_millis(10);
    }

    delay.delay_millis(DELAY_MS);

    tm.clear().ok();

    // Create a clock that goes from 23:58 to 23:59.
    for minute in 58..=59 {
        for second in 0..15 {
            let colon = second % 2 == 0;
            tm.options()
                .clock()
                .hour(23)
                .minute(minute)
                .finish()
                .set_dot(1, colon)
                .display()
                .ok();

            delay.delay_millis(100);
        }
    }

    delay.delay_millis(DELAY_MS);

    // If your bytes got too long you can animate them to make them fit.
    tm.options()
        .str("HELLO ruSt 123 ")
        .scroll()
        .style(ScrollStyle::Circular)
        .direction(ScrollDirection::LeftToRight)
        .delay_ms(200)
        .finish()
        .run();

    delay.delay_millis(DELAY_MS);

    // The most useless feature of all time, that got me working 2 days straight.
    // Flip your display upside down.
    tm.options().str("FLIP").flip().display().ok();

    delay.delay_millis(DELAY_MS);

    // Double flip has no effect.
    tm.options().str("FLIP").flip().flip().display().ok();

    delay.delay_millis(DELAY_MS);

    // You can also animate flipped displays.
    tm.options()
        .str("FLIP ")
        .str("FLOP")
        .flip()
        .scroll()
        .style(ScrollStyle::Linear)
        .direction(ScrollDirection::LeftToRight)
        .delay_ms(300)
        .finish()
        .run();

    delay.delay_millis(DELAY_MS);

    // Turn the display on/off.
    for _ in 0..5 {
        tm.off().ok();
        delay.delay_millis(300);
        tm.on().ok();
        delay.delay_millis(300);
    }

    delay.delay_millis(DELAY_MS);

    tm.options().str("8888").display().ok();

    // Set the brightness level.
    let levels = [
        Brightness::L0,
        Brightness::L1,
        Brightness::L2,
        Brightness::L3,
        Brightness::L4,
        Brightness::L5,
        Brightness::L6,
        Brightness::L7,
        Brightness::L3,
    ];

    for level in levels {
        tm.set_brightness(level).ok();

        delay.delay_millis(500);
    }

    delay.delay_millis(DELAY_MS);

    tm.clear().ok();

    // Loading animation.
    for _ in 0..8 {
        tm.circles().rotating().delay_ms(100).finish().run();
    }

    delay.delay_millis(DELAY_MS);

    tm.options().str("done").display().ok();

    #[allow(clippy::empty_loop)]
    loop {}
}
