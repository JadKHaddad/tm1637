#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    gpio::{Input, Io, Level, Output, OutputOpenDrain, Pull},
    prelude::*,
};
use futures::StreamExt;
use log::info;
use tm1637_embedded_hal::{
    mappings::{self, DigitBits, RotatingCircleBits, SegmentBits, UpCharBits},
    scroll::{ScrollDirection, ScrollStyle},
    tokens::Blocking,
    Brightness, TM1637Builder,
};

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    esp_println::logger::init_logger_from_env();

    let timer0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    let delay = Delay {};
    let clk = Output::new(peripherals.GPIO4.degrade(), Level::Low);
    let dio = OutputOpenDrain::new(peripherals.GPIO19.degrade(), Level::Low, Pull::Up);

    // let mut tm = TM1637Builder::new(clk, dio, delay).build_async::<4>();

    // tm.init().await.unwrap();

    // let (clk, dio, delay) = tm.into_parts();

    let mut tm = TM1637Builder::new(clk, dio, delay)
        .brightness(Brightness::L4)
        .delay_us(30)
        .build::<6, Blocking>();

    tm.init().unwrap();
    // let bytes = [
    //     SegmentBits::Dot as u8,
    //     SegmentBits::Dot as u8,
    //     SegmentBits::Dot as u8,
    //     SegmentBits::Dot as u8,
    //     SegmentBits::Dot as u8,
    //     SegmentBits::Dot as u8,
    // ];
    // tm.display_slice(0, &bytes).unwrap();

    let bytes = [
        DigitBits::Zero as u8,
        DigitBits::One as u8,
        DigitBits::Two as u8,
        DigitBits::Three as u8,
        DigitBits::Four as u8,
        DigitBits::Five as u8,
    ];

    // if bytes len >= 6
    // so take len - pos then reverse then give position 3
    // if pos > len then empty iterator

    // {
    //     let pos = 0;

    //     if pos == 0 {
    //         let cal_bytes = [bytes[5], bytes[4], bytes[3], bytes[2], bytes[1], bytes[0]];

    //         tm.display_slice(3, &cal_bytes).unwrap();
    //     }

    //     if pos == 1 {
    //         let cal_bytes = [bytes[4], bytes[3], bytes[2], bytes[1], bytes[0]];

    //         tm.display_slice(3, &cal_bytes).unwrap();
    //     }

    //     if pos == 2 {
    //         let cal_bytes = [bytes[3], bytes[2], bytes[1], bytes[0]];

    //         tm.display_slice(3, &cal_bytes).unwrap();
    //     }

    //     if pos == 3 {
    //         let cal_bytes = [bytes[2], bytes[1], bytes[0]];

    //         tm.display_slice(3, &cal_bytes).unwrap();
    //     }

    //     if pos == 4 {
    //         let cal_bytes = [bytes[1], bytes[0]];

    //         tm.display_slice(3, &cal_bytes).unwrap();
    //     }

    //     if pos == 5 {
    //         let cal_bytes = [bytes[0]];

    //         tm.display_slice(3, &cal_bytes).unwrap();
    //     }
    // }

    // for n in 0..999999 {
    //     tm.options().position(2).u32_6(n).display().ok();

    //     Timer::after(Duration::from_millis(100)).await;
    // }

    tm.options()
        .position(0)
        .str("Error.")
        .dot(0)
        .remove_dot(4)
        .display()
        .ok();
    // tm.options().position(5).u8_2(23).display().ok();

    let mut buffer = ryu::Buffer::new();
    let printed = buffer.format(1.234);

    tm.options().str(printed).display().ok();

    loop {}
}
