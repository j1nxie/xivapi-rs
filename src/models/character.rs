//! Character models.

use super::{
    free_company::FreeCompany,
    id::{AchievementId, CharacterId, FreeCompanyId},
    Member,
};
use chrono::{serde::ts_seconds, DateTime, Utc};
use ffxiv_types::{DataCenter, World};
use serde::Deserialize;
use serde_with::skip_serializing_none;
use std::{collections::BTreeMap, fmt::Display};
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Character {
    #[serde(rename = "ID")]
    pub id: CharacterId,
    pub name: String,
    pub nameday: String,
    #[serde(with = "ts_seconds")]
    pub parse_date: DateTime<Utc>,
    #[serde(rename = "PvPTeam")]
    pub pvp_team: Option<serde_json::Value>,
    pub race: Race,
    pub tribe: Tribe,
    #[serde(rename = "DC")]
    pub dc: DataCenter,
    #[serde(rename = "Server")]
    pub world: World,
    pub title: Option<u64>,
    pub town: Town,
    pub avatar: Url,
    pub bio: String,
    pub free_company_name: Option<String>,
    pub free_company_id: Option<FreeCompanyId>,
    pub gender: Gender,
    pub guardian_deity: GuardianDeity,
    pub class_jobs: Vec<ClassJob>,
    pub gear_set: GearSet,
    pub grand_company: Option<GrandCompany>,
    pub active_class_job: ClassJob,
    pub portrait: Url,
    // #[serde(flatten)]
    // pub verification: Verification,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CharacterResult {
    pub achievements: Option<AchievementInfo>,
    pub achievements_public: Option<AchievementInfo>,
    pub character: Option<Character>,
    pub free_company: Option<FreeCompany>,
    pub free_company_members: Option<Vec<Member>>,
    pub minions: Option<Vec<MimoInfo>>,
    pub mounts: Option<Vec<MimoInfo>>,
    // FIXME: hacky type to get this compiled - i don't actually know what's the type for PvP teams here.
    pub pvp_team: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AchievementInfo {
    pub list: Vec<Achievement>,
    pub points: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MimoInfo {
    pub icon: Url,
    pub name: String,
}

#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Achievement {
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub id: AchievementId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassJob {
    #[serde(rename = "ClassID")]
    pub class_id: u64,
    #[serde(rename = "JobID")]
    pub job_id: u64,
    pub level: u64,
    pub exp_level: u64,
    pub exp_level_max: u64,
    pub exp_level_togo: u64,
    pub is_specialised: bool,
    pub name: String,
    pub unlocked_state: UnlockedState,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UnlockedState {
    #[serde(rename = "ID")]
    pub id: Option<u64>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearSet {
    #[serde(rename = "ClassID")]
    pub class_id: u64,
    #[serde(rename = "JobID")]
    pub job_id: u64,
    pub level: u64,
    pub gear_key: String,
    pub attributes: BTreeMap<Attribute, u64>,
    pub gear: BTreeMap<GearSlot, Gear>,
}

#[derive(Debug, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum GearSlot {
    MainHand,
    Head,
    Body,
    Hands,
    Waist,
    Legs,
    Feet,
    OffHand,
    Earrings,
    Necklace,
    Bracelets,
    Ring1,
    Ring2,
    SoulCrystal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Gear {
    #[serde(rename = "ID")]
    pub id: Option<u64>,
    pub dye: Option<u64>,
    pub mirage: Option<serde_json::Value>,
    pub materia: Vec<u64>,
    pub creator: Option<u64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GrandCompany {
    #[serde(rename = "NameID")]
    name_id: u64,
    #[serde(rename = "RankID")]
    rank_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct Verification {
    #[serde(rename = "VerificationToken")]
    pub token: String,
    #[serde(rename = "VerificationTokenPass")]
    pub pass: bool,
}

enum_number!(Race {
    Hyur = 1,
    Elezen = 2,
    Lalafell = 3,
    Miqote = 4,
    Roegadyn = 5,
    AuRa = 6,
});

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hyur => write!(f, "Hyur"),
            Self::Elezen => write!(f, "Elezen"),
            Self::Lalafell => write!(f, "Lalafell"),
            Self::Miqote => write!(f, "Miqo'te"),
            Self::Roegadyn => write!(f, "Roegadyn"),
            Self::AuRa => write!(f, "Au Ra"),
        }
    }
}

enum_number!(Tribe {
    Midlander = 1,
    Highlander = 2,
    Wildwood = 3,
    Duskwight = 4,
    Plainsfolk = 5,
    Dunesfolk = 6,
    SeekerOfTheSun = 7,
    SeekerOfTheMoon = 8,
    SeaWolf = 9,
    Hellsguard = 10,
    Raen = 11,
    Xaela = 12,
    Helions = 13,
    TheLost = 14,
    Rava = 15,
    Veena = 16,
});

impl Display for Tribe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Midlander => write!(f, "Midlander"),
            Self::Highlander => write!(f, "Highlander"),
            Self::Wildwood => write!(f, "Wildwood"),
            Self::Duskwight => write!(f, "Duskwight"),
            Self::Plainsfolk => write!(f, "Plainsfolk"),
            Self::Dunesfolk => write!(f, "Dunesfolk"),
            Self::SeekerOfTheSun => write!(f, "Seeker of the Sun"),
            Self::SeekerOfTheMoon => write!(f, "Seeker of the Moon"),
            Self::SeaWolf => write!(f, "Sea Wolf"),
            Self::Hellsguard => write!(f, "Hellsguard"),
            Self::Raen => write!(f, "Raen"),
            Self::Xaela => write!(f, "Xaela"),
            Self::Helions => write!(f, "Helions"),
            Self::TheLost => write!(f, "The Lost"),
            Self::Rava => write!(f, "Rava"),
            Self::Veena => write!(f, "Veena"),
        }
    }
}

enum_number!(Town {
    LimsaLominsa = 1,
    Gridania = 2,
    UlDah = 3,
    Ishgard = 4,
    Kugane = 7,
});

impl Display for Town {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LimsaLominsa => write!(f, "Limsa Lominsa"),
            Self::Gridania => write!(f, "Gridania"),
            Self::UlDah => write!(f, "Ul'Dah"),
            Self::Ishgard => write!(f, "Ishgard"),
            Self::Kugane => write!(f, "Kugane"),
        }
    }
}

enum_number!(Gender {
    Male = 1,
    Female = 2,
});

enum_number!(GuardianDeity {
    Halone = 1,
    Menphina = 2,
    Thaliak = 3,
    Nymeia = 4,
    Llymlaen = 5,
    Oschon = 6,
    Byregot = 7,
    Rhalgr = 8,
    Azeyma = 9,
    NaldThal = 10,
    Nophica = 11,
    Althyk = 12,
});

impl Display for GuardianDeity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Halone => write!(f, "Halone"),
            Self::Menphina => write!(f, "Menphina"),
            Self::Thaliak => write!(f, "Thaliak"),
            Self::Nymeia => write!(f, "Nymeia"),
            Self::Llymlaen => write!(f, "Llymlaen"),
            Self::Oschon => write!(f, "Oschon"),
            Self::Byregot => write!(f, "Byregot"),
            Self::Rhalgr => write!(f, "Rhalgr"),
            Self::Azeyma => write!(f, "Azeyma"),
            Self::NaldThal => write!(f, "Nald'Thal"),
            Self::Nophica => write!(f, "Nophica"),
            Self::Althyk => write!(f, "Althyk"),
        }
    }
}

// FIXME: workaround for https://github.com/serde-rs/serde/issues/1183
macro_rules! special_enum_number {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum $name {
            $($variant = $value,)*
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer,
                {
                    serializer.serialize_u64(*self as u64)
                }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("positive integer")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$name, E>
                    where E: serde::de::Error,
                    {
                        let value: u64 = value
                                        .parse()
                                        .map_err(|_| E::custom(format!("expected integer, found {}", value)))?;
                        match value {
                            $( $value => Ok($name::$variant), )*
                            _ => Err(E::custom(
                                        format!("unknown {} value: {}",
                                        stringify!($name), value))),
                        }
                    }
            }

                deserializer.deserialize_str(Visitor)
            }
        }
    }
}

special_enum_number!(Attribute {
    Strength = 1,
    Dexterity = 2,
    Vitality = 3,
    Intelligence = 4,
    Mind = 5,
    Piety = 6,
    Hp = 7,
    Mp = 8,
    Tenacity = 19,
    AttackPower = 20,
    Defense = 21,
    DirectHitRate = 22,
    MagicDefense = 24,
    CriticalHitRate = 27,
    AttackMagicPotency = 33,
    HealingMagicPotency = 34,
    Determination = 44,
    SkillSpeed = 45,
    SpellSpeed = 46,
});
