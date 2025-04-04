use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

use crate::{
    box_mon::StringMonParseError,
    mon_field::{BitCount, FromGameValueError, FromStringInput, GameSerializer, ToGameValueError},
    mon_name::BoxMonCharacter,
    BoxMonBitVec,
};

const OT_NAME_LENGTH: usize = 7;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BoxMonOtName {
    name: [BoxMonCharacter; OT_NAME_LENGTH],
}

impl BoxMonOtName {
    pub fn new(name: [BoxMonCharacter; OT_NAME_LENGTH]) -> Self {
        BoxMonOtName { name }
    }
}

impl TryFrom<&str> for BoxMonOtName {
    type Error = StringMonParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if input.chars().count() != OT_NAME_LENGTH {
            return Err(StringMonParseError::InvalidOtNameLength(input.to_string()));
        }

        let mut name = [BoxMonCharacter::LowerA; OT_NAME_LENGTH];
        for (i, character) in input.chars().enumerate() {
            let character = match BoxMonCharacter::try_from_string(&character.to_string()) {
                Some(character) => character,
                None => {
                    return Err(StringMonParseError::InvalidOtNameCharacter(
                        character.to_string(),
                    ))
                }
            };
            name[i] = character;
        }

        Ok(BoxMonOtName { name })
    }
}

impl ToString for BoxMonOtName {
    fn to_string(&self) -> String {
        self.name.iter().map(|c| c.to_string()).collect()
    }
}

impl Serialize for BoxMonOtName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BoxMonOtName {
    fn deserialize<D>(deserializer: D) -> Result<BoxMonOtName, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        match BoxMonOtName::try_from(name.as_str()) {
            Ok(name) => Ok(name),
            Err(err) => Err(serde::de::Error::custom(err.to_string())),
        }
    }
}

impl BitCount for BoxMonOtName {
    fn bit_count() -> usize {
        BoxMonCharacter::bit_count() * OT_NAME_LENGTH
    }
}

impl GameSerializer for BoxMonOtName {
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError>
    where
        Self: Sized,
    {
        if value.0.len() != Self::bit_count() {
            return Err(ToGameValueError::BadBitsLength);
        }

        let mut name = [BoxMonCharacter::LowerA; OT_NAME_LENGTH];
        for i in 0..OT_NAME_LENGTH {
            let start = i * BoxMonCharacter::bit_count();
            let end = (i + 1) * BoxMonCharacter::bit_count();
            let character_bits = value.chunk(start, end);
            let character = match BoxMonCharacter::bits_to_game_value(&character_bits) {
                Ok(character) => character,
                Err(err) => return Err(err),
            };
            name[i] = character;
        }

        Ok(BoxMonOtName { name })
    }

    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError>
    where
        Self: Sized,
    {
        let mut bits = BitVec::new();
        for i in 0..OT_NAME_LENGTH {
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
        assert_eq!(BoxMonCharacter::possible_values(), 64);

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
        let count = BoxMonOtName::possible_values() as u64;
        let mut rng = ChaCha8Rng::seed_from_u64(3);

        let mut chosen = vec![];
        for _ in 0..1000 {
            chosen.push(rng.gen::<u64>() % count);
        }

        for i in chosen {
            let starting = BoxMonBitVec::new(BoxMonOtName::bit_count(), i);
            let box_mon_gender = BoxMonOtName::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u64());
        }
    }

    #[test]
    fn test_serializing_name() {
        let name = BoxMonOtName::new([
            BoxMonCharacter::UpperA,
            BoxMonCharacter::UpperB,
            BoxMonCharacter::UpperC,
            BoxMonCharacter::UpperD,
            BoxMonCharacter::UpperE,
            BoxMonCharacter::UpperF,
            BoxMonCharacter::UpperG,
        ]);
        let serialized = serde_json::to_string(&name).unwrap();
        assert_eq!(serialized, "\"ABCDEFG\"");
    }

    #[test]
    fn test_deserializing_name() {
        let serialized = "\"♂Iq5/iG\"";
        let name: BoxMonOtName = serde_json::from_str(serialized).unwrap();
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
            ]
        );
    }

    #[test]
    fn test_name_to_and_from_string() {
        let name = "ABCDEFG";
        let box_mon_name = BoxMonOtName::try_from(name).unwrap();
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
            ]
        )
    }
}
