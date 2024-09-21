#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, gpio, peripherals::Peripherals, prelude::*, Delay};
use tm1637_embedded_hal::{
    blocking::TM1637, demo::blocking::Demo, mappings::from_ascii_byte, Brightness,
};

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let io = gpio::IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let delay = Delay::new(&clocks);
    let clk = io.pins.gpio4.into_open_drain_output();
    let dio = io.pins.gpio19.into_open_drain_output();

    let mut tm = TM1637::builder(clk, dio, delay).build();

    // Initialize the display.
    // Clear the display and set the initial brightness.
    tm.init().unwrap();

    // Change the brightness
    tm.write_brightness(Brightness::L3).unwrap();

    // let segs = "Error".as_bytes().iter().copied().map(from_ascii_byte);

    // tm.move_segments_window(1, &b"Error".map(from_ascii_byte), 500)
    //     .ok();

    // let mut segs = [b'E', b'r', b'r', b'o', b'r', b' ', b'O', b'H', b' '].map(from_ascii_byte);
    // let s = "Error"
    //     .as_bytes()
    //     .iter()
    //     .take(5)
    //     .copied()
    //     .map(from_ascii_byte)
    //     .collect::<[u8; 5]>();

    // let str = "Error".as_bytes();
    // let mut segs = [0u8; 5];

    // str.iter()
    //     .take(5)
    //     .enumerate()
    //     .for_each(|(i, &byte)| segs[i] = from_ascii_byte(byte));

    let segs = b"Err Oh no ".map(from_ascii_byte);
    tm.move_segments_efficient::<4>(0, &segs, 500).ok();

    // let segs = b"Error".map(from_ascii_byte);
    // let mut segs = [b'E', b'r'].map(from_ascii_byte);

    // tm.move_segments(1, &mut segs, 500).ok();
    loop {}
    let mut demo = Demo::new(tm, delay, 500);
    loop {
        demo.rotating_circle(20, 200).unwrap();
        demo.time(10, 500).unwrap();
        demo.on_off(10, 200).unwrap();
        demo.moving_segments().unwrap();
        demo.moving_digits().unwrap();
        demo.countdown().unwrap();
        demo.moving_up_chars().unwrap();
        demo.moving_lo_chars().unwrap();
        demo.moving_special_chars().unwrap();
    }
}
