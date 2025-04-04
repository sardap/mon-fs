use crate::{
    box_mon::StringMonParseError,
    count_to_bits,
    mon_field::{BitCount, FromGameValueError, GameSerializer, ToGameValueError},
    BoxMonBitVec,
};
use serde_derive::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoxMonRibbonsField(pub u8);

const MAX_RIBBONS: u8 = 16;

impl TryFrom<u8> for BoxMonRibbonsField {
    type Error = StringMonParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > MAX_RIBBONS {
            return Err(StringMonParseError::InvalidRibbons(value));
        }
        Ok(BoxMonRibbonsField(value as u8))
    }
}

impl BitCount for BoxMonRibbonsField {
    fn bit_count() -> usize {
        count_to_bits(MAX_RIBBONS as usize)
    }
}

impl GameSerializer for BoxMonRibbonsField {
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError>
    where
        Self: Sized,
    {
        let level = value.as_u8() + 1;
        if level > 100 {
            return Err(ToGameValueError::InvalidValue("Level is greater than 100"));
        }
        Ok(BoxMonRibbonsField(level))
    }

    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError>
    where
        Self: Sized,
    {
        Ok(BoxMonBitVec::new(
            BoxMonRibbonsField::bit_count(),
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
        for _ in 0..BoxMonRibbonsField::bit_count() {
            bits.push(false);
        }
        let level = BoxMonRibbonsField::bits_to_game_value(&BoxMonBitVec(bits)).unwrap();
        assert_eq!(BoxMonRibbonsField(1), level);
    }

    #[test]
    fn test_to_and_from() {
        for i in 0..BoxMonRibbonsField::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonRibbonsField::bit_count(), i);
            let box_mon_gender = BoxMonRibbonsField::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            BoxMonRibbonsField::try_from(1).unwrap(),
            BoxMonRibbonsField(1)
        );
        assert_eq!(
            BoxMonRibbonsField::try_from(11).unwrap(),
            BoxMonRibbonsField(11)
        );
    }
}
