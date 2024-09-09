use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, FromRepr};

use crate::mon_field::{FromRepresentation, FromStringInput};

#[derive(
    FromRepr,
    Default,
    Debug,
    Clone,
    Copy,
    EnumCount,
    EnumIter,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum BoxMonGender {
    #[default]
    Male,
    Female,
}

impl FromRepresentation for BoxMonGender {
    fn from_repr(repr: u8) -> Option<Self> {
        Self::from_repr(repr)
    }

    fn to_u8(&self) -> u8 {
        *self as u8
    }
}

impl FromStringInput for BoxMonGender {
    fn try_from_string(input: &str) -> Option<Self> {
        Some(match input {
            "M" => BoxMonGender::Male,
            "F" => BoxMonGender::Female,
            _ => return None,
        })
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
        for i in 0..BoxMonGender::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonGender::bit_count(), i);
            let box_mon_gender = BoxMonGender::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }

    #[test]
    fn test_from_string() {
        assert_eq!(BoxMonGender::try_from_string("M"), Some(BoxMonGender::Male));
    }
}
