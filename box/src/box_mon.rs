use crate::{
    mon_field::{BitCount, GameSerializer, ToGameValueError},
    BoxMonBitVec,
};
use serde_derive::{Deserialize, Serialize};
use strum::Display;

pub trait BoxMon
where
    Self: Sized,
    Self: BitCount,
    Self: GameSerializer,
    Self: Clone,
    Self: Copy,
    Self: TryFrom<StringsMon, Error = StringMonParseError>,
{
}

#[derive(Debug, Display)]
pub enum StringMonParseError {
    InvalidShiny,
    InvalidSpecies,
    InvalidLevel,
    InvalidGender,
    InvalidName(String),
    InvalidPcMark,
    InvalidMoves(Vec<String>),
    InvalidNature(String),
    InvalidItem,
    InvalidBall(String),
    InvalidRibbons(u8),
    InvalidOtTidLength(usize),
    InvalidStupidNumber(String),
    InvalidOtNameLength(String),
    InvalidOtNameCharacter(String),
    InvalidExpLength(usize),
    InvalidMetLevel(u8),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringsMon {
    pub name: String,
    pub species: String,
    pub gender: String,
    pub pc_mark: Vec<bool>,
    pub captured_ball: String,
    pub move_set: Vec<String>,
    pub held_item: String,
    pub ribbons: u8,
    pub ot_tid: u16,
    pub shiny: bool,
    pub ot_name: String,
    pub exp: u32,
    pub met_level: u8,
    pub virus: bool,
    pub ot_gender: bool,
}

#[macro_export]
macro_rules! extend_game_bits {
    ($bits:ident, $self:ident, $($field:ident),*) => {
        $(
            let next_set = match $self.$field.game_value_to_bits() {
                Ok(val) => val,
                Err(err) => return Err(err),
            };
            $bits.extend(next_set.0.iter());
        )*
    };
}

pub fn parse_section<T: GameSerializer + BitCount>(
    bits: &BoxMonBitVec,
    offset: &mut usize,
) -> Result<T, ToGameValueError> {
    let start = *offset;
    let end = start + T::bit_count();
    let chunk = bits.chunk(start, end);
    let result = match T::bits_to_game_value(&chunk) {
        Ok(result) => result,
        Err(err) => return Err(err),
    };
    *offset += T::bit_count();
    Ok(result)
}
