use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter};
use strum_macros::FromRepr;

use crate::mon_field::{FromRepresentation, FromStringInput};

#[derive(
    FromRepr, Debug, Clone, Copy, EnumCount, EnumIter, PartialEq, Eq, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum BoxMonCapturedBall {
    DiveBall,
    GreatBall,
    NestBall,
    NetBall,
    PokeBall,
    RepeatBall,
    TimerBall,
    UltraBall,
}

impl FromRepresentation for BoxMonCapturedBall {
    fn from_repr(repr: u8) -> Option<Self> {
        Self::from_repr(repr)
    }

    fn to_u8(&self) -> u8 {
        *self as u8
    }
}

impl FromStringInput for BoxMonCapturedBall {
    fn try_from_string(input: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match input.to_lowercase().as_str() {
            "dive" => Some(BoxMonCapturedBall::DiveBall),
            "great" => Some(BoxMonCapturedBall::GreatBall),
            "nest" => Some(BoxMonCapturedBall::NestBall),
            "net" => Some(BoxMonCapturedBall::NetBall),
            "poke" => Some(BoxMonCapturedBall::PokeBall),
            "repeat" => Some(BoxMonCapturedBall::RepeatBall),
            "timer" => Some(BoxMonCapturedBall::TimerBall),
            "ultra" => Some(BoxMonCapturedBall::UltraBall),
            _ => None,
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
        for i in 0..BoxMonCapturedBall::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonCapturedBall::bit_count(), i);
            let box_mon_gender = BoxMonCapturedBall::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }
}
