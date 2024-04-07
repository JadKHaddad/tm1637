#![no_std]
#![no_main]

use panic_halt as _;
use rp2040_hal::gpio::{AnyPin, InOutPin};
use rp2040_hal::pac;
use tm1637::{demo::blocking::Demo, BlockingTM1637, Brightness, TM1637};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = rp2040_hal::Watchdog::new(pac.WATCHDOG);
    let clocks = rp2040_hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let sio = rp2040_hal::Sio::new(pac.SIO);

    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let clk = pins.gpio14.into_push_pull_output();
    let dio = InputOutputPin(InOutPin::new(pins.gpio15));
    let mut tm = TM1637::builder(clk, dio, timer).build();

    // Initialize the display.
    // Clear the display and set the initial brightness.
    tm.init().unwrap();

    // Change the brightness
    tm.write_brightness(Brightness::L3).unwrap();

    let mut demo = Demo::new(tm, timer, 500);
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

struct InputOutputPin<T: AnyPin>(InOutPin<T>);

impl<T: AnyPin> embedded_hal::digital::ErrorType for InputOutputPin<T> {
    type Error = rp2040_hal::gpio::Error;
}

impl<T: AnyPin> embedded_hal::digital::InputPin for InputOutputPin<T> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        embedded_hal_0_2::digital::v2::InputPin::is_high(&mut self.0)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        embedded_hal_0_2::digital::v2::InputPin::is_low(&mut self.0)
    }
}

impl<T: AnyPin> embedded_hal::digital::OutputPin for InputOutputPin<T> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        embedded_hal_0_2::digital::v2::OutputPin::set_low(&mut self.0)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        embedded_hal_0_2::digital::v2::OutputPin::set_high(&mut self.0)
    }
}
