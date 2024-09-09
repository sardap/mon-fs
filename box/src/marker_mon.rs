use crate::box_mon::{BoxMon, StringsMon};
use crate::mon_field::{
    BitCount, FromGameValueError, FromStringInput, GameSerializer, ToGameValueError,
};
use crate::mon_gender::BoxMonGender;
use crate::mon_held_item::BoxMonHeldItem;
use crate::mon_name::BoxMonName;
use crate::BoxMonBitVec;
use bit_vec::BitVec;
use serde_derive::{Deserialize, Serialize};

const NAME_START: usize = 0;
const NAME_END: usize = NAME_START + 56;
const PADDING_AMOUNT_START: usize = NAME_END + 1;
const PADDING_AMOUNT_END: usize = PADDING_AMOUNT_START + 7;
const COMPRESSED: usize = PADDING_AMOUNT_END + 1;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkerMon {
    pub gender: BoxMonGender,
    pub name: BoxMonName,
    pub held_item: BoxMonHeldItem,
}

#[derive(Debug)]
pub enum StringMonParseError {
    InvalidSpecies,
    InvalidGender,
    InvalidName,
    InvalidItem,
}

pub enum MarkerMonNewError {
    InvalidFilename,
}

impl MarkerMon {
    pub fn new_starter_marker<T: ToString>(
        filename: T,
        padding_amount: u8,
        compressed: bool,
    ) -> Result<Self, ToGameValueError> {
        if padding_amount as usize > BoxMon::bit_count() {
            return Err(ToGameValueError::BadBitsLength);
        }

        let mut bits = BoxMonBitVec(BitVec::from_elem(Self::bit_count(), false));

        let mut name_bits = BoxMonBitVec::new_from_raw(&filename.to_string().as_bytes());

        if name_bits.0.len() > NAME_END {
            return Err(ToGameValueError::BadBitsLength);
        }

        while name_bits.0.len() < NAME_END {
            name_bits.0.push(false);
        }

        for i in 0..name_bits.0.len() {
            bits.0.set(i + NAME_START, name_bits.0[i]);
        }

        for i in 0..(PADDING_AMOUNT_END - PADDING_AMOUNT_START) {
            bits.0
                .set(i + PADDING_AMOUNT_START, padding_amount & (1 << i) != 0);
        }

        bits.0.set(COMPRESSED, compressed);

        MarkerMon::bits_to_game_value(&bits)
    }

    pub fn bit_count() -> usize {
        BoxMonGender::bit_count() + BoxMonName::bit_count() + BoxMonHeldItem::bit_count()
    }

    pub fn try_from_strings_mon(raw: StringsMon) -> Result<Self, StringMonParseError> {
        let name = match BoxMonName::try_from_string(&raw.name) {
            Some(name) => name,
            None => return Err(StringMonParseError::InvalidName),
        };

        let held_item = match BoxMonHeldItem::try_from_string(&raw.item) {
            Some(held_item) => held_item,
            None => return Err(StringMonParseError::InvalidItem),
        };

        let gender = match BoxMonGender::try_from_string(&raw.gender) {
            Some(gender) => gender,
            None => return Err(StringMonParseError::InvalidGender),
        };

        Ok(MarkerMon {
            name,
            held_item,
            gender,
        })
    }

    pub fn padding_amount(&self) -> u8 {
        let bits = self.game_value_to_bits().unwrap();
        let mut bits = bits.chunk(PADDING_AMOUNT_START, PADDING_AMOUNT_END);
        while bits.0.len() < 8 {
            bits.0.push(false);
        }

        bits.as_u8()
    }

    pub fn is_compressed(&self) -> bool {
        let bits: BoxMonBitVec = self.game_value_to_bits().unwrap();
        bits.0[COMPRESSED]
    }
}

impl ToString for MarkerMon {
    fn to_string(&self) -> String {
        let bits = self.game_value_to_bits().unwrap();
        // Exclude last 8 bits since they are for the padding amount
        let bits = bits.chunk(0, bits.0.len() - 8);
        let raw = bits.to_raw();

        let mut result = String::from_utf8(raw).unwrap();
        result.retain(|c| c != '\0');
        result
    }
}

impl GameSerializer for MarkerMon {
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError>
    where
        Self: Sized,
    {
        if value.0.len() != Self::bit_count() {
            return Err(ToGameValueError::BadBitsLength);
        }

        let mut offset = 0;

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
        offset += BoxMonHeldItem::bit_count();

        let gender = match BoxMonGender::bits_to_game_value(
            &value.chunk(offset, offset + BoxMonGender::bit_count()),
        ) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };

        Ok(MarkerMon {
            name,
            held_item,
            gender,
        })
    }

    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError>
    where
        Self: Sized,
    {
        let mut bits = BitVec::new();

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

        let gender = match self.gender.game_value_to_bits() {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        bits.extend(gender.0.iter());

        assert_eq!(bits.len(), Self::bit_count());

        Ok(BoxMonBitVec(bits))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mon_held_item::BoxMonHeldItem,
        mon_name::{BoxMonCharacter, BoxMonName},
    };

    use super::*;

    #[test]
    fn parse_from_raw() {
        let raw = StringsMon {
            name: "ADBASDGADS".to_string(),
            species: "".to_string(),
            gender: "F".to_string(),
            item: "".to_string(),
        };

        let mon: MarkerMon = MarkerMon::try_from_strings_mon(raw).unwrap();

        assert_eq!(mon.gender, BoxMonGender::Female);
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
        assert_eq!(mon.held_item, BoxMonHeldItem::Empty);
    }

    #[test]
    fn test_new_starter_marker() {
        let mon = MarkerMon::new_starter_marker("a.txt", 34, true).unwrap();

        assert_eq!("a.txt", mon.to_string());
        assert_eq!(mon.padding_amount(), 34);

        let mon_js = serde_json::to_string(&mon).unwrap();

        let mon: MarkerMon = serde_json::from_str(&mon_js).unwrap();

        assert_eq!("a.txt", mon.to_string());
        assert_eq!(mon.padding_amount(), 34);
    }

    #[test]
    fn test_padding_amount() {
        for i in 0..=BoxMon::bit_count() {
            let compress = i % 2 == 0;
            let mon = MarkerMon::new_starter_marker("a.txt", i as u8, compress).unwrap();
            assert_eq!(i as u8, mon.padding_amount());
            assert_eq!(compress, mon.is_compressed(), "i = {}", i);
        }
    }
}
