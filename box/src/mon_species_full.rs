use std::collections::HashMap;

use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};
use strum_macros::{EnumCount, FromRepr};

use crate::{box_mon::StringMonParseError, mon_field::FromRepresentation};

#[derive(
    FromRepr,
    Debug,
    Clone,
    Copy,
    EnumCount,
    EnumIter,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Display,
)]
#[repr(u8)]
pub enum BoxMonSpeciesFull {
    Bulbasaur,
    Charmander,
    Squirtle,
    Caterpie,
    Weedle,
    Pidgey,
    Rattata,
    Spearow,
    Ekans,
    Pikachu,
    Sandshrew,
    Clefairy,
    Vulpix,
    Jigglypuff,
    Zubat,
    Oddish,
    Diglett,
    Meowth,
    Persian,
    Psyduck,
    Mankey,
    Growlithe,
    Poliwag,
    Abra,
    Machop,
    Bellsprout,
    Weepinbell,
    Victreebel,
    Tentacool,
    Geodude,
    Ponyta,
    Slowpoke,
    Doduo,
    Seel,
    Grimer,
    Shellder,
    Gastly,
    Gengar,
    Onix,
    Drowzee,
    Krabby,
    Exeggcute,
    Cubone,
    Pichu,
    Togepi,
    Lickitung,
    Koffing,
    Rhyhorn,
    Elekid,
    Tangela,
    Magby,
    Horsea,
    Goldeen,
    Scyther,
    Electabuzz,
    Magmar,
    Pinsir,
    Magikarp,
    Lapras,
    Eevee,
    Omanyte,
    Omastar,
    Kabuto,
    Kabutops,
    Aerodactyl,
    Snorlax,
    Dratini,
    Chikorita,
    Cyndaquil,
    Totodile,
    Sentret,
    Hoothoot,
    Ledyba,
    Spinarak,
    Chinchou,
    Natu,
    Mareep,
    Igglybuff,
    Ampharos,
    Bellossom,
    Marill,
    Azumarill,
    Sudowoodo,
    Politoed,
    Hoppip,
    Skiploom,
    Jumpluff,
    Aipom,
    Sunkern,
    Sunflora,
    Yanma,
    Wooper,
    Quagsire,
    Espeon,
    Umbreon,
    Murkrow,
    Slowking,
    Misdreavus,
    Wobbuffet,
    Girafarig,
    Pineco,
    Forretress,
    Dunsparce,
    Gligar,
    Steelix,
    Snubbull,
    Granbull,
    Qwilfish,
    Scizor,
    Shuckle,
    Heracross,
    Sneasel,
    Teddiursa,
    Ursaring,
    Slugma,
    Magcargo,
    Swinub,
    Piloswine,
    Corsola,
    Remoraid,
    Octillery,
    Delibird,
    Mantine,
    Skarmory,
    Houndour,
    Houndoom,
    Kingdra,
    Phanpy,
    Donphan,
    Stantler,
    Smeargle,
    Larvitar,
    Pupitar,
    Tyranitar,
    Treecko,
    Grovyle,
    Sceptile,
    Torchic,
    Combusken,
    Blaziken,
    Mudkip,
    Marshtomp,
    Swampert,
    Poochyena,
    Mightyena,
    Zigzagoon,
    Linoone,
    Wurmple,
    Silcoon,
    Beautifly,
    Cascoon,
    Dustox,
    Lotad,
    Lombre,
    Ludicolo,
    Seedot,
    Nuzleaf,
    Shiftry,
    Taillow,
    Swellow,
    Wingull,
    Pelipper,
    Ralts,
    Kirlia,
    Gardevoir,
    Surskit,
    Masquerain,
    Shroomish,
    Breloom,
    Slakoth,
    Vigoroth,
    Slaking,
    Nincada,
    Ninjask,
    Whismur,
    Loudred,
    Exploud,
    Makuhita,
    Hariyama,
    Azurill,
    Nosepass,
    Skitty,
    Delcatty,
    Sableye,
    Mawile,
    Aron,
    Lairon,
    Aggron,
    Meditite,
    Medicham,
    Electrike,
    Manectric,
    Plusle,
    Minun,
    Roselia,
    Gulpin,
    Swalot,
    Carvanha,
    Sharpedo,
    Wailmer,
    Wailord,
    Numel,
    Camerupt,
    Torkoal,
    Spoink,
    Grumpig,
    Spinda,
    Trapinch,
    Vibrava,
    Flygon,
    Cacnea,
    Cacturne,
    Swablu,
    Altaria,
    Zangoose,
    Seviper,
    Barboach,
    Whiscash,
    Corphish,
    Crawdaunt,
    Lileep,
    Cradily,
    Anorith,
    Armaldo,
    Feebas,
    Milotic,
    Castform,
    Kecleon,
    Shuppet,
    Banette,
    Duskull,
    Dusclops,
    Tropius,
    Chimecho,
    Absol,
    Wynaut,
    Snorunt,
    Glalie,
    Spheal,
    Sealeo,
    Walrein,
    Clamperl,
    Huntail,
    Gorebyss,
    Relicanth,
    Luvdisc,
    Bagon,
    Shelgon,
    Salamence,
    Volbeat,
    Nidorino,
    Latios,
    Nidoking,
    Tyrogue,
    Hitmonlee,
    Tauros,
}

impl TryFrom<&str> for BoxMonSpeciesFull {
    type Error = StringMonParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let input = value.to_uppercase();

        lazy_static! {
            static ref STRING_TO_ENUM: HashMap<String, BoxMonSpeciesFull> = {
                let mut m = HashMap::new();
                for variant in BoxMonSpeciesFull::iter() {
                    m.insert(variant.to_string().to_uppercase(), variant);
                }
                m
            };
        }

        match STRING_TO_ENUM.get(&input) {
            Some(&variant) => Ok(variant),
            None => Err(StringMonParseError::InvalidSpecies),
        }
    }
}

impl FromRepresentation for BoxMonSpeciesFull {
    fn from_repr(repr: u8) -> Option<Self> {
        Self::from_repr(repr)
    }

    fn to_u8(&self) -> u8 {
        *self as u8
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
        for i in 0..BoxMonSpeciesFull::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonSpeciesFull::bit_count(), i);
            let box_mon_gender = BoxMonSpeciesFull::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            BoxMonSpeciesFull::try_from("POOCHYENA").unwrap(),
            BoxMonSpeciesFull::Poochyena
        );

        assert_eq!(
            BoxMonSpeciesFull::try_from("NINCADA").unwrap(),
            BoxMonSpeciesFull::Nincada
        );

        assert_eq!(
            BoxMonSpeciesFull::try_from("WHISMUR").unwrap(),
            BoxMonSpeciesFull::Whismur
        );

        assert_eq!(
            BoxMonSpeciesFull::try_from("TAILLOW").unwrap(),
            BoxMonSpeciesFull::Taillow
        );
    }
}
