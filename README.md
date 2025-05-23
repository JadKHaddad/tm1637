# TM1637

![Build Status](https://github.com/JadKHaddad/tm1637/actions/workflows/build-and-test.yaml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/tm1637-embedded-hal.svg)](https://crates.io/crates/tm1637-embedded-hal)
[![Crates.io (MSRV)](https://img.shields.io/crates/msrv/tm1637-embedded-hal)](https://crates.io/crates/tm1637-embedded-hal)
[![docs.rs](https://docs.rs/tm1637-embedded-hal/badge.svg)](https://docs.rs/tm1637-embedded-hal)
[![Crates.io (Downloads)](https://img.shields.io/crates/d/tm1637-embedded-hal)](https://crates.io/crates/tm1637-embedded-hal)
[![Crates.io (License)](https://img.shields.io/crates/l/tm1637-embedded-hal)](https://crates.io/crates/tm1637-embedded-hal)

A platform agnostic driver to interface with the `TM1637` (7-segment display) using the [`embedded-hal`](https://crates.io/crates/embedded-hal) and [`embedded-hal-async`](https://crates.io/crates/embedded-hal-async) traits.

!["ruSt" on a 4-digit display](https://github.com/JadKHaddad/tm1637/blob/main/assets/4digits-rust.webp?raw=true)

## Features

- `ack`: Enables the driver to use the [`InputPin`](https://docs.rs/embedded-hal/latest/embedded_hal/digital/trait.InputPin.html) trait for the `DIO` pin and wait for the acknowledgment signal from the display.
- `defmt`: Implements [`defmt::Format`](https://docs.rs/defmt/latest/defmt/trait.Format.html) for structs and enums.

## Usage

See [examples](https://github.com/JadKHaddad/tm1637/tree/main/examples) directory or visit [wokwi.com](https://wokwi.com/projects/397159262874205185).

![example](https://github.com/JadKHaddad/tm1637/blob/main/assets/esp32c3-wokwi.gif?raw=true)

## Demo

See [demo](DEMO.md).

## Other Repositories

- `Rust` [generic-tm1637-gpio-driver-rust](https://github.com/phip1611/generic-tm1637-gpio-driver-rust)
- `Rust` [tm1637-rs](https://github.com/igelbox/tm1637-rs)
- `C` [TM1637-Driver](https://github.com/AlexAlexFr/TM1637-Driver)
- `Arduino` [TM1637](https://github.com/avishorp/TM1637)

## License

Licensed under either of

- Apache License, Version 2.0. [LICENSE-APACHE](LICENSE-APACHE) or [Apache-2.0 license](http://apache.org/licenses/LICENSE-2.0)
- MIT license. [LICENSE-MIT](LICENSE-MIT) or [MIT license](http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
