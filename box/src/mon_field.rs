use strum::IntoEnumIterator;

use crate::{count_to_bits, BoxMonBitVec};

pub trait FromRepresentation {
    fn from_repr(repr: u8) -> Option<Self>
    where
        Self: Sized;

    fn to_u8(&self) -> u8;
}

pub trait GameSerializer {
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError>
    where
        Self: Sized;
    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError>
    where
        Self: Sized;
}

pub trait FromStringInput {
    fn try_from_string(input: &str) -> Option<Self>
    where
        Self: Sized;
}

pub trait BitCount {
    fn bit_count() -> usize;
}

impl<T> BitCount for T
where
    T: IntoEnumIterator,
{
    fn bit_count() -> usize {
        count_to_bits(T::iter().count())
    }
}

pub trait ByteCount {
    fn byte_count() -> usize;
}

impl<T> ByteCount for T
where
    T: BitCount,
{
    fn byte_count() -> usize {
        T::bit_count() / 8
    }
}

#[cfg(test)]
pub trait PossibleValues {
    fn possible_values() -> usize;
}

#[cfg(test)]
impl<T> PossibleValues for T
where
    T: BitCount,
{
    fn possible_values() -> usize {
        2usize.pow(T::bit_count() as u32)
    }
}

impl<T> GameSerializer for T
where
    T: BitCount + FromRepresentation + Copy + Sized,
{
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError> {
        if value.0.len() != Self::bit_count() {
            return Err(ToGameValueError::BadBitsLength);
        }

        let x = value.as_u8();
        match Self::from_repr(x) {
            Some(value) => Ok(value),
            None => Err(ToGameValueError::BitsValueOutOfRange),
        }
    }

    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError> {
        let value = self.to_u8();
        Ok(BoxMonBitVec::new(Self::bit_count(), value))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ToGameValueError {
    BadBitsLength,
    BitsValueOutOfRange,
}

#[derive(Debug, Clone, Copy)]
pub enum FromGameValueError {
    BadSize(usize),
}
