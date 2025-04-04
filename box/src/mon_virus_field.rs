use serde::{Deserialize, Serialize};

use crate::mon_field::{BitCount, GameSerializer};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoxMonVirusField(pub bool);

impl From<bool> for BoxMonVirusField {
    fn from(value: bool) -> Self {
        BoxMonVirusField(value)
    }
}

impl GameSerializer for BoxMonVirusField {
    fn bits_to_game_value(
        value: &crate::BoxMonBitVec,
    ) -> Result<Self, crate::mon_field::ToGameValueError>
    where
        Self: Sized,
    {
        Ok(BoxMonVirusField(value.as_u8() != 0))
    }

    fn game_value_to_bits(
        &self,
    ) -> Result<crate::BoxMonBitVec, crate::mon_field::FromGameValueError>
    where
        Self: Sized,
    {
        let mut bits = crate::BoxMonBitVec::new(Self::bit_count(), 0u64);
        bits.0.set(0, self.0);
        Ok(bits)
    }
}

impl BitCount for BoxMonVirusField {
    fn bit_count() -> usize {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mon_field::{BitCount, GameSerializer, PossibleValues},
        BoxMonBitVec,
    };

    use super::*;

    #[test]
    fn test_to_and_from() {
        for i in 0..BoxMonVirusField::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonVirusField::bit_count(), i);
            let box_mon_shiny = BoxMonVirusField::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_shiny.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }

    #[test]
    fn test_from_string() {
        assert_eq!(BoxMonVirusField::from(true), BoxMonVirusField(true));
    }
}
