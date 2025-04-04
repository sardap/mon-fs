use crate::{
    box_mon::StringMonParseError,
    count_to_bits,
    mon_field::{BitCount, FromGameValueError, GameSerializer, ToGameValueError},
    BoxMonBitVec,
};
use serde_derive::{Deserialize, Serialize};

const MAX_LEVEL: u8 = 64;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoxMonMetLevelField(pub u8);

impl TryFrom<u8> for BoxMonMetLevelField {
    type Error = StringMonParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 0 || value > MAX_LEVEL {
            return Err(StringMonParseError::InvalidExpLength(value as usize));
        }
        Ok(BoxMonMetLevelField(value))
    }
}

impl BitCount for BoxMonMetLevelField {
    fn bit_count() -> usize {
        count_to_bits(MAX_LEVEL as usize)
    }
}

impl GameSerializer for BoxMonMetLevelField {
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError>
    where
        Self: Sized,
    {
        let level = value.as_u8() + 1;
        if level > 100 {
            return Err(ToGameValueError::InvalidValue("Level is greater than 100"));
        }
        Ok(BoxMonMetLevelField(level))
    }

    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError>
    where
        Self: Sized,
    {
        Ok(BoxMonBitVec::new(
            BoxMonMetLevelField::bit_count(),
            self.0 - 1,
        ))
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
        for _ in 0..BoxMonMetLevelField::bit_count() {
            bits.push(false);
        }
        let level = BoxMonMetLevelField::bits_to_game_value(&BoxMonBitVec(bits)).unwrap();
        assert_eq!(BoxMonMetLevelField(1), level);
    }

    #[test]
    fn test_to_and_from() {
        for i in 0..BoxMonMetLevelField::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonMetLevelField::bit_count(), i);
            let box_mon_gender = BoxMonMetLevelField::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            BoxMonMetLevelField::try_from(1).unwrap(),
            BoxMonMetLevelField(1)
        );
        assert_eq!(
            BoxMonMetLevelField::try_from(35).unwrap(),
            BoxMonMetLevelField(35)
        );
    }
}
