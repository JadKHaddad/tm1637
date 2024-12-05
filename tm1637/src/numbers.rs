use crate::mappings::DigitBits;

pub fn u8(u: u8) -> [u8; 1] {
    let u = u.clamp(0, 9);

    [DigitBits::from_digit(u) as u8]
}

pub fn u8_2(u: u8) -> [u8; 2] {
    let u = u.clamp(0, 99);

    [
        DigitBits::from_digit(u / 10) as u8,
        DigitBits::from_digit(u % 10) as u8,
    ]
}

pub fn u16_3(u: u16) -> [u8; 3] {
    let u = u.clamp(0, 999);

    [
        DigitBits::from_digit((u / 100) as u8) as u8,
        DigitBits::from_digit(((u / 10) % 10) as u8) as u8,
        DigitBits::from_digit((u % 10) as u8) as u8,
    ]
}

pub fn u16_4(u: u16) -> [u8; 4] {
    let u = u.clamp(0, 9999);

    [
        DigitBits::from_digit((u / 1000) as u8) as u8,
        DigitBits::from_digit(((u / 100) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 10) % 10) as u8) as u8,
        DigitBits::from_digit((u % 10) as u8) as u8,
    ]
}

pub fn u32_5(u: u32) -> [u8; 5] {
    let u = u.clamp(0, 99999);

    [
        DigitBits::from_digit((u / 10000) as u8) as u8,
        DigitBits::from_digit(((u / 1000) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 100) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 10) % 10) as u8) as u8,
        DigitBits::from_digit((u % 10) as u8) as u8,
    ]
}

pub fn u32_6(u: u32) -> [u8; 6] {
    let u = u.clamp(0, 999999);

    [
        DigitBits::from_digit((u / 100000) as u8) as u8,
        DigitBits::from_digit(((u / 10000) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 1000) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 100) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 10) % 10) as u8) as u8,
        DigitBits::from_digit((u % 10) as u8) as u8,
    ]
}

pub fn u32_7(u: u32) -> [u8; 7] {
    let u = u.clamp(0, 9999999);

    [
        DigitBits::from_digit((u / 1000000) as u8) as u8,
        DigitBits::from_digit(((u / 100000) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 10000) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 1000) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 100) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 10) % 10) as u8) as u8,
        DigitBits::from_digit((u % 10) as u8) as u8,
    ]
}

pub fn u32_8(u: u32) -> [u8; 8] {
    let u = u.clamp(0, 99999999);

    [
        DigitBits::from_digit((u / 10000000) as u8) as u8,
        DigitBits::from_digit(((u / 1000000) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 100000) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 10000) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 1000) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 100) % 10) as u8) as u8,
        DigitBits::from_digit(((u / 10) % 10) as u8) as u8,
        DigitBits::from_digit((u % 10) as u8) as u8,
    ]
}
