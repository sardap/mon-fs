use serde::{Deserialize, Serialize};

use crate::{
    box_mon::StringMonParseError,
    mon_field::{BitCount, FromGameValueError, GameSerializer, ToGameValueError},
    BoxMonBitVec,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoxMonOtTidField(pub u16);

impl TryFrom<u16> for BoxMonOtTidField {
    type Error = StringMonParseError;

    fn try_from(input: u16) -> Result<Self, Self::Error> {
        Ok(BoxMonOtTidField(input))
    }
}

impl BitCount for BoxMonOtTidField {
    fn bit_count() -> usize {
        16
    }
}

impl GameSerializer for BoxMonOtTidField {
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError>
    where
        Self: Sized,
    {
        if value.0.len() != Self::bit_count() {
            return Err(ToGameValueError::BadBitsLength);
        }

        Ok(BoxMonOtTidField(value.as_u64() as u16))
    }

    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError>
    where
        Self: Sized,
    {
        Ok(BoxMonBitVec::new(BoxMonOtTidField::bit_count(), self.0))
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use crate::{
        mon_field::{GameSerializer, PossibleValues},
        BoxMonBitVec,
    };

    use super::*;

    #[test]
    fn test_move_set_to_and_from() {
        let count = BoxMonOtTidField::possible_values() as u64;
        let mut rng = ChaCha8Rng::seed_from_u64(3);

        let mut chosen = vec![];
        for _ in 0..1000 {
            chosen.push(rng.gen::<u64>() % count);
        }

        for i in chosen {
            let starting = BoxMonBitVec::new(BoxMonOtTidField::bit_count(), i);
            let box_mon_gender = BoxMonOtTidField::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u64());
        }
    }

    #[test]
    fn test_encode_and_decode() {
        let move_set = BoxMonOtTidField(12345);

        let bits = move_set.game_value_to_bits().unwrap();
        let decoded = BoxMonOtTidField::bits_to_game_value(&bits).unwrap();

        assert_eq!(move_set, decoded);
    }

    #[test]
    fn test_bits_correct() {
        let mut bits: BoxMonBitVec = BoxMonBitVec::default();
        for i in 0..BoxMonOtTidField::bit_count() {
            bits.0.push(i % 2 != 0);
        }

        let mon = BoxMonOtTidField::bits_to_game_value(&bits).unwrap();
        let new_bits = mon.game_value_to_bits().unwrap();

        assert_eq!(bits, new_bits);
    }
}
