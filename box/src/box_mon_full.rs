use crate::box_mon::{parse_section, BoxMon, StringMonParseError, StringsMon};
use crate::mon_captured_ball::BoxMonCapturedBall;
use crate::mon_exp_field::BoxMonExpField;
use crate::mon_field::{
    BitCount, FromGameValueError, FromStringInput, GameSerializer, ToGameValueError,
};
use crate::mon_gender::BoxMonGender;
use crate::mon_held_item_full::BoxMonHeldItemFull;
use crate::mon_met_level_field::BoxMonMetLevelField;
use crate::mon_moves_field::BoxMonMoveSet;
use crate::mon_name::BoxMonName;
use crate::mon_ot_gender_field::BoxMonOtGenderField;
use crate::mon_ot_name_field::BoxMonOtName;
use crate::mon_ot_tid_field::BoxMonOtTidField;
use crate::mon_pc_mark_field::BoxMonPCMark;
use crate::mon_ribbons_field::BoxMonRibbonsField;
use crate::mon_shiny_field::BoxMonShinyField;
use crate::mon_species_full::BoxMonSpeciesFull;
use crate::mon_virus_field::BoxMonVirusField;
use crate::{extend_game_bits, BoxMonBitVec};
use bit_vec::BitVec;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoxMonFull {
    pub species: BoxMonSpeciesFull,
    pub name: BoxMonName,
    pub held_item: BoxMonHeldItemFull,
    pub ribbons: BoxMonRibbonsField,
    pub ot_tid: BoxMonOtTidField,
    pub shiny: BoxMonShinyField,
    pub gender: BoxMonGender,
    pub pc_mark: BoxMonPCMark,
    pub ball: BoxMonCapturedBall,
    pub move_set: BoxMonMoveSet,
    pub ot_name: BoxMonOtName,
    pub exp: BoxMonExpField,
    pub met_level: BoxMonMetLevelField,
    pub virus: BoxMonVirusField,
    pub ot_gender: BoxMonOtGenderField,
}

impl BoxMon for BoxMonFull {}

impl BitCount for BoxMonFull {
    fn bit_count() -> usize {
        BoxMonSpeciesFull::bit_count()
            + BoxMonName::bit_count()
            + BoxMonHeldItemFull::bit_count()
            + BoxMonRibbonsField::bit_count()
            + BoxMonOtTidField::bit_count()
            + BoxMonShinyField::bit_count()
            + BoxMonGender::bit_count()
            + BoxMonPCMark::bit_count()
            + BoxMonCapturedBall::bit_count()
            + BoxMonMoveSet::bit_count()
            + BoxMonOtName::bit_count()
            + BoxMonExpField::bit_count()
            + BoxMonMetLevelField::bit_count()
            + BoxMonVirusField::bit_count()
            + BoxMonOtGenderField::bit_count()
    }
}

impl TryFrom<StringsMon> for BoxMonFull {
    type Error = StringMonParseError;

    fn try_from(raw: StringsMon) -> Result<Self, Self::Error> {
        let species = raw.species.as_str().try_into()?;

        let gender = match BoxMonGender::try_from_string(&raw.gender) {
            Some(gender) => gender,
            None => return Err(StringMonParseError::InvalidGender),
        };

        let pc_mark: BoxMonPCMark = raw.pc_mark.try_into()?;

        let ball = match BoxMonCapturedBall::try_from_string(&raw.ball) {
            Some(box_mon_ball) => box_mon_ball,
            None => return Err(StringMonParseError::InvalidBall(raw.ball)),
        };

        let move_set: BoxMonMoveSet = raw.move_set.try_into()?;

        let name = match BoxMonName::try_from_string(&raw.name) {
            Some(name) => name,
            None => return Err(StringMonParseError::InvalidName(raw.name)),
        };

        let held_item = raw.held_item.as_str().try_into()?;

        let ribbons = raw.ribbons.try_into()?;

        let ot_tid = raw.ot_tid.try_into()?;

        let shiny = raw.shiny.into();

        let ot_name = raw.ot_name.as_str().try_into()?;

        let exp = raw.exp.try_into()?;

        let met_level = raw.met_level.try_into()?;

        let virus = raw.virus.into();

        let ot_gender = raw.ot_gender.into();

        Ok(BoxMonFull {
            species,
            name,
            held_item,
            ribbons,
            ot_tid,
            shiny,
            gender,
            pc_mark,
            ball,
            move_set,
            ot_name,
            exp,
            met_level,
            virus,
            ot_gender,
        })
    }
}

impl GameSerializer for BoxMonFull {
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError>
    where
        Self: Sized,
    {
        if value.0.len() != Self::bit_count() {
            return Err(ToGameValueError::BadBitsLength);
        }

        let mut offset = 0;

        let offset = &mut offset;

        let species = parse_section::<BoxMonSpeciesFull>(&value, offset)?;
        let name = parse_section::<BoxMonName>(&value, offset)?;
        let held_item = parse_section::<BoxMonHeldItemFull>(&value, offset)?;
        let ribbons = parse_section::<BoxMonRibbonsField>(&value, offset)?;
        let ot_tid = parse_section::<BoxMonOtTidField>(&value, offset)?;
        let shiny = parse_section::<BoxMonShinyField>(&value, offset)?;
        let gender = parse_section::<BoxMonGender>(&value, offset)?;
        let pc_mark = parse_section::<BoxMonPCMark>(&value, offset)?;
        let ball = parse_section::<BoxMonCapturedBall>(&value, offset)?;
        let move_set = parse_section::<BoxMonMoveSet>(&value, offset)?;
        let ot_name = parse_section::<BoxMonOtName>(&value, offset)?;
        let exp = parse_section::<BoxMonExpField>(&value, offset)?;
        let met_level = parse_section::<BoxMonMetLevelField>(&value, offset)?;
        let virus = parse_section::<BoxMonVirusField>(&value, offset)?;
        let ot_gender = parse_section::<BoxMonOtGenderField>(&value, offset)?;

        Ok(BoxMonFull {
            species,
            name,
            held_item,
            ribbons,
            ot_tid,
            shiny,
            gender,
            pc_mark,
            ball,
            move_set,
            ot_name,
            exp,
            met_level,
            virus,
            ot_gender,
        })
    }

    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError>
    where
        Self: Sized,
    {
        let mut bits = BitVec::new();

        extend_game_bits!(
            bits, self, species, name, held_item, ribbons, ot_tid, shiny, gender, pc_mark, ball,
            move_set, ot_name, exp, met_level, virus, ot_gender
        );

        Ok(BoxMonBitVec(bits))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mon_gender::BoxMonGender,
        mon_moves_field::BoxMonMove,
        mon_name::{BoxMonCharacter, BoxMonName},
    };

    use super::*;

    #[test]
    fn test_get_value_of_box_mon() {
        let mon = BoxMonFull {
            species: BoxMonSpeciesFull::Bulbasaur,
            gender: BoxMonGender::Male,
            pc_mark: BoxMonPCMark([false, true, false, true]),
            ball: BoxMonCapturedBall::PokeBall,
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
            held_item: BoxMonHeldItemFull::DireHit,
            move_set: BoxMonMoveSet::new([
                BoxMonMove::Pound,
                BoxMonMove::Growl,
                BoxMonMove::Absorb,
                BoxMonMove::LeechSeed,
            ]),
            ribbons: BoxMonRibbonsField(5),
            ot_tid: BoxMonOtTidField(12345),
            shiny: BoxMonShinyField(true),
            ot_name: BoxMonOtName::new([
                BoxMonCharacter::UpperP,
                BoxMonCharacter::LowerO,
                BoxMonCharacter::LowerO,
                BoxMonCharacter::LowerO,
                BoxMonCharacter::LowerO,
                BoxMonCharacter::LowerO,
                BoxMonCharacter::LowerO,
            ]),
            exp: BoxMonExpField(12345),
            met_level: BoxMonMetLevelField(5),
            virus: BoxMonVirusField(true),
            ot_gender: BoxMonOtGenderField(true),
        };

        let bits = mon.game_value_to_bits().unwrap();
        assert_eq!(bits.0.len(), BoxMonFull::bit_count());
        let recreated_mon = BoxMonFull::bits_to_game_value(&bits).unwrap();

        assert_eq!(mon, recreated_mon);
    }

    #[test]
    fn parse_from_raw() {
        let raw = StringsMon {
            name: "ADBASDGADS".to_string(),
            species: "NATU".to_string(),
            gender: "M".to_string(),
            pc_mark: vec![false, true, false, true],
            ball: "POKE".to_string(),
            held_item: "TM04".to_string(),
            move_set: vec![
                "POUND".to_string(),
                "GROWL".to_string(),
                "ABSORB".to_string(),
                "LEECHSEED".to_string(),
            ],
            ribbons: 11,
            ot_tid: 12345,
            shiny: true,
            ot_name: "Poooooo".to_string(),
            exp: 123456,
            met_level: 5,
            virus: false,
            ot_gender: true,
        };

        let mon = BoxMonFull::try_from(raw).unwrap();

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
        assert_eq!(mon.species, BoxMonSpeciesFull::Natu);
        assert_eq!(mon.gender, BoxMonGender::Male);
        assert_eq!(mon.pc_mark, BoxMonPCMark([false, true, false, true]));
        assert_eq!(
            mon.move_set,
            BoxMonMoveSet::new([
                BoxMonMove::Pound,
                BoxMonMove::Growl,
                BoxMonMove::Absorb,
                BoxMonMove::LeechSeed,
            ])
        );
        assert_eq!(mon.ball, BoxMonCapturedBall::PokeBall);
        assert_eq!(mon.held_item, BoxMonHeldItemFull::Tm04);
        assert_eq!(mon.ribbons, BoxMonRibbonsField(11));
        assert_eq!(mon.ot_tid, BoxMonOtTidField(12345));
        assert_eq!(mon.shiny, BoxMonShinyField(true));
        assert_eq!(mon.exp, BoxMonExpField(123456));
        assert_eq!(mon.met_level, BoxMonMetLevelField(5));
        assert_eq!(mon.virus, BoxMonVirusField(false));
    }

    #[test]
    fn test_box_mon_bits_correct() {
        let mut bits: BoxMonBitVec = BoxMonBitVec::default();
        for i in 0..BoxMonFull::bit_count() {
            bits.0.push(i % 2 == 0);
        }

        let mon = BoxMonFull::bits_to_game_value(&bits).unwrap();
        let new_bits = mon.game_value_to_bits().unwrap();

        assert_eq!(bits, new_bits);
    }
}
