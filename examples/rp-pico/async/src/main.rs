#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Level, OutputOpenDrain},
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};
use embassy_time::{Delay, Duration, Timer};
use panic_probe as _;
use tm1637::{asynch::TM1637, demo::asynch::Demo};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let delay = Delay {};
    let clk = OutputOpenDrain::new(p.PIN_14, Level::Low);
    let dio = OutputOpenDrain::new(p.PIN_15, Level::Low);

    let driver = Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // Spin up some other tasks.
    // Decrease the delay_ms and the logs will be printed more frequently causing the display to lag.
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
