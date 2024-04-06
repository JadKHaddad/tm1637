#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, embassy, peripherals::Peripherals, prelude::*, timer::TimerGroup, IO,
};
use tm1637::{demo::asynchronous::Demo, AsyncTM1637, TM1637};

#[main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let delay = Delay {};
    let clk = io.pins.gpio4.into_open_drain_output();
    let dio = io.pins.gpio19.into_open_drain_output();

    embassy::init(&clocks, timg0);

    // Spin up some other tasks.
    // Decrease the delay_ms and the logs will be printed more frequently causing the display to lag, but at least it's not blocking :>
    spawner.spawn(logger0(2000)).unwrap();
    spawner.spawn(logger1(2000)).unwrap();
    spawner.spawn(logger2(2000)).unwrap();

    let mut tm = TM1637::builder(clk, dio, delay).build();

    // Initialize the display.
    // Clear the display and set the initial brightness.
    tm.init().await.unwrap();

    let delay = Delay {};
    let mut demo = Demo::new(tm, delay);
    loop {
        demo.timer().await.unwrap();
    }
}

#[embassy_executor::task]
async fn logger0(deley_ms: u64) {
    loop {
        log::info!("Hi from logger0");
        Timer::after(Duration::from_millis(deley_ms)).await;
    }
}

#[embassy_executor::task]
async fn logger1(deley_ms: u64) {
    loop {
        log::info!("Hi from logger1");
        Timer::after(Duration::from_millis(deley_ms)).await;
    }
}

#[embassy_executor::task]
async fn logger2(deley_ms: u64) {
    loop {
        log::info!("Hi from logger2");
        Timer::after(Duration::from_millis(deley_ms)).await;
    }
}
