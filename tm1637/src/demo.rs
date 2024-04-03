//! This module contains a demo implementation for the `TM1637` device.

#[cfg(feature = "blocking")]
pub mod blocking {
    use embedded_hal::{
        delay::DelayNs,
        digital::{InputPin, OutputPin},
    };

    use crate::{
        device::{brightness::Brightness, TM1637},
        functionality::{blocking::BlockingTM1637, TM1637Error},
        mappings::{DigitBits, LoCharBits, SegmentBits, SpecialCharBits, UpCharBits},
    };

    pub struct Demo<CLK, DIO, DELAY, ERR>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
        DELAY: DelayNs,
    {
        device: TM1637<CLK, DIO, DELAY>,
        delay: DELAY,
        moving_delay_ms: u32,
    }

    impl<CLK, DIO, DELAY, ERR> Demo<CLK, DIO, DELAY, ERR>
    where
        ERR: core::fmt::Debug,
        CLK: OutputPin<Error = ERR>,
        DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
        DELAY: DelayNs,
    {
        pub fn new(device: TM1637<CLK, DIO, DELAY>, delay: DELAY, moving_delay_ms: u32) -> Self {
            Self {
                device,
                delay,
                moving_delay_ms,
            }
        }

        /// Clear the display.
        pub fn clear(&mut self) -> Result<(), TM1637Error<ERR>> {
            self.device.clear()
        }

        /// Move all segments across the display.
        pub fn moving_segments(&mut self) -> Result<(), TM1637Error<ERR>> {
            let all_seg_bits = &mut SegmentBits::all_u8()[0..7];
            for _ in 0..7 {
                all_seg_bits.rotate_left(1);
                self.device.write_raw(0, &all_seg_bits)?;
                self.delay.delay_ms(self.moving_delay_ms);
            }

            Ok(())
        }

        /// Move all digits across the display.
        pub fn moving_digits(&mut self) -> Result<(), TM1637Error<ERR>> {
            let mut all_dig_bits = DigitBits::all_u8();
            for _ in 0..10 {
                all_dig_bits.rotate_left(1);
                self.device.write_raw(0, &all_dig_bits)?;
                self.delay.delay_ms(self.moving_delay_ms);
            }

            Ok(())
        }

        /// Move all uppercase characters across the display.
        pub fn moving_up_chars(&mut self) -> Result<(), TM1637Error<ERR>> {
            let mut all_up_char_bits = UpCharBits::all_u8();
            for _ in 0..13 {
                all_up_char_bits.rotate_left(1);
                self.device.write_raw(0, &all_up_char_bits)?;
                self.delay.delay_ms(self.moving_delay_ms);
            }

            Ok(())
        }

        /// Move all lowercase characters across the display.
        pub fn moving_lo_chars(&mut self) -> Result<(), TM1637Error<ERR>> {
            let mut all_lo_char_bits = LoCharBits::all_u8();
            for _ in 0..12 {
                all_lo_char_bits.rotate_left(1);
                self.device.write_raw(0, &all_lo_char_bits)?;
                self.delay.delay_ms(self.moving_delay_ms);
            }

            Ok(())
        }

        /// Move all special characters across the display.
        pub fn moving_special_chars(&mut self) -> Result<(), TM1637Error<ERR>> {
            let mut all_sp_char_bits = SpecialCharBits::all_u8();
            for _ in 0..12 {
                all_sp_char_bits.rotate_left(1);
                self.device.write_raw(0, &all_sp_char_bits)?;
                self.delay.delay_ms(self.moving_delay_ms);
            }

            Ok(())
        }

        /// Turn the display on and off.
        pub fn on_off(
            &mut self,
            brightness: Brightness,
            cycles: u32,
            of_off_delay_ms: u32,
        ) -> Result<(), TM1637Error<ERR>> {
            for _ in 0..cycles {
                self.device.set_brightness(Brightness::OFF)?;
                self.delay.delay_ms(of_off_delay_ms);
                self.device.set_brightness(brightness)?;
                self.delay.delay_ms(of_off_delay_ms);
            }

            Ok(())
        }

        /// Display the time and make the dots blink.
        ///
        /// Displays 19:06 with blinking dots.
        pub fn time(&mut self, cycles: u32, blink_delay_ms: u32) -> Result<(), TM1637Error<ERR>> {
            self.device.write_raw(
                0,
                &[
                    DigitBits::One as u8,
                    DigitBits::Nine as u8 | SegmentBits::SegPoint as u8,
                    DigitBits::Zero as u8,
                    DigitBits::Six as u8,
                ],
            )?;

            let mut show = true;
            for _ in 0..cycles {
                let bit = match show {
                    true => DigitBits::Nine as u8 | SegmentBits::SegPoint as u8,
                    false => DigitBits::Nine as u8,
                };

                self.device.write_raw(1, &[bit])?;

                self.delay.delay_ms(blink_delay_ms);

                show = !show;
            }

            Ok(())
        }

        /// Create a rotating circle animation.
        ///
        /// Creates a rotating circle at address 0.
        pub fn rotating_circle(
            &mut self,
            cycles: u32,
            rotating_delay_ms: u32,
        ) -> Result<(), TM1637Error<ERR>> {
            // First of all we create the shapes want to animate

            //  ---
            // |   |
            // |
            //  ---
            // This shape consists of these segments: B, A, F, E and D.
            // Let's create the shape
            let shape_1 = SegmentBits::SegB as u8
                | SegmentBits::SegA as u8
                | SegmentBits::SegF as u8
                | SegmentBits::SegE as u8
                | SegmentBits::SegD as u8;

            //  ---
            // |
            // |   |
            //  ---
            // This shape consists of these segments: A, F, E, D and C.
            let shape_2 = SegmentBits::SegA as u8
                | SegmentBits::SegF as u8
                | SegmentBits::SegE as u8
                | SegmentBits::SegD as u8
                | SegmentBits::SegC as u8;

            // and so on...

            //
            // |   |
            // |   |
            //  ---
            let shape_3 = SegmentBits::SegF as u8
                | SegmentBits::SegE as u8
                | SegmentBits::SegD as u8
                | SegmentBits::SegC as u8
                | SegmentBits::SegB as u8;

            //  ---
            //     |
            // |   |
            //  ---
            let shape_4 = SegmentBits::SegE as u8
                | SegmentBits::SegD as u8
                | SegmentBits::SegC as u8
                | SegmentBits::SegB as u8
                | SegmentBits::SegA as u8;

            //  ---
            // |   |
            //     |
            //  ---
            let shape_5 = SegmentBits::SegD as u8
                | SegmentBits::SegC as u8
                | SegmentBits::SegB as u8
                | SegmentBits::SegA as u8
                | SegmentBits::SegF as u8;

            //  ---
            // |   |
            // |   |
            //
            let shape_6 = SegmentBits::SegC as u8
                | SegmentBits::SegB as u8
                | SegmentBits::SegA as u8
                | SegmentBits::SegF as u8
                | SegmentBits::SegE as u8;

            let mut shapes = [shape_1, shape_2, shape_3, shape_4, shape_5, shape_6];
            for _ in 0..cycles {
                shapes.rotate_left(1);
                self.device.write_raw(0, &shapes[0..1]).unwrap();
                self.delay.delay_ms(rotating_delay_ms);
            }

            Ok(())
        }
    }
}
