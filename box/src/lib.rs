#![feature(io_error_more)]
use core::panic;
pub mod box_mon;
pub mod marker_mon;
pub mod mon_captured_ball;
pub mod mon_field;
pub mod mon_gender;
pub mod mon_held_item;
pub mod mon_name;
pub mod mon_species;
pub mod pc;
use bit_vec::BitVec;

#[derive(Debug, Clone, Default)]
pub struct BoxMonBitVec(pub BitVec);

impl PartialEq for BoxMonBitVec {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for BoxMonBitVec {}

impl BoxMonBitVec {
    pub fn new<T: Into<usize>, J: Into<u64>>(size: T, value: J) -> BoxMonBitVec {
        let mut x = BitVec::new();

        let size = size.into();
        let value: u64 = value.into();

        for i in 0..size {
            x.push(value & (1 << i as usize) != 0);
        }

        BoxMonBitVec(x)
    }

    pub fn new_from_raw(raw: &[u8]) -> BoxMonBitVec {
        let mut x: BitVec = BitVec::new();
        for byte in raw {
            for i in 0..8 {
                x.push(byte & (1 << i) != 0);
            }
        }
        BoxMonBitVec(x)
    }

    pub fn to_raw(&self) -> Vec<u8> {
        let mut raw = Vec::new();
        let mut byte = 0;
        let mut bit = 0;
        for i in 0..self.0.len() {
            if self.0[i] {
                byte |= 1 << bit;
            }
            bit += 1;
            if bit == 8 {
                raw.push(byte);
                byte = 0;
                bit = 0;
            }
        }

        raw
    }

    pub fn chunk(&self, start: usize, end: usize) -> BoxMonBitVec {
        Self(self.0.iter().skip(start).take(end - start).collect())
    }

    pub fn as_u8(&self) -> u8 {
        let mut x = 0;
        for i in 0..self.0.len() {
            if self.0[i] {
                x |= 1 << i;
            }
        }
        x
    }

    pub fn as_u16(&self) -> u16 {
        let mut x = 0;
        for i in 0..self.0.len() {
            if self.0[i] {
                x |= 1 << i;
            }
        }
        x
    }

    pub fn as_u64(&self) -> u64 {
        let mut x = 0;
        for i in 0..self.0.len() {
            if self.0[i] {
                x |= 1 << i;
            }
        }
        x
    }
}

pub const fn count_to_bits(n: usize) -> usize {
    if n == 0 {
        0
    } else if n <= 3 {
        1
    } else if n <= 7 {
        2
    } else if n <= 15 {
        3
    } else if n <= 31 {
        4
    } else if n <= 63 {
        5
    } else if n <= 127 {
        6
    } else {
        panic!("Invalid field size");
    }

    // if n <= 6 {
    //     1
    // } else if n <= 7 {
    //     2
    // } else if n <= 15 {
    //     3
    // } else if n <= 31 {
    //     4
    // } else if n <= 63 {
    //     5
    // } else if n <= 127 {
    //     6
    // } else {
    //     panic!("Invalid field size");
    // }
}

#[cfg(test)]
mod test {
    use crate::BoxMonBitVec;

    #[test]
    fn test_to_raw() {
        let big_data = include_bytes!("../../test_assets/p.webp");

        let bits = BoxMonBitVec::new_from_raw(big_data);

        let decoded = bits.to_raw();

        assert_eq!(big_data.to_vec().len(), decoded.len());
        assert_eq!(big_data.to_vec(), decoded);
    }
}
