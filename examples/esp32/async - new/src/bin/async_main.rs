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
    mappings::{DigitBits, UpCharBits},
    scroll::{ScrollDirection, ScrollStyle},
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

    let mut tm = TM1637Builder::new(clk, dio, delay).build_async::<4>();

    tm.init().await.unwrap();

    let bytes = [
        UpCharBits::UpH as u8,
        UpCharBits::UpE as u8,
        UpCharBits::UpL as u8,
        UpCharBits::UpL as u8,
        UpCharBits::UpO as u8,
        0,
    ];

    // let count = tm
    //     .options()
    //     .put_str("HELLO")
    //     .set_dot(1)
    //     .flip()
    //     .animate()
    //     .run()
    //     .await;

    // info!("Count: {}", count);

    let (clk, dio, delay) = tm.into_parts();

    let mut tm = TM1637Builder::new(clk, dio, delay).build_blocking::<4>();

    tm.init().unwrap();

    let count = tm
        .options()
        .str("123456")
        // .position(1)
        // .put_slice(&bytes)
        .flip()
        .scroll()
        .delay_ms(700)
        .direction(ScrollDirection::RightToLeft)
        .style(ScrollStyle::Linear)
        .run();

    info!("Count: {}", count);

    // let slice = &[
    //     DigitBits::One as u8,
    //     DigitBits::Two as u8,
    //     DigitBits::Three as u8,
    //     DigitBits::Four as u8,
    //     DigitBits::Five as u8,
    //     DigitBits::Six as u8,
    // ];

    // let iter = windows_non_overlapping::<4>(slice, Direction::RightToLeft);

    // tm.animate(0, 700, iter).count();

    loop {}
}
