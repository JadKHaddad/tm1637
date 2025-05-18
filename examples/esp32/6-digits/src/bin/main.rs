#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    Config,
};
use tm1637_embedded_hal::{
    mappings::{DigitBits, LoCharBits, SpecialCharBits, UpCharBits},
    options::{ScrollDirection, ScrollStyle},
    Brightness, TM1637Builder,
};

const DELAY_MS: u32 = 2000;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[esp_hal::main]
fn main() -> ! {
    let peripherals = esp_hal::init(Config::default());

    let delay = Delay::new();
    let clk = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());
    let dio = Output::new(peripherals.GPIO19, Level::Low, OutputConfig::default());

    // Create a TM1637 instance with 6 digits.
    let mut tm = TM1637Builder::new(clk, dio, delay)
        // Set the brightness to level 3.
        .brightness(Brightness::L3)
        // Set the delay between each bit to 100us. Experiment with this value to find the best value for your display.
        .delay_us(100)
        .build_blocking::<6>();

    // Initialize the display.
    // Clear the display and set the initial brightness.
    tm.init().ok();

    // Display the number `123456` starting from the first position on the display.
    // 6-digit displays are a little bit tricky to align. Use the High-Level API for a more convenient way.
    let bytes = [
        0b01001111, /* 3 */
        0b01011011, /* 2 */
        0b00000110, /* 1 */
        0b01111101, /* 6 */
        0b01101101, /* 5 */
        0b01100110, /* 4 */
    ];
    tm.display_slice(0, &bytes).ok();

    delay.delay_millis(DELAY_MS);

    // Use the `mappings` module to display the sequence `AC2bS?`.
    let bytes = [
        DigitBits::Two as u8,
        UpCharBits::UpC as u8,
        UpCharBits::UpA as u8,
        SpecialCharBits::QuestionMark as u8,
        LoCharBits::LoB as u8,
        UpCharBits::UpS as u8,
    ];
    tm.display_slice(0, &bytes).ok();

    delay.delay_millis(DELAY_MS);

    // Use a High-Level API to display what ever you want.

    // Display the string `Error` at the first position.
    tm.options().position(0).str("Error ").display().ok();

    delay.delay_millis(DELAY_MS);

    // Display a string with dots.
    tm.options().str("E.r.r.o.r. .").display().ok();

    delay.delay_millis(DELAY_MS);

    // Using the High-Level API you can concatenate multiple bytes. let's display a calculated temperature value.
    let temperature = 1596;
    tm.options().u16_4(temperature).str(" c").display().ok();

    delay.delay_millis(DELAY_MS);

    // Clear the display.
    tm.clear().ok();

    // Create a timer that counts from -99 to 99.
    for i in -99..100 {
        // `r_i32_6` translates an `i32` to it's 6 digit representation aligning the bytes to the right.
        tm.options().r_i32_6(i).display().ok();

        delay.delay_millis(10);
    }

    delay.delay_millis(DELAY_MS);

    // You can also display floating point numbers using `ryu` crate.
    let mut buffer = ryu::Buffer::new();
    let printed = buffer.format(10.2345);
    tm.options().str(printed).display().ok();

    delay.delay_millis(DELAY_MS);

    tm.clear().ok();

    // If your bytes got too long you can animate them to make them fit.
    tm.options()
        .str("HELLO ruSt 1234 ")
        .scroll()
        .style(ScrollStyle::Circular)
        .direction(ScrollDirection::LeftToRight)
        .delay_ms(200)
        .finish()
        .run();

    delay.delay_millis(DELAY_MS);

    // The most useless feature of all time, that got me working 2 days straight.
    // Flip your display upside down.
    tm.options().str("FLIPPI").flip().display().ok();

    delay.delay_millis(DELAY_MS);

    // Double flip has no effect.
    tm.options().str("FLIPPI").flip().flip().display().ok();

    delay.delay_millis(DELAY_MS);

    // You can also animate flipped displays.
    tm.options()
        .str("FLIPPI ")
        .str("FLOPPI")
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

    tm.options().str("8.8.8.8.8.8.").display().ok();

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

    tm.options().str("done. . .").display().ok();

    #[allow(clippy::empty_loop)]
    loop {}
}
