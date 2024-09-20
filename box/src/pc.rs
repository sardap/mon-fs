use crate::box_mon::BoxMon;
use crate::mon_field::{BitCount, ByteCount, GameSerializer};
use crate::BoxMonBitVec;
use bit_vec::BitVec;
use serde_derive::{Deserialize, Serialize};

const PC_BOX_SIZE: usize = 30;
const NUM_PC_BOXES: usize = 14;
pub const NUM_OF_MONS: usize = PC_BOX_SIZE * NUM_PC_BOXES;
const NUM_OF_DATA_MONS: usize = NUM_OF_MONS - 2;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PC {
    pub mons: Vec<Option<BoxMon>>,
    #[serde(skip)]
    current_read_offset: usize,
    #[serde(skip)]
    raw_cache: Option<BoxMonBitVec>,
}

impl BitCount for PC {
    fn bit_count() -> usize {
        return (BoxMon::byte_count() * NUM_OF_DATA_MONS) * 8;
    }
}

impl PC {
    pub fn new() -> PC {
        PC {
            mons: vec![None; NUM_OF_MONS],
            current_read_offset: 0,
            raw_cache: None,
        }
    }

    pub fn fill_empty_mon_slots(&mut self) {
        while self.mons.len() < NUM_OF_MONS {
            self.mons.push(None);
        }
    }

    pub fn set_padding_amount(&mut self, padding_amount: u8) {
        let mut bits = BitVec::from_elem(8, false);

        for i in 0..8 {
            bits.set(i, padding_amount & (1 << i) != 0);
        }

        while bits.len() < BoxMon::bit_count() {
            bits.push(false);
        }

        self.mons[0] = Some(BoxMon::bits_to_game_value(&BoxMonBitVec(bits)).unwrap());
    }

    pub fn get_padding_amount(&self) -> u8 {
        let mon = self.mons[0].as_ref().unwrap();
        let bits = mon.game_value_to_bits().unwrap();
        let mut padding = 0;
        for i in 0..8 {
            if bits.0[i] {
                padding |= 1 << i;
            }
        }
        padding
    }

    pub fn set_mon(&mut self, box_index: usize, mon_index: usize, mon: BoxMon) {
        let index = box_index * PC_BOX_SIZE + mon_index;
        self.mons[index] = Some(mon);
        self.raw_cache = None;
    }

    fn get_empty_offset(&self) -> usize {
        for offset in 1..self.mons.len() {
            if self.mons[offset].is_none() {
                return offset;
            }
        }

        return self.mons.len() - 1;
    }

    fn get_data(&mut self) -> &BoxMonBitVec {
        if self.raw_cache.is_none() {
            let last_mon_index = self.get_empty_offset();
            let mut fat: BitVec = BitVec::new();
            for i in 1..last_mon_index + 1 {
                // Skip the padding mon
                match self.mons[i] {
                    Some(mon) => {
                        let bits = mon.game_value_to_bits().unwrap();
                        if i == last_mon_index - 1 {
                            let padding_amount = self.get_padding_amount();
                            for i in 0..BoxMon::bit_count() - padding_amount as usize {
                                fat.push(bits.0[i]);
                            }
                        } else {
                            fat.extend(bits.0);
                        }
                    }
                    None => break,
                }
            }

            self.raw_cache = Some(BoxMonBitVec(fat));
        }

        self.raw_cache.as_ref().unwrap()
    }

    pub fn remaining_bytes(&self) -> usize {
        let current_offset = self.get_empty_offset();
        let remaining_bytes = (NUM_OF_DATA_MONS - (current_offset - 1)) * BoxMon::byte_count();
        remaining_bytes
    }
}

impl std::io::Write for PC {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let bits: BoxMonBitVec = BoxMonBitVec::new_from_raw(buf);

        assert_eq!(buf.len(), bits.0.len() / 8);

        let mut offset = 0;

        let mut current_offset = self.get_empty_offset();
        if self.remaining_bytes() < buf.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "Not enough space in PC have {} need {}",
                    self.remaining_bytes(),
                    buf.len()
                ),
            ));
        }

        self.raw_cache = None;

        if current_offset > 1 {
            let padding_amount = self.get_padding_amount();
            let mut last_mon_bits = self.mons[current_offset - 1]
                .unwrap()
                .game_value_to_bits()
                .unwrap();

            let start_offset = BoxMon::bit_count() - padding_amount as usize;
            let end_offset = BoxMon::bit_count().min(bits.0.len());
            // Replace padding data with
            for i in start_offset..end_offset {
                last_mon_bits.0.set(i, bits.0[i]);
            }

            if padding_amount as usize > bits.0.len() {
                self.set_padding_amount(padding_amount - bits.0.len() as u8);
                return Ok(buf.len());
            }

            self.set_padding_amount(0);

            // Last mon padding data has been filled now we can treat it as a normal mon
            current_offset += 1;
        }

        loop {
            let end_offset = (offset + BoxMon::bit_count()).min(bits.0.len());

            let mut chunk = bits.chunk(offset, end_offset);
            let surplus_bits = BoxMon::bit_count() - chunk.0.len();
            while chunk.0.len() < BoxMon::bit_count() {
                chunk.0.push(false);
            }

            let mon = BoxMon::bits_to_game_value(&chunk).unwrap();
            self.mons[current_offset] = Some(mon);
            current_offset += 1;

            let amount_left = bits.0.len() - end_offset;
            if amount_left == 0 {
                self.set_padding_amount(surplus_bits as u8);

                return Ok(end_offset / 8);
            }
            offset = end_offset;
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl std::io::Read for PC {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let current_read_offset = self.current_read_offset;
        let data = self.get_data();

        let end = (current_read_offset + buf.len()).min(data.0.len());

        let fat = data.chunk(current_read_offset * 8, end * 8);
        let fat = fat.to_raw();

        for i in 0..fat.len() {
            buf[i] = fat[i];
        }

        self.current_read_offset += fat.len();

        Ok(fat.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Read;
    use std::io::Write;

    #[test]
    fn write_arbitrary_data_to_pc() {
        let mut pc = PC::new();

        let huge_amount_of_data = include_bytes!("../../test_assets/ricky.webp").to_vec();

        pc.write(&huge_amount_of_data).unwrap();

        let mut buf = Vec::new();
        pc.read_to_end(&mut buf).unwrap();

        assert_eq!(huge_amount_of_data, buf);
    }

    #[test]
    fn ensure_normal_amount_of_bits() {
        assert_eq!(PC::bit_count() % 8, 0);
    }

    #[test]
    fn byte_count_accuracy() {
        let pc = PC::new();
        assert_eq!(PC::byte_count(), pc.remaining_bytes());
    }

    #[test]
    fn completely_fill_pc() {
        let mut pc = PC::new();

        let mut data = vec![0; PC::byte_count()];
        for i in 0..data.len() {
            data[i] = (i % 255) as u8;
        }

        pc.write(&data).unwrap();

        let mut buf = Vec::new();
        pc.read_to_end(&mut buf).unwrap();

        assert_eq!(data, buf);
    }
}
