#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    gpio::{Input, Io, Level, Output, OutputOpenDrain, Pull},
    prelude::*,
};
use log::info;
use tm1637_embedded_hal::{
    mappings::UpCharBits, AnimationStyle, Brightness, Direction, TM1637Builder,
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

    let mut tm = TM1637Builder::new(clk, dio, delay)
        .brightness(Brightness::L3)
        .build_async::<4>();

    tm.init().await.unwrap();
    // let bytes = [UpCharBits::UpF as u8];

    // tm.write_segments_raw_flipped(1, &bytes).await.ok();

    // tm.display_str_flipped(0, "HE").await.ok();

    // tm.fit_str(0, "StALIOn ", 700).await.ok();

    // tm.display_str_rev(0, "Error").await.ok();

    // tm.put_str("HEL").fit(300).await.ok();

    // tm.put_str("SUPEA HELLO ").display_or_fit(300).await.ok();

    let bytes = [
        UpCharBits::UpH as u8,
        UpCharBits::UpE as u8,
        UpCharBits::UpL as u8,
        UpCharBits::UpL as u8,
        UpCharBits::UpO as u8,
        0,
    ];

    // tm.move_slice_mapped(0, &bytes, 500, |byte| byte, Direction::RightToLeft)
    //     .await
    //     .ok();

    // tm.fit_slice_flipped(0, &bytes, 500).await.ok();

    // tm.put_str("HELLO ")
    //     .move_overlapping(700, Direction::LeftToRight)
    //     .await
    //     .ok();

    // tm.put_str("HELLO ").flip().display().await.ok();
    // tm.put_str("HELLO ").display_rev().await.ok();
    // tm.put_str("HELLO ").flip().display_rev().await.ok();

    tm.put_str("HEL")
        .animate()
        .delay_ms(700)
        .direction(Direction::LeftToRight)
        .style(AnimationStyle::ToEnd)
        .display()
        .await
        .ok();

    // tm.put_str("HELLO ")
    //     .flip()
    //     .animate()
    //     .delay_ms(700)
    //     .direction(Direction::LeftToRight)
    //     .style(AnimationStyle::ToEnd)
    //     .display()
    //     .await
    //     .ok();

    loop {}
}
