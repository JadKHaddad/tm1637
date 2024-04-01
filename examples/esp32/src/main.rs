#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, embassy, gpio, peripherals::Peripherals, prelude::*, timer::TimerGroup,
    Delay,
};
use tm1637::{
    mappings::{LoCharBits, NumCharBits, SegmentBits, SpecialCharBits, UpCharBits},
    Brightness, TM1637,
};

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let io = gpio::IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut delay = Delay::new(&clocks);
    let clk = io.pins.gpio4.into_open_drain_output();
    let dio = io.pins.gpio19.into_open_drain_output();

    let mut tm = TM1637::new(clk, dio, delay, 10, 4);

    tm.clear().unwrap();
    tm.set_brightness(Brightness::L0).unwrap();

    // let mut all_seg_bits = SegmentBits::all_u8();
    // for _ in 0..7 {
    //     all_seg_bits.rotate_left(1);
    //     tm.write_raw(0, &all_seg_bits).unwrap();
    //     DelayNs::delay_ms(&mut delay, 1000);
    // }

    // let mut all_num_char_bits = NumCharBits::all_u8();
    // for _ in 0..10 {
    //     all_num_char_bits.rotate_left(1);
    //     tm.write_raw(0, &all_num_char_bits).unwrap();
    //     DelayNs::delay_ms(&mut delay, 1000);
    // }

    // let mut all_up_char_bits = UpCharBits::all_u8();
    // for _ in 0..13 {
    //     all_up_char_bits.rotate_left(1);
    //     tm.write_raw(0, &all_up_char_bits).unwrap();
    //     DelayNs::delay_ms(&mut delay, 1000);
    // }

    // let mut all_lo_char_bits = LoCharBits::all_u8();
    // for _ in 0..12 {
    //     all_lo_char_bits.rotate_left(1);
    //     tm.write_raw(0, &all_lo_char_bits).unwrap();
    //     DelayNs::delay_ms(&mut delay, 1000);
    // }

    // let mut all_sp_char_bits = SpecialCharBits::all_u8();
    // for _ in 0..12 {
    //     all_sp_char_bits.rotate_left(1);
    //     tm.write_raw(0, &all_sp_char_bits).unwrap();
    //     DelayNs::delay_ms(&mut delay, 1000);
    // }

    // // Display 19:06
    // tm.write_raw(
    //     0,
    //     &[
    //         NumCharBits::One as u8,
    //         NumCharBits::Nine as u8 | SegmentBits::SegPoint as u8,
    //         NumCharBits::Zero as u8,
    //         NumCharBits::Six as u8,
    //     ],
    // )
    // .unwrap();

    // // Make the dots blink
    // let mut show = true;
    // loop {
    //     let bit = match show {
    //         true => NumCharBits::Nine as u8 | SegmentBits::SegPoint as u8,
    //         false => NumCharBits::Nine as u8,
    //     };

    //     tm.write_raw(1, &[bit]).unwrap();

    //     DelayNs::delay_ms(&mut delay, 500);

    //     show = !show;
    // }

    // Rotating circle at address 0
    // First of all we create the shapes wewant to animate

    //  ---
    // |   |
    // |
    //  ---
    // This shape consists of these segments: B, A, F, E and D.
    // Let's create the shape
    let shape_1 = SegmentBits::SegB as u8
        | SegmentBits::SegA as u8
        | SegmentBits::SegF as u8
        | SegmentBits::SegE as u8
        | SegmentBits::SegD as u8;

    //  ---
    // |
    // |   |
    //  ---
    // This shape consists of these segments: A, F, E, D and C.
    let shape_2 = SegmentBits::SegA as u8
        | SegmentBits::SegF as u8
        | SegmentBits::SegE as u8
        | SegmentBits::SegD as u8
        | SegmentBits::SegC as u8;

    // and so on...

    //
    // |   |
    // |   |
    //  ---
    let shape_3 = SegmentBits::SegF as u8
        | SegmentBits::SegE as u8
        | SegmentBits::SegD as u8
        | SegmentBits::SegC as u8
        | SegmentBits::SegB as u8;

    //  ---
    //     |
    // |   |
    //  ---
    let shape_4 = SegmentBits::SegE as u8
        | SegmentBits::SegD as u8
        | SegmentBits::SegC as u8
        | SegmentBits::SegB as u8
        | SegmentBits::SegA as u8;

    //  ---
    // |   |
    //     |
    //  ---
    let shape_5 = SegmentBits::SegD as u8
        | SegmentBits::SegC as u8
        | SegmentBits::SegB as u8
        | SegmentBits::SegA as u8
        | SegmentBits::SegF as u8;

    //  ---
    // |   |
    // |   |
    //
    let shape_6 = SegmentBits::SegC as u8
        | SegmentBits::SegB as u8
        | SegmentBits::SegA as u8
        | SegmentBits::SegF as u8
        | SegmentBits::SegE as u8;

    let mut shapes = [shape_1, shape_2, shape_3, shape_4, shape_5, shape_6];
    loop {
        shapes.rotate_left(1);
        tm.write_raw(0, &shapes[0..1]).unwrap();
        DelayNs::delay_ms(&mut delay, 200);
    }
}
