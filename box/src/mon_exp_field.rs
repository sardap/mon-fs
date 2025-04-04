use crate::{
    box_mon::StringMonParseError,
    count_to_bits,
    mon_field::{BitCount, FromGameValueError, GameSerializer, ToGameValueError},
    BoxMonBitVec,
};
use serde_derive::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoxMonExpField(pub u32);

const MAX_EXP: u32 = 524_288;

impl TryFrom<u32> for BoxMonExpField {
    type Error = StringMonParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > MAX_EXP {
            return Err(StringMonParseError::InvalidExpLength(value as usize));
        }

        Ok(BoxMonExpField(value))
    }
}

impl BitCount for BoxMonExpField {
    fn bit_count() -> usize {
        count_to_bits(MAX_EXP as usize)
    }
}

impl GameSerializer for BoxMonExpField {
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError>
    where
        Self: Sized,
    {
        let value = value.as_u64() as u32;
        match BoxMonExpField::try_from(value) {
            Ok(value) => Ok(value),
            Err(_) => Err(ToGameValueError::BitsValueOutOfRange),
        }
    }

    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError>
    where
        Self: Sized,
    {
        Ok(BoxMonBitVec::new(BoxMonExpField::bit_count(), self.0))
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
        for _ in 0..BoxMonExpField::bit_count() {
            bits.push(false);
        }
        let level = BoxMonExpField::bits_to_game_value(&BoxMonBitVec(bits)).unwrap();
        assert_eq!(BoxMonExpField(0), level);
    }

    #[test]
    fn test_to_and_from() {
        for i in 0..BoxMonExpField::possible_values() as u64 {
            let starting = BoxMonBitVec::new(BoxMonExpField::bit_count(), i);
            let box_mon_gender = BoxMonExpField::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u64());
        }
    }

    #[test]
    fn test_from_string() {
        assert_eq!(BoxMonExpField::try_from(1).unwrap(), BoxMonExpField(1));
        assert_eq!(
            BoxMonExpField::try_from(MAX_EXP).unwrap(),
            BoxMonExpField(MAX_EXP)
        );
    }
}
