use serde_derive::{Deserialize, Serialize};
use strum::EnumIter;
use strum_macros::{EnumCount, FromRepr};

use crate::mon_field::{FromRepresentation, FromStringInput};

#[derive(
    FromRepr, Debug, Clone, Copy, EnumCount, EnumIter, PartialEq, Eq, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum BoxMonSpecies {
    POOCHYENA,
    NINCADA,
    WHISMUR,
    TAILLOW,
}

impl FromRepresentation for BoxMonSpecies {
    fn from_repr(repr: u8) -> Option<Self> {
        Self::from_repr(repr)
    }

    fn to_u8(&self) -> u8 {
        *self as u8
    }
}

impl FromStringInput for BoxMonSpecies {
    fn try_from_string(input: &str) -> Option<Self> {
        Some(match input {
            "POOCHYENA" => BoxMonSpecies::POOCHYENA,
            "NINCADA" => BoxMonSpecies::NINCADA,
            "WHISMUR" => BoxMonSpecies::WHISMUR,
            "TAILLOW" => BoxMonSpecies::TAILLOW,
            _ => return None,
        })
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
        bits.push(false);
        bits.push(false);
        let box_mon_species = BoxMonSpecies::bits_to_game_value(&BoxMonBitVec(bits)).unwrap();
        assert_eq!(BoxMonSpecies::POOCHYENA, box_mon_species);

        let mut bits = BitVec::new();
        bits.push(true);
        bits.push(false);
        let box_mon_species = BoxMonSpecies::bits_to_game_value(&BoxMonBitVec(bits)).unwrap();
        assert_eq!(BoxMonSpecies::NINCADA, box_mon_species);

        let mut bits = BitVec::new();
        bits.push(false);
        bits.push(true);
        let box_mon_species = BoxMonSpecies::bits_to_game_value(&BoxMonBitVec(bits)).unwrap();
        assert_eq!(BoxMonSpecies::WHISMUR, box_mon_species);

        let mut bits = BitVec::new();
        bits.push(true);
        bits.push(true);
        let box_mon_species = BoxMonSpecies::bits_to_game_value(&BoxMonBitVec(bits)).unwrap();
        assert_eq!(BoxMonSpecies::TAILLOW, box_mon_species);
    }

    #[test]
    fn test_to_and_from() {
        for i in 0..BoxMonSpecies::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonSpecies::bit_count(), i);
            let box_mon_gender = BoxMonSpecies::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            BoxMonSpecies::try_from_string("POOCHYENA"),
            Some(BoxMonSpecies::POOCHYENA)
        );

        assert_eq!(
            BoxMonSpecies::try_from_string("NINCADA"),
            Some(BoxMonSpecies::NINCADA)
        );

        assert_eq!(
            BoxMonSpecies::try_from_string("WHISMUR"),
            Some(BoxMonSpecies::WHISMUR)
        );

        assert_eq!(
            BoxMonSpecies::try_from_string("TAILLOW"),
            Some(BoxMonSpecies::TAILLOW)
        );
    }
}
