use crate::{
    box_mon::StringMonParseError,
    mon_field::{BitCount, FromGameValueError, GameSerializer, ToGameValueError},
    BoxMonBitVec,
};
use serde_derive::{Deserialize, Serialize};

const PC_MARK_COUNT: usize = 4;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoxMonPCMark(pub [bool; PC_MARK_COUNT]);

impl TryFrom<Vec<bool>> for BoxMonPCMark {
    type Error = StringMonParseError;

    fn try_from(input: Vec<bool>) -> Result<Self, Self::Error> {
        if input.len() != 4 {
            return Err(StringMonParseError::InvalidPcMark);
        }

        let mut pc_mark = [false; PC_MARK_COUNT];
        for i in 0..PC_MARK_COUNT {
            pc_mark[i] = input[i];
        }

        Ok(BoxMonPCMark(pc_mark))
    }
}

impl BitCount for BoxMonPCMark {
    fn bit_count() -> usize {
        4
    }
}

impl GameSerializer for BoxMonPCMark {
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError>
    where
        Self: Sized,
    {
        if value.0.len() != Self::bit_count() {
            return Err(ToGameValueError::BadBitsLength);
        }

        let mut pc_mark = [false; PC_MARK_COUNT];
        for i in 0..PC_MARK_COUNT {
            pc_mark[i] = value.0[i];
        }

        Ok(BoxMonPCMark(pc_mark))
    }

    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError>
    where
        Self: Sized,
    {
        let mut bits = BoxMonBitVec::new(BoxMonPCMark::bit_count(), 0u64);
        for i in 0..PC_MARK_COUNT {
            bits.0.set(i, self.0[i]);
        }

        Ok(bits)
    }
}

#[cfg(test)]
mod tests {
    use bit_vec::BitVec;

    use crate::{
        mon_field::{BitCount, GameSerializer, PossibleValues},
        BoxMonBitVec,
    };

    use super::*;

    #[test]
    fn test_from_bits() {
        let mut bits = BitVec::new();
        for _ in 0..BoxMonPCMark::bit_count() {
            bits.push(false);
        }
        let pc_mark = BoxMonPCMark::bits_to_game_value(&BoxMonBitVec(bits)).unwrap();
        assert_eq!(BoxMonPCMark([false, false, false, false]), pc_mark);
    }

    #[test]
    fn test_to_and_from() {
        for i in 0..BoxMonPCMark::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonPCMark::bit_count(), i);
            let box_mon_gender = BoxMonPCMark::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }
}
