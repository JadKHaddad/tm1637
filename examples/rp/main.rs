#![no_std]
#![no_main]

use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use panic_halt as _;
use rp2040_hal::{clocks::init_clocks_and_plls, entry, pac, sio::Sio, watchdog::Watchdog, Timer};
use tm1637::{demo::blocking::Demo, BlockingTM1637, Brightness, TM1637};

#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
    const XOSC_CRYSTAL_FREQ: u32 = 12_000_000; // Typically found in BSP crates
    let mut clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);
    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio25.into_push_pull_output();
    led_pin.set_high().unwrap();

    let delay = Timer::new(pac.TIMER, &mut pac.RESETS, &mut clocks);
    let clk = pins.gpio14.into_push_pull_output();
    let dio = rp2040_hal::gpio::InOutPin::new(pins.gpio15);

    let mut tm = TM1637::builder(clk, dio, delay).build();

    // Initialize the display.
    // Clear the display and set the initial brightness.
    tm.init().unwrap();

    // Change the brightness
    tm.write_brightness(Brightness::L3).unwrap();

    let mut demo = Demo::new(tm, delay, 500);
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
