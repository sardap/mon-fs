use crate::mon_field::{
    BitCount, FromGameValueError, FromStringInput, GameSerializer, ToGameValueError,
};
use crate::mon_gender::BoxMonGender;
use crate::mon_held_item::BoxMonHeldItem;
use crate::mon_name::BoxMonName;
use crate::mon_species::BoxMonSpecies;
use crate::BoxMonBitVec;
use bit_vec::BitVec;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoxMon {
    pub species: BoxMonSpecies,
    pub gender: BoxMonGender,
    pub name: BoxMonName,
    pub held_item: BoxMonHeldItem,
}

#[derive(Debug)]
pub enum StringMonParseError {
    InvalidSpecies,
    InvalidGender,
    InvalidName(String),
    InvalidItem,
}

impl BitCount for BoxMon {
    fn bit_count() -> usize {
        BoxMonSpecies::bit_count()
            + BoxMonGender::bit_count()
            + BoxMonName::bit_count()
            + BoxMonHeldItem::bit_count()
    }
}

impl BoxMon {
    pub fn try_from_strings_mon(raw: StringsMon) -> Result<Self, StringMonParseError> {
        let species = match BoxMonSpecies::try_from_string(&raw.species) {
            Some(species) => species,
            None => return Err(StringMonParseError::InvalidSpecies),
        };

        let gender = match BoxMonGender::try_from_string(&raw.gender) {
            Some(gender) => gender,
            None => return Err(StringMonParseError::InvalidGender),
        };

        let name = match BoxMonName::try_from_string(&raw.name) {
            Some(name) => name,
            None => return Err(StringMonParseError::InvalidName(raw.name)),
        };

        let held_item = match BoxMonHeldItem::try_from_string(&raw.item) {
            Some(held_item) => held_item,
            None => return Err(StringMonParseError::InvalidItem),
        };

        Ok(BoxMon {
            species,
            gender,
            name,
            held_item,
        })
    }
}

impl GameSerializer for BoxMon {
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError>
    where
        Self: Sized,
    {
        if value.0.len() != Self::bit_count() {
            return Err(ToGameValueError::BadBitsLength);
        }

        let mut offset = 0;
        let species =
            match BoxMonSpecies::bits_to_game_value(&value.chunk(0, BoxMonSpecies::bit_count())) {
                Ok(species) => species,
                Err(err) => return Err(err),
            };
        offset += BoxMonSpecies::bit_count();

        let gender = match BoxMonGender::bits_to_game_value(
            &value.chunk(offset, offset + BoxMonGender::bit_count()),
        ) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        offset += BoxMonGender::bit_count();

        let name = match BoxMonName::bits_to_game_value(
            &value.chunk(offset, offset + BoxMonName::bit_count()),
        ) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        offset += BoxMonName::bit_count();

        let held_item = match BoxMonHeldItem::bits_to_game_value(
            &value.chunk(offset, offset + BoxMonHeldItem::bit_count()),
        ) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };

        Ok(BoxMon {
            species,
            gender,
            name,
            held_item,
        })
    }

    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError>
    where
        Self: Sized,
    {
        let mut bits = BitVec::new();

        let next_set = match self.species.game_value_to_bits() {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        bits.extend(next_set.0.iter());

        let next_set = match self.gender.game_value_to_bits() {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        bits.extend(next_set.0.iter());

        let next_set = match self.name.game_value_to_bits() {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        bits.extend(next_set.0.iter());

        let next_set = match self.held_item.game_value_to_bits() {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        bits.extend(next_set.0.iter());

        Ok(BoxMonBitVec(bits))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringsMon {
    pub name: String,
    pub species: String,
    pub gender: String,
    pub item: String,
}

#[cfg(test)]
mod tests {
    use crate::{
        mon_gender::BoxMonGender,
        mon_held_item::BoxMonHeldItem,
        mon_name::{BoxMonCharacter, BoxMonName},
        mon_species::BoxMonSpecies,
    };

    use super::*;

    #[test]
    fn test_get_value_of_box_mon() {
        let mon = BoxMon {
            species: BoxMonSpecies::POOCHYENA,
            gender: BoxMonGender::Male,
            name: BoxMonName::new([
                BoxMonCharacter::UpperP,
                BoxMonCharacter::LowerO,
                BoxMonCharacter::LowerO,
                BoxMonCharacter::LowerC,
                BoxMonCharacter::LowerH,
                BoxMonCharacter::LowerY,
                BoxMonCharacter::LowerE,
                BoxMonCharacter::LowerN,
                BoxMonCharacter::LowerA,
                BoxMonCharacter::Seven,
            ]),
            held_item: BoxMonHeldItem::Empty,
        };

        let bits = mon.game_value_to_bits().unwrap();
        let recreated_mon = BoxMon::bits_to_game_value(&bits).unwrap();
        assert_eq!(mon, recreated_mon);
    }

    #[test]
    fn parse_from_raw() {
        let raw = StringsMon {
            name: "ADBASDGADS".to_string(),
            species: "POOCHYENA".to_string(),
            gender: "M".to_string(),
            item: "".to_string(),
        };

        let mon = BoxMon::try_from_strings_mon(raw).unwrap();

        assert_eq!(
            mon.name,
            BoxMonName::new([
                BoxMonCharacter::UpperA,
                BoxMonCharacter::UpperD,
                BoxMonCharacter::UpperB,
                BoxMonCharacter::UpperA,
                BoxMonCharacter::UpperS,
                BoxMonCharacter::UpperD,
                BoxMonCharacter::UpperG,
                BoxMonCharacter::UpperA,
                BoxMonCharacter::UpperD,
                BoxMonCharacter::UpperS,
            ])
        );
        assert_eq!(mon.species, BoxMonSpecies::POOCHYENA);
        assert_eq!(mon.gender, BoxMonGender::Male);
        assert_eq!(mon.held_item, BoxMonHeldItem::Empty);
    }

    #[test]
    fn test_box_mon_size() {
        assert_eq!(BoxMon::bit_count() * 2 % 8, 0);
    }
}
