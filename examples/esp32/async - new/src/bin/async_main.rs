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
use tm1637_embedded_hal::asynch::TM1637;

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

    let mut tm = TM1637::builder(clk, dio, delay).build();

    tm.init().await.unwrap();
    tm.write_ascii_str(0, "UP  ").await.unwrap();
    tm.write_ascii_str(0, "HO  ").await.unwrap();

    loop {}
}
