use crate::mon_field::{FromRepresentation, FromStringInput};
use edit_distance::edit_distance;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::FromRepr;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(FromRepr, Default, Debug, Clone, Copy, EnumCountMacro, EnumIter, PartialEq, Eq)]
#[repr(u8)]
pub enum BoxMonHeldItem {
    #[default]
    Empty,
    Antidote,
    Awakening,
    BurnHeal,
    Calcium,
    Carbos,
    DireHit,
    EscapeRope,
    FluffyTail,
    FullHeal,
    FullRestore,
    GuardSpec,
    HealPowder,
    HpUp,
    HyperPotion,
    IceHeal,
    Iron,
    MaxPotion,
    MaxRepel,
    ParlyzHeal,
    Potion,
    Protein,
    Repel,
    RevivalHerb,
    Revive,
    SuperPotion,
    SuperRepel,
    XAccuracy,
    XAttack,
    XDefend,
    XSpecial,
    XSpeed,
    Zinc,
}

impl ToString for BoxMonHeldItem {
    fn to_string(&self) -> String {
        match self {
            BoxMonHeldItem::Empty => "".to_string(),
            BoxMonHeldItem::Antidote => "Antidote".to_string(),
            BoxMonHeldItem::Awakening => "Awakening".to_string(),
            BoxMonHeldItem::BurnHeal => "Burn Heal".to_string(),
            BoxMonHeldItem::Calcium => "Calcium".to_string(),
            BoxMonHeldItem::Carbos => "Carbos".to_string(),
            BoxMonHeldItem::DireHit => "Dire Hit".to_string(),
            BoxMonHeldItem::EscapeRope => "Escape Rope".to_string(),
            BoxMonHeldItem::FluffyTail => "Fluffy Tail".to_string(),
            BoxMonHeldItem::FullHeal => "Full Heal".to_string(),
            BoxMonHeldItem::FullRestore => "Full Restore".to_string(),
            BoxMonHeldItem::GuardSpec => "Guard Spec.".to_string(),
            BoxMonHeldItem::HealPowder => "Heal Powder".to_string(),
            BoxMonHeldItem::HpUp => "HP Up".to_string(),
            BoxMonHeldItem::HyperPotion => "Hyper Potion".to_string(),
            BoxMonHeldItem::IceHeal => "Ice Heal".to_string(),
            BoxMonHeldItem::Iron => "Iron".to_string(),
            BoxMonHeldItem::MaxPotion => "Max Potion".to_string(),
            BoxMonHeldItem::MaxRepel => "Max Repel".to_string(),
            BoxMonHeldItem::ParlyzHeal => "Parlyz Heal".to_string(),
            BoxMonHeldItem::Potion => "Potion".to_string(),
            BoxMonHeldItem::Protein => "Protein".to_string(),
            BoxMonHeldItem::Repel => "Repel".to_string(),
            BoxMonHeldItem::RevivalHerb => "Revival Herb".to_string(),
            BoxMonHeldItem::Revive => "Revive".to_string(),
            BoxMonHeldItem::SuperPotion => "Super Potion".to_string(),
            BoxMonHeldItem::SuperRepel => "Super Repel".to_string(),
            BoxMonHeldItem::XAccuracy => "X Accuracy".to_string(),
            BoxMonHeldItem::XAttack => "X Attack".to_string(),
            BoxMonHeldItem::XDefend => "X Defend".to_string(),
            BoxMonHeldItem::XSpecial => "X Special".to_string(),
            BoxMonHeldItem::XSpeed => "X Speed".to_string(),
            BoxMonHeldItem::Zinc => "Zinc".to_string(),
        }
    }
}

impl Serialize for BoxMonHeldItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BoxMonHeldItem {
    fn deserialize<D>(deserializer: D) -> Result<BoxMonHeldItem, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "" => Ok(BoxMonHeldItem::Empty),
            "Antidote" => Ok(BoxMonHeldItem::Antidote),
            "Awakening" => Ok(BoxMonHeldItem::Awakening),
            "Burn Heal" => Ok(BoxMonHeldItem::BurnHeal),
            "Calcium" => Ok(BoxMonHeldItem::Calcium),
            "Carbos" => Ok(BoxMonHeldItem::Carbos),
            "Dire Hit" => Ok(BoxMonHeldItem::DireHit),
            "Escape Rope" => Ok(BoxMonHeldItem::EscapeRope),
            "Fluffy Tail" => Ok(BoxMonHeldItem::FluffyTail),
            "Full Heal" => Ok(BoxMonHeldItem::FullHeal),
            "Full Restore" => Ok(BoxMonHeldItem::FullRestore),
            "Guard Spec." => Ok(BoxMonHeldItem::GuardSpec),
            "Heal Powder" => Ok(BoxMonHeldItem::HealPowder),
            "HP Up" => Ok(BoxMonHeldItem::HpUp),
            "Hyper Potion" => Ok(BoxMonHeldItem::HyperPotion),
            "Ice Heal" => Ok(BoxMonHeldItem::IceHeal),
            "Iron" => Ok(BoxMonHeldItem::Iron),
            "Max Potion" => Ok(BoxMonHeldItem::MaxPotion),
            "Max Repel" => Ok(BoxMonHeldItem::MaxRepel),
            "Parlyz Heal" => Ok(BoxMonHeldItem::ParlyzHeal),
            "Potion" => Ok(BoxMonHeldItem::Potion),
            "Protein" => Ok(BoxMonHeldItem::Protein),
            "Repel" => Ok(BoxMonHeldItem::Repel),
            "Revival Herb" => Ok(BoxMonHeldItem::RevivalHerb),
            "Revive" => Ok(BoxMonHeldItem::Revive),
            "Super Potion" => Ok(BoxMonHeldItem::SuperPotion),
            "Super Repel" => Ok(BoxMonHeldItem::SuperRepel),
            "X Accuracy" => Ok(BoxMonHeldItem::XAccuracy),
            "X Attack" => Ok(BoxMonHeldItem::XAttack),
            "X Defend" => Ok(BoxMonHeldItem::XDefend),
            "X Special" => Ok(BoxMonHeldItem::XSpecial),
            "X Speed" => Ok(BoxMonHeldItem::XSpeed),
            "Zinc" => Ok(BoxMonHeldItem::Zinc),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid BoxMonHeldItem: {}",
                s
            ))),
        }
    }
}

impl FromRepresentation for BoxMonHeldItem {
    fn from_repr(repr: u8) -> Option<Self> {
        Self::from_repr(repr)
    }

    fn to_u8(&self) -> u8 {
        *self as u8
    }
}

impl FromStringInput for BoxMonHeldItem {
    fn try_from_string(input: &str) -> Option<Self> {
        let mut mapping = HashMap::new();
        mapping.insert("", BoxMonHeldItem::Empty);
        mapping.insert("ANTIDOTE", BoxMonHeldItem::Antidote);
        mapping.insert("AWAKENING", BoxMonHeldItem::Awakening);
        mapping.insert("BURNHEAL", BoxMonHeldItem::BurnHeal);
        mapping.insert("CALCIUM", BoxMonHeldItem::Calcium);
        mapping.insert("CARBOS", BoxMonHeldItem::Carbos);
        mapping.insert("DIREHIT", BoxMonHeldItem::DireHit);
        mapping.insert("ESCAPEROPE", BoxMonHeldItem::EscapeRope);
        mapping.insert("FLUFFYTAIL", BoxMonHeldItem::FluffyTail);
        mapping.insert("FULLHEAL", BoxMonHeldItem::FullHeal);
        mapping.insert("FULLRESTORE", BoxMonHeldItem::FullRestore);
        mapping.insert("GUARDSPEC", BoxMonHeldItem::GuardSpec);
        mapping.insert("HEALPOWDER", BoxMonHeldItem::HealPowder);
        mapping.insert("HPUP", BoxMonHeldItem::HpUp);
        mapping.insert("HYPERPOTION", BoxMonHeldItem::HyperPotion);
        mapping.insert("ICEHEAL", BoxMonHeldItem::IceHeal);
        mapping.insert("IRON", BoxMonHeldItem::Iron);
        mapping.insert("MAXPOTION", BoxMonHeldItem::MaxPotion);
        mapping.insert("MAXREPEL", BoxMonHeldItem::MaxRepel);
        mapping.insert("PARALYZEHEAL", BoxMonHeldItem::ParlyzHeal);
        mapping.insert("POTION", BoxMonHeldItem::Potion);
        mapping.insert("PROTEIN", BoxMonHeldItem::Protein);
        mapping.insert("REPEL", BoxMonHeldItem::Repel);
        mapping.insert("REVIVALHERB", BoxMonHeldItem::RevivalHerb);
        mapping.insert("REVIVE", BoxMonHeldItem::Revive);
        mapping.insert("SUPERPOTION", BoxMonHeldItem::SuperPotion);
        mapping.insert("SUPERREPEL", BoxMonHeldItem::SuperRepel);
        mapping.insert("XACCURACY", BoxMonHeldItem::XAccuracy);
        mapping.insert("XATTACK", BoxMonHeldItem::XAttack);
        mapping.insert("XDEFEND", BoxMonHeldItem::XDefend);
        mapping.insert("XSPECIAL", BoxMonHeldItem::XSpecial);
        mapping.insert("XSPEED", BoxMonHeldItem::XSpeed);
        mapping.insert("ZINC", BoxMonHeldItem::Zinc);

        match mapping.get(input) {
            Some(value) => Some(*value),
            None => {
                let keys = mapping.keys();
                let mut best_key = None;
                let mut best_score = (usize::MAX, None);
                for key in keys {
                    let distance = edit_distance(input, key);
                    if distance < best_score.0 {
                        best_score = (distance, Some(*key));
                        best_key = Some(*key);
                    }
                }

                // Fuck it
                mapping.get(best_key.unwrap()).cloned()
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
        for i in 0..BoxMonHeldItem::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonHeldItem::bit_count(), i);
            let box_mon_gender = BoxMonHeldItem::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            BoxMonHeldItem::try_from_string(""),
            Some(BoxMonHeldItem::Empty)
        );
    }
}
