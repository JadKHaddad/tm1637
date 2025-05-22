//! For more usage examples, see esp32/4-digits.

use ftdi_embedded_hal::ftdi;
use linux_embedded_hal::Delay;
use tm1637_embedded_hal::{Brightness, TM1637Builder};

fn main() -> Result<(), anyhow::Error> {
    use ftdi_embedded_hal as hal;

    // Note: Some adapters might have a different USB VID/PID
    let device = ftdi::find_by_vid_pid(0x0403, 0x6014)
        .interface(ftdi::Interface::A)
        .open()?;

    let hal = hal::FtHal::init_default(device)?;

    let delay = Delay;
    let clk = hal.ad6()?;
    // AD7 is an output-only pin.
    let dio = hal.ad7()?;

    // Create a TM1637 instance with 4 digits.
    let mut tm = TM1637Builder::new(clk, dio, delay)
        // Set the brightness to level 3.
        .brightness(Brightness::L3)
        // Set the delay between each bit to 100us. Experiment with this value to find the best value for your display.
        .delay_us(100)
        .build_blocking::<4>();

    // Initialize the display.
    // Clear the display and set the initial brightness.
    tm.init().ok();

    tm.options().str("ruSt").display().ok();

    Ok(())
}
