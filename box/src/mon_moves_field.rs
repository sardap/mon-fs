use bit_vec::BitVec;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, IntoEnumIterator};
use strum_macros::FromRepr;

use crate::{
    box_mon::StringMonParseError,
    mon_field::{
        BitCount, FromGameValueError, FromRepresentation, FromStringInput, GameSerializer,
        ToGameValueError,
    },
    BoxMonBitVec,
};

#[derive(FromRepr, Default, Debug, Clone, Copy, EnumCount, EnumIter, PartialEq, Eq, Display)]
#[repr(u8)]
pub enum BoxMonMove {
    #[default]
    Pound,
    KarateChop,
    DoubleSlap,
    CometPunch,
    MegaPunch,
    PayDay,
    FirePunch,
    IcePunch,
    ThunderPunch,
    Scratch,
    ViceGrip,
    Guillotine,
    RazorWind,
    SwordsDance,
    Cut,
    Gust,
    WingAttack,
    Whirlwind,
    Fly,
    Bind,
    Slam,
    VineWhip,
    Stomp,
    DoubleKick,
    MegaKick,
    JumpKick,
    RollingKick,
    SandAttack,
    Headbutt,
    HornAttack,
    FuryAttack,
    HornDrill,
    Tackle,
    BodySlam,
    Wrap,
    TakeDown,
    Thrash,
    DoubleEdge,
    TailWhip,
    PoisonSting,
    Twineedle,
    PinMissile,
    Leer,
    Bite,
    Growl,
    Roar,
    Sing,
    Supersonic,
    SonicBoom,
    Disable,
    Acid,
    Ember,
    Flamethrower,
    Mist,
    WaterGun,
    HydroPump,
    Surf,
    IceBeam,
    Blizzard,
    Psybeam,
    BubbleBeam,
    AuroraBeam,
    HyperBeam,
    Peck,
    DrillPeck,
    Submission,
    LowKick,
    Counter,
    SeismicToss,
    Strength,
    Absorb,
    MegaDrain,
    LeechSeed,
    Growth,
    RazorLeaf,
    SolarBeam,
    PoisonPowder,
    StunSpore,
    SleepPowder,
    PetalDance,
    StringShot,
    DragonRage,
    FireSpin,
    ThunderShock,
    Thunderbolt,
    ThunderWave,
    Thunder,
    RockThrow,
    Earthquake,
    Fissure,
    Dig,
    Toxic,
    Confusion,
    Psychic,
    Hypnosis,
    Meditate,
    Agility,
    QuickAttack,
    Rage,
    Teleport,
    NightShade,
    Mimic,
    Screech,
    DoubleTeam,
    Recover,
    Harden,
    Minimize,
    Smokescreen,
    ConfuseRay,
    Withdraw,
    DefenseCurl,
    Barrier,
    LightScreen,
    Haze,
    Reflect,
    FocusEnergy,
    Bide,
    Metronome,
    MirrorMove,
    SelfDestruct,
    EggBomb,
    Lick,
    Smog,
    Sludge,
    BoneClub,
    FireBlast,
    Waterfall,
    Clamp,
    Swift,
    SkullBash,
    SpikeCannon,
    Constrict,
    Amnesia,
    Kinesis,
    SoftBoiled,
    HiJumpKick,
    Glare,
    DreamEater,
    PoisonGas,
    Barrage,
    LeechLife,
    LovelyKiss,
    SkyAttack,
    Transform,
    Bubble,
    DizzyPunch,
    Spore,
    Flash,
    Psywave,
    Splash,
    AcidArmor,
    Crabhammer,
    Explosion,
    FurySwipes,
    Bonemerang,
    Rest,
    RockSlide,
    HyperFang,
    Sharpen,
    Conversion,
    TriAttack,
    SuperFang,
    Slash,
    Substitute,
    Sketch,
    TripleKick,
    Thief,
    SpiderWeb,
    MindReader,
    Nightmare,
    FlameWheel,
    Snore,
    Curse,
    Flail,
    Conversion2,
    Aeroblast,
    CottonSpore,
    Reversal,
    Spite,
    PowderSnow,
    Protect,
    MachPunch,
    ScaryFace,
    FaintAttack,
    SweetKiss,
    BellyDrum,
    SludgeBomb,
    MudSlap,
    Octazooka,
    Spikes,
    ZapCannon,
    Foresight,
    DestinyBond,
    PerishSong,
    IcyWind,
    Detect,
    BoneRush,
    LockOn,
    Outrage,
    Sandstorm,
    GigaDrain,
    Endure,
    Charm,
    Rollout,
    FalseSwipe,
    Swagger,
    MilkDrink,
    Spark,
    FuryCutter,
    SteelWing,
    MeanLook,
    Attract,
    SleepTalk,
    HealBell,
    Return,
    Present,
    Frustration,
    Safeguard,
    PainSplit,
    SacredFire,
    Magnitude,
    DynamicPunch,
    Megahorn,
    DragonBreath,
    BatonPass,
    Encore,
    Pursuit,
    RapidSpin,
    SweetScent,
    IronTail,
    MetalClaw,
    VitalThrow,
    MorningSun,
    Synthesis,
    Moonlight,
    HiddenPower,
    CrossChop,
    Twister,
    RainDance,
    SunnyDay,
    Crunch,
    MirrorCoat,
    PsychUp,
    ExtremeSpeed,
    AncientPower,
    ShadowBall,
    FutureSight,
    RockSmash,
    Whirlpool,
    BeatUp,
    FakeOut,
    Uproar,
    Stockpile,
    SpitUp,
    Swallow,
    HeatWave,
}

impl Serialize for BoxMonMove {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BoxMonMove {
    fn deserialize<D>(deserializer: D) -> Result<BoxMonMove, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let input = String::deserialize(deserializer)?;
        match BoxMonMove::try_from_string(&input) {
            Some(name) => Ok(name),
            None => Err(serde::de::Error::custom("Invalid BoxMonMoveSet")),
        }
    }
}

impl FromStringInput for BoxMonMove {
    fn try_from_string(input: &str) -> Option<Self> {
        lazy_static! {
            static ref STRING_TO_ENUM: std::collections::HashMap<String, BoxMonMove> = {
                let mut m = std::collections::HashMap::new();
                for variant in BoxMonMove::iter() {
                    m.insert(variant.to_string().to_uppercase(), variant);
                }
                m
            };
        }

        let input = input.to_uppercase();
        STRING_TO_ENUM.get(&input).copied()
    }
}

impl FromRepresentation for BoxMonMove {
    fn from_repr(repr: u8) -> Option<Self> {
        Self::from_repr(repr)
    }

    fn to_u8(&self) -> u8 {
        *self as u8
    }
}

const MOVE_SET_COUNT: usize = 4;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoxMonMoveSet([BoxMonMove; MOVE_SET_COUNT]);

impl BoxMonMoveSet {
    pub fn new(moves: [BoxMonMove; MOVE_SET_COUNT]) -> Self {
        BoxMonMoveSet(moves)
    }
}

impl TryFrom<Vec<String>> for BoxMonMoveSet {
    type Error = StringMonParseError;

    fn try_from(input: Vec<String>) -> Result<Self, Self::Error> {
        if input.len() != MOVE_SET_COUNT {
            return Err(StringMonParseError::InvalidMoves(input));
        }

        let mut moves = [BoxMonMove::default(); MOVE_SET_COUNT];
        for (i, split) in input.iter().enumerate() {
            let mon_move = match BoxMonMove::try_from_string(split) {
                Some(mon_move) => mon_move,
                None => return Err(StringMonParseError::InvalidMoves(input)),
            };
            moves[i] = mon_move;
        }

        Ok(BoxMonMoveSet(moves))
    }
}

impl ToString for BoxMonMoveSet {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|box_mon_move| box_mon_move.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl BitCount for BoxMonMoveSet {
    fn bit_count() -> usize {
        BoxMonMove::bit_count() * MOVE_SET_COUNT
    }
}

impl GameSerializer for BoxMonMoveSet {
    fn bits_to_game_value(value: &BoxMonBitVec) -> Result<Self, ToGameValueError>
    where
        Self: Sized,
    {
        if value.0.len() != Self::bit_count() {
            return Err(ToGameValueError::BadBitsLength);
        }

        let mut move_set = [BoxMonMove::default(); MOVE_SET_COUNT];
        for i in 0..MOVE_SET_COUNT {
            let start = i * BoxMonMove::bit_count();
            let end = (i + 1) * BoxMonMove::bit_count();
            let move_bits = value.chunk(start, end);
            let box_mon_move = match BoxMonMove::bits_to_game_value(&move_bits) {
                Ok(box_mon_move) => box_mon_move,
                Err(err) => return Err(err),
            };
            move_set[i] = box_mon_move;
        }

        Ok(BoxMonMoveSet(move_set))
    }

    fn game_value_to_bits(&self) -> Result<BoxMonBitVec, FromGameValueError>
    where
        Self: Sized,
    {
        let mut bits = BitVec::new();
        for i in 0..MOVE_SET_COUNT {
            let box_mon_move = self.0[i];
            let mon_move_bits = match box_mon_move.game_value_to_bits() {
                Ok(bits) => bits,
                Err(err) => return Err(err),
            };
            bits.extend(mon_move_bits.0.iter());
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
    fn test_move_to_and_from() {
        assert_eq!(BoxMonMove::possible_values(), 256);

        for i in 0..BoxMonMove::possible_values() as u8 {
            let starting = BoxMonBitVec::new(BoxMonMove::bit_count(), i);
            let box_mon_move = BoxMonMove::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_move.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u8());
        }
    }

    #[test]
    fn test_move_set_to_and_from() {
        let count = BoxMonMoveSet::possible_values() as u64;
        let mut rng = ChaCha8Rng::seed_from_u64(3);

        let mut chosen = vec![];
        for _ in 0..1000 {
            chosen.push(rng.gen::<u64>() % count);
        }

        for i in chosen {
            let starting = BoxMonBitVec::new(BoxMonMoveSet::bit_count(), i);
            let box_mon_gender = BoxMonMoveSet::bits_to_game_value(&starting).unwrap();
            let decoded = box_mon_gender.game_value_to_bits().unwrap();
            assert_eq!(starting, decoded);
            assert_eq!(i, decoded.as_u64());
        }
    }

    #[test]
    fn test_encode_and_decode() {
        let move_set = BoxMonMoveSet::new([
            BoxMonMove::Pound,
            BoxMonMove::Headbutt,
            BoxMonMove::Dig,
            BoxMonMove::RazorWind,
        ]);

        let bits = move_set.game_value_to_bits().unwrap();
        let decoded = BoxMonMoveSet::bits_to_game_value(&bits).unwrap();

        assert_eq!(move_set, decoded);
    }

    #[test]
    fn test_move_set_to_and_from_string() {
        let moves = vec![
            "Pound".to_string(),
            "Headbutt".to_string(),
            "Dig".to_string(),
            "RazorWind".to_string(),
        ];
        let box_mon_name: BoxMonMoveSet = moves.try_into().unwrap();
        assert_eq!(
            box_mon_name.0.to_vec(),
            vec![
                BoxMonMove::Pound,
                BoxMonMove::Headbutt,
                BoxMonMove::Dig,
                BoxMonMove::RazorWind
            ]
        )
    }
}
