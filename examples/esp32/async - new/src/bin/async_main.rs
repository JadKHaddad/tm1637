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

    let mut tm = TM1637Builder::new(clk, dio, delay).build::<4, Blocking>();

    tm.init().unwrap();

    let bytes = [
        UpCharBits::UpH as u8,
        UpCharBits::UpE as u8,
        UpCharBits::UpL as u8,
        UpCharBits::UpL as u8,
        UpCharBits::UpO as u8,
    ];

    // let count = tm.options().str("HELLO").position(2).flip().display().ok();
    // let count = tm.options().slice(&bytes).position(1).flip().display().ok();
    // .scroll()
    // .delay_ms(700)
    // .direction(ScrollDirection::RightToLeft)
    // .style(ScrollStyle::Linear)
    // .finish()
    // .run();
    // let cricles = RotatingCircleBits::all_u8();
    // let count = tm
    //     .options()
    //     .slice(&cricles)
    //     .position(0)
    //     // .flip()
    //     .repeat()
    //     .finish()
    //     .run();

    // let count = tm
    //     .options()
    //     .rotating_circle()
    //     .position(1)
    //     .delay_ms(70)
    //     // .flip()
    //     .run();

    // info!("Count: {:?}", count);

    let slice = &[
        DigitBits::One as u8,
        DigitBits::Two as u8,
        DigitBits::Three as u8,
        DigitBits::Four as u8,
        DigitBits::Five as u8,
        DigitBits::Six as u8,
    ];

    // let iter = windows_non_overlapping::<4>(slice, Direction::RightToLeft);

    // tm.animate(0, 700, iter).count();

    tm.options().flip().u8_2(15).str("oC").flip().display().ok();

    loop {}
}
