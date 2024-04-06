#![no_std]
#![no_main]

use bsp::{entry, hal::gpio::InOutPin};
use defmt::*;
use defmt_rtt as _;
use embedded_hal_old::digital::v2::{InputPin, OutputPin};
use panic_probe as _;
// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use tm1637::{demo::blocking::Demo, mappings::DigitBits, BlockingTM1637, Brightness, TM1637};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = MyDelay(cortex_m::delay::Delay::new(
        core.SYST,
        clocks.system_clock.freq().to_Hz(),
    ));

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let clk = MyOutput(pins.gpio14.into_push_pull_output());
    let dio = MyInputOutput(InOutPin::new(pins.gpio15));

    let mut tm = TM1637::builder(clk, dio, delay).build();

    // Initialize the display.
    // Clear the display and set the initial brightness.
    tm.init().unwrap();

    // Change the brightness
    tm.write_brightness(Brightness::L3).unwrap();

    let mut all_dig_bits = [0; 16];
    tm.write_segments_raw(0, &DigitBits::all_u8()).unwrap();

    loop {}

    // let mut demo = Demo::new(tm, delay, 500);
    // loop {
    //     demo.rotating_circle(20, 200).unwrap();
    //     demo.time(10, 500).unwrap();
    //     demo.on_off(10, 200).unwrap();
    //     demo.moving_segments().unwrap();
    //     demo.moving_digits().unwrap();
    //     demo.moving_up_chars().unwrap();
    //     demo.moving_lo_chars().unwrap();
    //     demo.moving_special_chars().unwrap();
    // }
}

struct MyDelay(cortex_m::delay::Delay);

impl embedded_hal::delay::DelayNs for MyDelay {
    fn delay_ns(&mut self, ns: u32) {
        self.0.delay_us(ns / 1000);
    }
}

struct MyInputOutput<INOUT>(INOUT);

impl<INOUT> embedded_hal::digital::ErrorType for MyInputOutput<INOUT> {
    type Error = core::convert::Infallible;
}

impl<INOUT> embedded_hal::digital::InputPin for MyInputOutput<INOUT>
where
    INOUT: embedded_hal_old::digital::v2::InputPin,
    <INOUT as embedded_hal_old::digital::v2::InputPin>::Error: core::fmt::Debug,
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.0.is_high().unwrap())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.0.is_low().unwrap())
    }
}

impl<INOUT> embedded_hal::digital::OutputPin for MyInputOutput<INOUT>
where
    INOUT: embedded_hal_old::digital::v2::OutputPin,
    <INOUT as embedded_hal_old::digital::v2::OutputPin>::Error: core::fmt::Debug,
{
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(self.0.set_low().unwrap())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(self.0.set_high().unwrap())
    }
}

struct MyOutput<OUT>(OUT);

impl<OUT> embedded_hal::digital::ErrorType for MyOutput<OUT> {
    type Error = core::convert::Infallible;
}

impl<OUT> embedded_hal::digital::OutputPin for MyOutput<OUT>
where
    OUT: embedded_hal_old::digital::v2::OutputPin,
    <OUT as embedded_hal_old::digital::v2::OutputPin>::Error: core::fmt::Debug,
{
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(self.0.set_low().unwrap())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(self.0.set_high().unwrap())
    }
}

fn cl<CLK, ERR>(clk: CLK) -> Result<(), ERR>
where
    CLK: embedded_hal::digital::OutputPin<Error = ERR>,
{
    Ok(())
}

fn fun<CLK, DIO, DELAY, ERR>(clk: CLK, dio: DIO, delay: DELAY) -> Result<(), ERR>
where
    CLK: embedded_hal::digital::OutputPin<Error = ERR>,
    DIO: embedded_hal::digital::OutputPin<Error = ERR>
        + embedded_hal::digital::InputPin<Error = ERR>,
    DELAY: embedded_hal::delay::DelayNs,
{
    Ok(())
}

// End of file

// let clk = pins.gpio14.into_push_pull_output();
// let dio = rp2040_hal::gpio::InOutPin::new(pins.gpio15);

// let mut tm = TM1637::builder(clk, dio, delay).build();

// // Initialize the display.
// // Clear the display and set the initial brightness.
// tm.init().unwrap();

// // Change the brightness
// tm.write_brightness(Brightness::L3).unwrap();

// let mut demo = Demo::new(tm, delay, 500);
// loop {
//     demo.rotating_circle(20, 200).unwrap();
//     demo.time(10, 500).unwrap();
//     demo.on_off(10, 200).unwrap();
//     demo.moving_segments().unwrap();
//     demo.moving_digits().unwrap();
//     demo.moving_up_chars().unwrap();
//     demo.moving_lo_chars().unwrap();
//     demo.moving_special_chars().unwrap();
// }
