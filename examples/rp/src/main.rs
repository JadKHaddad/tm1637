#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, OutputOpenDrain};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::Delay;
use tm1637::{
    demo::blocking::Demo,
    device::{brightness::Brightness, TM1637},
    functionality::blocking::BlockingTM1637,
};
use {defmt_rtt as _, panic_probe as _};

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

    let mut tm = TM1637::builder(clk, dio, delay).build();

    // initialize the display. clears the display and sets the initial brightness.
    tm.init().unwrap();
    // change the brightness
    tm.write_brightness(Brightness::L3).unwrap();

    let mut demo = Demo::new(tm, Delay {}, 500);
    loop {
        demo.rotating_circle(20, 200).unwrap();
        demo.time(10, 500).unwrap();
        demo.on_off(10, 200).unwrap();
        demo.moving_segments().unwrap();
        demo.moving_digits().unwrap();
        demo.moving_up_chars().unwrap();
        demo.moving_lo_chars().unwrap();
        demo.moving_special_chars().unwrap();
    }
}
