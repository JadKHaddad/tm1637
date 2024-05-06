# TM1637

A platform agnostic driver to interface with the `TM1637` (7-segment display) using the [`embedded-hal`](https://crates.io/crates/embedded-hal) and [`embedded-hal-async`](https://crates.io/crates/embedded-hal-async) traits.

## Features

The following features are available:

- `blocking`: enables blocking functionality.
- `async`: enables asynchronous functionality.
- `impl-debug`: implements `core::fmt::Debug` for structs and enums.
- `impl-defmt-format`: implements `defmt::Format` for structs and enums.
- `mappings`: enables the mappings module.
- `demo`: enables the demo module.
- `disable-checks`: disables bound checks while writing to the display. When enabled, positions greater than available positions on the display will be written to the display regardless, causing more delay than needed. Enable this feature only if you are sure about the positions you are writing to.

## Usage

See [examples](https://github.com/JadKHaddad/tm1637/tree/main/examples) directory or visit [wokwi.com](https://wokwi.com/projects/397159262874205185).

![example](https://github.com/JadKHaddad/tm1637/blob/main/assets/esp32c3-wokwi.gif?raw=true)

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
