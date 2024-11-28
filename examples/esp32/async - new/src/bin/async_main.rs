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
use tm1637_embedded_hal::{mappings::UpCharBits, Brightness, TM1637Builder};

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

    tm.display_str_rev(0, "Error").await.ok();

    loop {}
}
