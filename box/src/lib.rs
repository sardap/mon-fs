#![feature(io_error_more)]
pub mod box_mon;
pub mod file_pc;
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
        let mut bit_vec = BitVec::new();

        let size = size.into();
        let value: u64 = value.into();

        for i in 0..size {
            bit_vec.push(value & (1 << i as usize) != 0);
        }

        BoxMonBitVec(bit_vec)
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
        let mut raw = Vec::with_capacity((self.0.len() + 7) / 8);
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

        if bit != 0 {
            raw.push(byte);
        }

        raw
    }

    pub fn chunk(&self, start: usize, end: usize) -> BoxMonBitVec {
        Self(self.0.iter().skip(start).take(end - start).collect())
    }

    pub fn as_u8(&self) -> u8 {
        let mut result = 0;
        for i in 0..self.0.len() {
            if self.0[i] {
                result |= 1 << i;
            }
        }
        result
    }

    #[cfg(test)]
    pub fn as_u64(&self) -> u64 {
        let mut result = 0;
        for i in 0..self.0.len() {
            if self.0[i] {
                result |= 1 << i;
            }
        }
        result
    }
}

pub fn count_to_bits(n: usize) -> usize {
    (n as f64).log2().floor() as usize
}

#[cfg(test)]
mod test {
    use crate::BoxMonBitVec;

    #[test]
    fn test_to_raw() {
        let big_data = include_bytes!("../../test_assets/ricky.webp");

        let bits = BoxMonBitVec::new_from_raw(big_data);

        let decoded = bits.to_raw();

        assert_eq!(big_data.to_vec().len(), decoded.len());
        assert_eq!(big_data.to_vec(), decoded);
    }
}
