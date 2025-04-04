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
pub enum BoxMonHeldItemFull {
    #[default]
    MasterBall,
    UltraBall,
    GreatBall,
    PokeBall,
    SafariBall,
    NetBall,
    DiveBall,
    NestBall,
    RepeatBall,
    TimerBall,
    LuxuryBall,
    PremierBall,
    Potion,
    Antidote,
    BurnHeal,
    IceHeal,
    Awakening,
    ParalyzeHeal,
    FullRestore,
    MaxPotion,
    HyperPotion,
    SuperPotion,
    FullHeal,
    Revive,
    MaxRevive,
    FreshWater,
    SodaPop,
    Lemonade,
    EnergyPowder,
    EnergyRoot,
    HealPowder,
    RevivalHerb,
    Ether,
    MaxEther,
    Elixir,
    MaxElixir,
    LavaCookie,
    BlueFlute,
    YellowFlute,
    RedFlute,
    BlackFlute,
    WhiteFlute,
    BerryJuice,
    HpUp,
    Protein,
    Iron,
    Carbos,
    Calcium,
    RareCandy,
    PpUp,
    Zinc,
    PpMax,
    GuardSpec,
    DireHit,
    XAttack,
    XDefend,
    XSpeed,
    XAccuracy,
    XSpecial,
    PokeDoll,
    FluffyTail,
    SuperRepel,
    MaxRepel,
    EscapeRope,
    Repel,
    TamatoBerry,
    CornnBerry,
    MagostBerry,
    RabutaBerry,
    NomelBerry,
    SpelonBerry,
    PamtreBerry,
    WatmelBerry,
    DurinBerry,
    BelueBerry,
    LiechiBerry,
    GanlonBerry,
    SalacBerry,
    PetayaBerry,
    ApicotBerry,
    LansatBerry,
    StarfBerry,
    EnigmaBerry,
    Tm01,
    Tm02,
    Tm03,
    Tm04,
    Tm05,
    Tm06,
    Tm07,
    Tm08,
    Tm09,
    Tm10,
    Tm11,
    Tm12,
    Tm13,
    Tm14,
    Tm15,
    Tm16,
    Tm17,
    Tm18,
    Tm19,
    Tm20,
    Tm21,
    Tm22,
    Tm23,
    Tm24,
    Tm25,
    Tm26,
    Tm27,
    Tm28,
    Tm29,
    Tm30,
    Tm31,
    Tm32,
    Tm33,
    Tm34,
    Tm35,
    Tm36,
    Tm37,
    Tm38,
    Tm39,
    Tm40,
    Tm41,
    Tm42,
    Tm43,
    Tm44,
    Tm45,
}

impl FromRepresentation for BoxMonHeldItemFull {
    fn from_repr(repr: u8) -> Option<Self> {
        Self::from_repr(repr)
    }

    fn to_u8(&self) -> u8 {
        *self as u8
    }
}

impl FromStringInput for BoxMonHeldItemFull {
    fn try_from_string(input: &str) -> Option<Self> {
        if input.is_empty() {
            return Some(BoxMonHeldItemFull::default());
        }

        lazy_static! {
            static ref STRING_TO_ENUM: HashMap<String, BoxMonHeldItemFull> = {
                let mut m = HashMap::new();
                for variant in BoxMonHeldItemFull::iter() {
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
        for i in 0..BoxMonHeldItemFull::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonHeldItemFull::bit_count(), i);
            let box_mon_gender = BoxMonHeldItemFull::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            BoxMonHeldItemFull::try_from_string(""),
            Some(BoxMonHeldItemFull::default())
        );

        assert_eq!(
            BoxMonHeldItemFull::try_from_string("SPELONBERRY"),
            Some(BoxMonHeldItemFull::SpelonBerry)
        )
    }
}
