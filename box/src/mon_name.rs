use bit_vec::BitVec;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter};
use strum_macros::FromRepr;

use crate::{
    mon_field::{
        BitCount, FromGameValueError, FromRepresentation, FromStringInput, GameSerializer,
        ToGameValueError,
    },
    BoxMonBitVec,
};

#[derive(FromRepr, Default, Debug, Clone, Copy, EnumCount, EnumIter, PartialEq, Eq)]
#[repr(u8)]
pub enum BoxMonCharacter {
    #[default]
    LowerA,
    UpperA,
    LowerB,
    UpperB,
    LowerC,
    UpperC,
    LowerD,
    UpperD,
    LowerE,
    UpperE,
    LowerF,
    UpperF,
    LowerG,
    UpperG,
    LowerH,
    UpperH,
    LowerI,
    UpperI,
    LowerJ,
    UpperJ,
    LowerK,
    UpperK,
    LowerM,
    UpperM,
    LowerN,
    UpperN,
    LowerO,
    UpperO,
    LowerP,
    UpperP,
    LowerQ,
    UpperQ,
    LowerR,
    UpperR,
    LowerS,
    UpperS,
    LowerT,
    UpperT,
    LowerU,
    UpperU,
    LowerV,
    UpperV,
    LowerW,
    UpperW,
    LowerX,
    UpperX,
    LowerY,
    UpperY,
    LowerZ,
    UpperZ,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    ExclamationMark,
    QuestionMark,
    ForwardSlash,
    Dash,
    Ellipsis,
    Male,
    Female,
}

impl FromStringInput for BoxMonCharacter {
    fn try_from_string(input: &str) -> Option<Self> {
        let result = match input {
            "a" => BoxMonCharacter::LowerA,
            "A" => BoxMonCharacter::UpperA,
            "b" => BoxMonCharacter::LowerB,
            "B" => BoxMonCharacter::UpperB,
            "c" => BoxMonCharacter::LowerC,
            "C" => BoxMonCharacter::UpperC,
            "d" => BoxMonCharacter::LowerD,
            "D" => BoxMonCharacter::UpperD,
            "e" => BoxMonCharacter::LowerE,
            "E" => BoxMonCharacter::UpperE,
            "f" => BoxMonCharacter::LowerF,
            "F" => BoxMonCharacter::UpperF,
            "g" => BoxMonCharacter::LowerG,
            "G" => BoxMonCharacter::UpperG,
            "h" => BoxMonCharacter::LowerH,
            "H" => BoxMonCharacter::UpperH,
            "i" => BoxMonCharacter::LowerI,
            "I" => BoxMonCharacter::UpperI,
            "j" => BoxMonCharacter::LowerJ,
            "J" => BoxMonCharacter::UpperJ,
            "k" => BoxMonCharacter::LowerK,
            "K" => BoxMonCharacter::UpperK,
            "m" => BoxMonCharacter::LowerM,
            "M" => BoxMonCharacter::UpperM,
            "n" => BoxMonCharacter::LowerN,
            "N" => BoxMonCharacter::UpperN,
            "o" => BoxMonCharacter::LowerO,
            "O" => BoxMonCharacter::UpperO,
            "p" => BoxMonCharacter::LowerP,
            "P" => BoxMonCharacter::UpperP,
            "q" => BoxMonCharacter::LowerQ,
            "Q" => BoxMonCharacter::UpperQ,
            "r" => BoxMonCharacter::LowerR,
            "R" => BoxMonCharacter::UpperR,
            "s" => BoxMonCharacter::LowerS,
            "S" => BoxMonCharacter::UpperS,
            "t" => BoxMonCharacter::LowerT,
            "T" => BoxMonCharacter::UpperT,
            "u" => BoxMonCharacter::LowerU,
            "U" => BoxMonCharacter::UpperU,
            "v" => BoxMonCharacter::LowerV,
            "V" => BoxMonCharacter::UpperV,
            "w" => BoxMonCharacter::LowerW,
            "W" => BoxMonCharacter::UpperW,
            "x" => BoxMonCharacter::LowerX,
            "X" => BoxMonCharacter::UpperX,
            "y" => BoxMonCharacter::LowerY,
            "Y" => BoxMonCharacter::UpperY,
            "z" => BoxMonCharacter::LowerZ,
            "Z" => BoxMonCharacter::UpperZ,
            "2" => BoxMonCharacter::Two,
            "3" => BoxMonCharacter::Three,
            "4" => BoxMonCharacter::Four,
            "5" => BoxMonCharacter::Five,
            "6" => BoxMonCharacter::Six,
            "7" => BoxMonCharacter::Seven,
            "8" => BoxMonCharacter::Eight,
            "9" => BoxMonCharacter::Nine,
            "!" => BoxMonCharacter::ExclamationMark,
            "?" => BoxMonCharacter::QuestionMark,
            "/" => BoxMonCharacter::ForwardSlash,
            "-" => BoxMonCharacter::Dash,
            "…" => BoxMonCharacter::Ellipsis,
            "♂" => BoxMonCharacter::Male,
            "♀" => BoxMonCharacter::Female,
            _ => return None,
        };

        Some(result)
    }
}

impl ToString for BoxMonCharacter {
    fn to_string(&self) -> String {
        match self {
            BoxMonCharacter::LowerA => "a",
            BoxMonCharacter::UpperA => "A",
            BoxMonCharacter::LowerB => "b",
            BoxMonCharacter::UpperB => "B",
            BoxMonCharacter::LowerC => "c",
            BoxMonCharacter::UpperC => "C",
            BoxMonCharacter::LowerD => "d",
            BoxMonCharacter::UpperD => "D",
            BoxMonCharacter::LowerE => "e",
            BoxMonCharacter::UpperE => "E",
            BoxMonCharacter::LowerF => "f",
            BoxMonCharacter::UpperF => "F",
            BoxMonCharacter::LowerG => "g",
            BoxMonCharacter::UpperG => "G",
            BoxMonCharacter::LowerH => "h",
            BoxMonCharacter::UpperH => "H",
            BoxMonCharacter::LowerI => "i",
            BoxMonCharacter::UpperI => "I",
            BoxMonCharacter::LowerJ => "j",
            BoxMonCharacter::UpperJ => "J",
            BoxMonCharacter::LowerK => "k",
            BoxMonCharacter::UpperK => "K",
            BoxMonCharacter::LowerM => "m",
            BoxMonCharacter::UpperM => "M",
            BoxMonCharacter::LowerN => "n",
            BoxMonCharacter::UpperN => "N",
            BoxMonCharacter::LowerO => "o",
            BoxMonCharacter::UpperO => "O",
            BoxMonCharacter::LowerP => "p",
            BoxMonCharacter::UpperP => "P",
            BoxMonCharacter::LowerQ => "q",
            BoxMonCharacter::UpperQ => "Q",
            BoxMonCharacter::LowerR => "r",
            BoxMonCharacter::UpperR => "R",
            BoxMonCharacter::LowerS => "s",
            BoxMonCharacter::UpperS => "S",
            BoxMonCharacter::LowerT => "t",
            BoxMonCharacter::UpperT => "T",
            BoxMonCharacter::LowerU => "u",
            BoxMonCharacter::UpperU => "U",
            BoxMonCharacter::LowerV => "v",
            BoxMonCharacter::UpperV => "V",
            BoxMonCharacter::LowerW => "w",
            BoxMonCharacter::UpperW => "W",
            BoxMonCharacter::LowerX => "x",
            BoxMonCharacter::UpperX => "X",
            BoxMonCharacter::LowerY => "y",
            BoxMonCharacter::UpperY => "Y",
            BoxMonCharacter::LowerZ => "z",
            BoxMonCharacter::UpperZ => "Z",
            BoxMonCharacter::Two => "2",
            BoxMonCharacter::Three => "3",
            BoxMonCharacter::Four => "4",
            BoxMonCharacter::Five => "5",
            BoxMonCharacter::Six => "6",
            BoxMonCharacter::Seven => "7",
            BoxMonCharacter::Eight => "8",
            BoxMonCharacter::Nine => "9",
            BoxMonCharacter::ExclamationMark => "!",
            BoxMonCharacter::QuestionMark => "?",
            BoxMonCharacter::ForwardSlash => "/",
            BoxMonCharacter::Dash => "-",
            BoxMonCharacter::Ellipsis => "…",
            BoxMonCharacter::Male => "♂",
            BoxMonCharacter::Female => "♀",
        }
        .to_string()
    }
}

impl FromRepresentation for BoxMonCharacter {
    fn from_repr(repr: u8) -> Option<Self> {
        Self::from_repr(repr)
    }

    fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BoxMonName {
    name: [BoxMonCharacter; 10],
}

impl BoxMonName {
    pub fn new(name: [BoxMonCharacter; 10]) -> Self {
        BoxMonName { name }
    }
}

impl FromStringInput for BoxMonName {
    fn try_from_string(input: &str) -> Option<Self> {
        if input.chars().count() != 10 {
            return None;
        }

        let mut name = [BoxMonCharacter::LowerA; 10];
        for (i, character) in input.chars().enumerate() {
            let character = match BoxMonCharacter::try_from_string(&character.to_string()) {
                Some(character) => character,
                None => return None,
            };
            name[i] = character;
        }

        Some(BoxMonName { name })
    }
}

impl ToString for BoxMonName {
    fn to_string(&self) -> String {
        self.name.iter().map(|c| c.to_string()).collect()
    }
}

impl Serialize for BoxMonName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BoxMonName {
    fn deserialize<D>(deserializer: D) -> Result<BoxMonName, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        match BoxMonName::try_from_string(&name) {
            Some(name) => Ok(name),
            None => Err(serde::de::Error::custom("Invalid BoxMonName")),
        }
    }
}

impl BitCount for BoxMonName {
    fn bit_count() -> usize {
        BoxMonCharacter::bit_count() * 10
    }
}

impl GameSerializer for BoxMonName {
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError>
    where
        Self: Sized,
    {
        if value.0.len() != Self::bit_count() {
            return Err(ToGameValueError::BadBitsLength);
        }

        let mut name = [BoxMonCharacter::LowerA; 10];
        for i in 0..10 {
            let start = i * BoxMonCharacter::bit_count();
            let end = (i + 1) * BoxMonCharacter::bit_count();
            let character_bits = value.chunk(start, end);
            let character = match BoxMonCharacter::bits_to_game_value(&character_bits) {
                Ok(character) => character,
                Err(err) => return Err(err),
            };
            name[i] = character;
        }

        Ok(BoxMonName { name })
    }

    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError>
    where
        Self: Sized,
    {
        let mut bits = BitVec::new();
        for i in 0..10 {
            let character = self.name[i];
            let character_bits = match character.game_value_to_bits() {
                Ok(bits) => bits,
                Err(err) => return Err(err),
            };
            bits.extend(character_bits.0.iter());
        }

        Ok(BoxMonBitVec(bits))
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
    fn test_char_to_and_from() {
        for i in 0..BoxMonCharacter::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonCharacter::bit_count(), i);
            let box_mon_gender = BoxMonCharacter::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }

    #[test]
    fn test_name_to_and_from() {
        let count = BoxMonName::possible_values() as u64;
        let mut rng = ChaCha8Rng::seed_from_u64(3);

        let mut chosen = vec![];
        for _ in 0..1000 {
            chosen.push(rng.gen::<u64>() % count);
        }

        for i in chosen {
            let starting = BoxMonBitVec::new(BoxMonName::bit_count(), i);
            let box_mon_gender = BoxMonName::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u64());
        }
    }

    #[test]
    fn test_serializing_name() {
        let name = BoxMonName::new([
            BoxMonCharacter::UpperA,
            BoxMonCharacter::UpperB,
            BoxMonCharacter::UpperC,
            BoxMonCharacter::UpperD,
            BoxMonCharacter::UpperE,
            BoxMonCharacter::UpperF,
            BoxMonCharacter::UpperG,
            BoxMonCharacter::UpperH,
            BoxMonCharacter::UpperI,
            BoxMonCharacter::UpperJ,
        ]);
        let serialized = serde_json::to_string(&name).unwrap();
        assert_eq!(serialized, "\"ABCDEFGHIJ\"");
    }

    #[test]
    fn test_deserializing_name() {
        let serialized = "\"♂Iq5/iGJD9\"";
        let name: BoxMonName = serde_json::from_str(serialized).unwrap();
        assert_eq!(
            name.name,
            [
                BoxMonCharacter::Male,
                BoxMonCharacter::UpperI,
                BoxMonCharacter::LowerQ,
                BoxMonCharacter::Five,
                BoxMonCharacter::ForwardSlash,
                BoxMonCharacter::LowerI,
                BoxMonCharacter::UpperG,
                BoxMonCharacter::UpperJ,
                BoxMonCharacter::UpperD,
                BoxMonCharacter::Nine,
            ]
        );
    }

    #[test]
    fn test_name_to_and_from_string() {
        let name = "ABCDEFGHIJ";
        let box_mon_name = BoxMonName::try_from_string(name).unwrap();
        assert_eq!(
            box_mon_name.name.to_vec(),
            vec![
                BoxMonCharacter::UpperA,
                BoxMonCharacter::UpperB,
                BoxMonCharacter::UpperC,
                BoxMonCharacter::UpperD,
                BoxMonCharacter::UpperE,
                BoxMonCharacter::UpperF,
                BoxMonCharacter::UpperG,
                BoxMonCharacter::UpperH,
                BoxMonCharacter::UpperI,
                BoxMonCharacter::UpperJ,
            ]
        )
    }
}
