use ftdi_embedded_hal::ftdi;
use tm1637_embedded_hal::{blocking::TM1637, demo::blocking::Demo, Brightness};

fn main() -> Result<(), anyhow::Error> {
    use ftdi_embedded_hal as hal;

    // Note: Some adapters might have a different USB VID/PID
    let device = ftdi::find_by_vid_pid(0x0403, 0x6014)
        .interface(ftdi::Interface::A)
        .open()?;

    let hal = hal::FtHal::init_default(device)?;

    let tm = TM1637::new(
        hal.ad6()?, // CLK pin.
        hal.ad7()?, // DIO pin. Note: AD7 is an output-only pin.
        linux_embedded_hal::Delay,
        Brightness::L3,
        10, // clock delay, µs
        4,  // digits
    );

    let mut demo = Demo::new(tm, linux_embedded_hal::Delay, 500);
    loop {
        demo.rotating_circle(20, 200).unwrap();
        demo.time(10, 500).unwrap();
        demo.on_off(10, 200).unwrap();
        demo.moving_segments().unwrap();
        demo.moving_digits().unwrap();
        demo.countdown().unwrap();
        demo.moving_up_chars().unwrap();
        demo.moving_lo_chars().unwrap();
        demo.moving_special_chars().unwrap();
    }
}
