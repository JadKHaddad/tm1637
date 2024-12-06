use crate::mappings::{DigitBits, SpecialCharBits};

// TODO: use a macro or something to generate these functions

pub fn u8(n: u8) -> [u8; 1] {
    let n = n.clamp(0, 9);

    [DigitBits::from_digit(n) as u8]
}

pub fn u8_2(n: u8) -> [u8; 2] {
    let n = n.clamp(0, 99);

    if n > 9 {
        [
            DigitBits::from_digit(n / 10) as u8,
            DigitBits::from_digit(n % 10) as u8,
        ]
    } else {
        [DigitBits::from_digit(n) as u8, 0]
    }
}

pub fn r_u8_2(n: u8) -> [u8; 2] {
    let n = n.clamp(0, 99);

    if n > 9 {
        [
            DigitBits::from_digit(n / 10) as u8,
            DigitBits::from_digit(n % 10) as u8,
        ]
    } else {
        [0, DigitBits::from_digit(n) as u8]
    }
}

pub fn u16_3(n: u16) -> [u8; 3] {
    let n = n.clamp(0, 999);

    if n > 99 {
        [
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9 {
        [
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
        ]
    } else {
        [DigitBits::from_digit((n % 10) as u8) as u8, 0, 0]
    }
}

pub fn r_u16_3(n: u16) -> [u8; 3] {
    let n = n.clamp(0, 999);

    if n > 99 {
        [
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9 {
        [
            0,
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else {
        [0, 0, DigitBits::from_digit((n % 10) as u8) as u8]
    }
}

pub fn u16_4(n: u16) -> [u8; 4] {
    let n = n.clamp(0, 9999);

    if n > 999 {
        [
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99 {
        [
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
        ]
    } else if n > 9 {
        [
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
        ]
    } else {
        [DigitBits::from_digit((n % 10) as u8) as u8, 0, 0, 0]
    }
}

pub fn r_u16_4(n: u16) -> [u8; 4] {
    let n = n.clamp(0, 9999);

    if n > 999 {
        [
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99 {
        [
            0,
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9 {
        [
            0,
            0,
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else {
        [0, 0, 0, DigitBits::from_digit((n % 10) as u8) as u8]
    }
}

pub fn u32_5(n: u32) -> [u8; 5] {
    let n = n.clamp(0, 99999);

    if n > 9999 {
        [
            DigitBits::from_digit((n / 10000) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 999 {
        [
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
        ]
    } else if n > 99 {
        [
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
        ]
    } else if n > 9 {
        [
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
        ]
    } else {
        [DigitBits::from_digit((n % 10) as u8) as u8, 0, 0, 0, 0]
    }
}

pub fn r_u32_5(n: u32) -> [u8; 5] {
    let n = n.clamp(0, 99999);

    if n > 9999 {
        [
            DigitBits::from_digit((n / 10000) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 999 {
        [
            0,
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99 {
        [
            0,
            0,
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9 {
        [
            0,
            0,
            0,
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else {
        [0, 0, 0, 0, DigitBits::from_digit((n % 10) as u8) as u8]
    }
}

pub fn u32_6(n: u32) -> [u8; 6] {
    let n = n.clamp(0, 999999);

    if n > 99999 {
        [
            DigitBits::from_digit((n / 100000) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9999 {
        [
            DigitBits::from_digit((n / 10000) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
        ]
    } else if n > 999 {
        [
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
        ]
    } else if n > 99 {
        [
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
        ]
    } else if n > 9 {
        [
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
            0,
        ]
    } else {
        [DigitBits::from_digit((n % 10) as u8) as u8, 0, 0, 0, 0, 0]
    }
}

pub fn r_u32_6(n: u32) -> [u8; 6] {
    let n = n.clamp(0, 999999);

    if n > 99999 {
        [
            DigitBits::from_digit((n / 100000) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9999 {
        [
            0,
            DigitBits::from_digit((n / 10000) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 999 {
        [
            0,
            0,
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99 {
        [
            0,
            0,
            0,
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9 {
        [
            0,
            0,
            0,
            0,
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else {
        [0, 0, 0, 0, 0, DigitBits::from_digit((n % 10) as u8) as u8]
    }
}

pub fn u32_7(n: u32) -> [u8; 7] {
    let n = n.clamp(0, 9999999);

    if n > 999999 {
        [
            DigitBits::from_digit((n / 1000000) as u8) as u8,
            DigitBits::from_digit(((n / 100000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99999 {
        [
            DigitBits::from_digit((n / 100000) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
        ]
    } else if n > 9999 {
        [
            DigitBits::from_digit((n / 10000) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
        ]
    } else if n > 999 {
        [
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
        ]
    } else if n > 99 {
        [
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
            0,
        ]
    } else if n > 9 {
        [
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
            0,
            0,
        ]
    } else {
        [
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
            0,
            0,
            0,
        ]
    }
}

pub fn r_u32_7(n: u32) -> [u8; 7] {
    let n = n.clamp(0, 9999999);

    if n > 999999 {
        [
            DigitBits::from_digit((n / 1000000) as u8) as u8,
            DigitBits::from_digit(((n / 100000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99999 {
        [
            0,
            DigitBits::from_digit((n / 100000) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9999 {
        [
            0,
            0,
            DigitBits::from_digit((n / 10000) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 999 {
        [
            0,
            0,
            0,
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99 {
        [
            0,
            0,
            0,
            0,
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9 {
        [
            0,
            0,
            0,
            0,
            0,
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else {
        [
            0,
            0,
            0,
            0,
            0,
            0,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    }
}

pub fn u32_8(n: u32) -> [u8; 8] {
    let n = n.clamp(0, 99999999);

    if n > 9999999 {
        [
            DigitBits::from_digit((n / 10000000) as u8) as u8,
            DigitBits::from_digit(((n / 1000000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 999999 {
        [
            DigitBits::from_digit((n / 1000000) as u8) as u8,
            DigitBits::from_digit(((n / 100000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
        ]
    } else if n > 99999 {
        [
            DigitBits::from_digit((n / 100000) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
        ]
    } else if n > 9999 {
        [
            DigitBits::from_digit((n / 10000) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
        ]
    } else if n > 999 {
        [
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
            0,
        ]
    } else if n > 99 {
        [
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
            0,
            0,
        ]
    } else if n > 9 {
        [
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
            0,
            0,
            0,
        ]
    } else {
        [
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]
    }
}

pub fn r_u32_8(n: u32) -> [u8; 8] {
    let n = n.clamp(0, 99999999);

    if n > 9999999 {
        [
            DigitBits::from_digit((n / 10000000) as u8) as u8,
            DigitBits::from_digit(((n / 1000000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 999999 {
        [
            0,
            DigitBits::from_digit((n / 1000000) as u8) as u8,
            DigitBits::from_digit(((n / 100000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99999 {
        [
            0,
            0,
            DigitBits::from_digit((n / 100000) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9999 {
        [
            0,
            0,
            0,
            DigitBits::from_digit((n / 10000) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 999 {
        [
            0,
            0,
            0,
            0,
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99 {
        [
            0,
            0,
            0,
            0,
            0,
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9 {
        [
            0,
            0,
            0,
            0,
            0,
            0,
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else {
        [
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    }
}

pub fn i8_2(n: i8) -> [u8; 2] {
    let n = n.clamp(-9, 99);

    if n > 0 {
        u8_2(n.unsigned_abs())
    } else {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n) as u8) as u8,
        ]
    }
}

pub fn i16_3(n: i16) -> [u8; 3] {
    let n = n.clamp(-99, 999);

    if n > 0 {
        u16_3(n.unsigned_abs())
    } else if n < -9 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
        ]
    }
}

pub fn r_i16_3(n: i16) -> [u8; 3] {
    let n = n.clamp(-99, 999);

    if n > 0 {
        r_u16_3(n.unsigned_abs())
    } else if n < -9 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else {
        [
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    }
}

pub fn i16_4(n: i16) -> [u8; 4] {
    let n = n.clamp(-999, 9999);

    if n > 0 {
        u16_4(n.unsigned_abs())
    } else if n < -99 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 100) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else if n < -9 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
        ]
    } else {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
            0,
        ]
    }
}

pub fn r_i16_4(n: i16) -> [u8; 4] {
    let n = n.clamp(-999, 9999);

    if n > 0 {
        r_u16_4(n.unsigned_abs())
    } else if n < -99 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 100) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else if n < -9 {
        [
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else {
        [
            0,
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    }
}

// TODO: more stuff
