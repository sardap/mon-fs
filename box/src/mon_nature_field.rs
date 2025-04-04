use crate::mon_field::{FromRepresentation, FromStringInput};
use edit_distance::edit_distance;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{Display, IntoEnumIterator};
use strum_macros::FromRepr;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(
    FromRepr,
    Default,
    Debug,
    Clone,
    Copy,
    EnumCountMacro,
    EnumIter,
    PartialEq,
    Eq,
    Display,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum BoxMonNatureField {
    #[default]
    Adamant,
    Bashful,
    Bold,
    Brave,
    Calm,
    Careful,
    Docile,
    Gentle,
    Hardy,
    Hasty,
    Impish,
    Jolly,
    Lax,
    Lonely,
    Mild,
    Modest,
    Naive,
    Naughty,
    Quiet,
    Quirky,
    Rash,
    Relaxed,
    Sassy,
    Serious,
    Timid,
}

impl FromRepresentation for BoxMonNatureField {
    fn from_repr(repr: u8) -> Option<Self> {
        Self::from_repr(repr)
    }

    fn to_u8(&self) -> u8 {
        *self as u8
    }
}

impl FromStringInput for BoxMonNatureField {
    fn try_from_string(input: &str) -> Option<Self> {
        if input.is_empty() {
            return Some(BoxMonNatureField::default());
        }

        lazy_static! {
            static ref STRING_TO_ENUM: HashMap<String, BoxMonNatureField> = {
                let mut m = HashMap::new();
                for variant in BoxMonNatureField::iter() {
                    m.insert(variant.to_string().to_uppercase(), variant);
                }
                m
            };
        }

        match STRING_TO_ENUM.get(input) {
            Some(value) => Some(*value),
            None => {
                let keys = STRING_TO_ENUM.keys();
                let mut best_key = None;
                let mut best_score = (usize::MAX, None);
                for key in keys {
                    let distance = edit_distance(input, key);
                    if distance < best_score.0 {
                        best_score = (distance, Some(key.clone()));
                        best_key = Some(key.clone());
                    }
                }

                let best_key = best_key.unwrap();

                // Fuck it
                STRING_TO_ENUM.get(&best_key).cloned()
            }
        }
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
        for i in 0..BoxMonNatureField::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonNatureField::bit_count(), i);
            let box_mon_gender = BoxMonNatureField::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            BoxMonNatureField::try_from_string(""),
            Some(BoxMonNatureField::default())
        );

        assert_eq!(
            BoxMonNatureField::try_from_string("Bold"),
            Some(BoxMonNatureField::Bold)
        )
    }
}
