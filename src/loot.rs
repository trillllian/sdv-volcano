use crate::{GameSettings, format_icon, rng};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum CommonChest {
    CinderShards,
    GoldenCoconut,
    TaroTuber,
    PineappleSeeds,
    ProtectionRing,
    SoulSapperRing,
    DwarfSword,
    DwarfHammer,
    DwarfDagger,
}

impl Display for CommonChest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::CinderShards => "Cinder Shard (3)",
            Self::GoldenCoconut => "Golden Coconut",
            Self::TaroTuber => "Taro Tuber (8)",
            Self::PineappleSeeds => "Pineapple Seeds (5)",
            Self::ProtectionRing => "Protection Ring",
            Self::SoulSapperRing => "Soul Sapper Ring",
            Self::DwarfSword => "Dwarf Sword",
            Self::DwarfHammer => "Dwarf Hammer",
            Self::DwarfDagger => "Dwarf Dagger",
        })
    }
}

impl CommonChest {
    pub fn generate(seed: i32, settings: GameSettings) -> Self {
        let mut rng = rng::DotnetRng::new(seed);
        rng.next(); // one roll used for rare/normal check
        let ind = loop {
            let ind = rng.next_range(7);
            if ind == 1 && !settings.cracked_golden_coconut {
                continue;
            }
            break ind;
        };
        match ind {
            0 => Self::CinderShards,
            1 => Self::GoldenCoconut,
            2 => Self::TaroTuber,
            3 => Self::PineappleSeeds,
            4 => Self::ProtectionRing,
            5 => Self::SoulSapperRing,
            6 => {
                [Self::DwarfSword, Self::DwarfHammer, Self::DwarfDagger][rng.next_range(3) as usize]
            }
            _ => unreachable!(),
        }
    }
    fn get_icon(&self) -> &'static str {
        match self {
            CommonChest::CinderShards => "cinder_shard",
            CommonChest::GoldenCoconut => "golden_coconut",
            CommonChest::TaroTuber => "taro_tuber",
            CommonChest::PineappleSeeds => "pineapple_seeds",
            CommonChest::ProtectionRing => "protection_ring",
            CommonChest::SoulSapperRing => "soul_sapper_ring",
            CommonChest::DwarfSword => "dwarf_sword",
            CommonChest::DwarfHammer => "dwarf_hammer",
            CommonChest::DwarfDagger => "dwarf_dagger",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum RareChest {
    CinderShards,
    MermaidBoots,
    DragonscaleBoots,
    GoldenCoconuts,
    PhoenixRing,
    HotJavaRing,
    DragontoothCutlass,
    DragontoothClub,
    DragontoothShiv,
    DeluxePirateHat,
    OstrichEgg,
}

impl Display for RareChest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::CinderShards => "Cinder Shard (10)",
            Self::MermaidBoots => "Mermaid Boots",
            Self::DragonscaleBoots => "Dragonscale Boots",
            Self::GoldenCoconuts => "Golden Coconut (3)",
            Self::PhoenixRing => "Phoenix Ring",
            Self::HotJavaRing => "Hot Java Ring",
            Self::DragontoothCutlass => "Dragontooth Cutlass",
            Self::DragontoothClub => "Dragontooth Club",
            Self::DragontoothShiv => "Dragontooth Shiv",
            Self::DeluxePirateHat => "Deluxe Pirate Hat",
            Self::OstrichEgg => "Ostrich Egg",
        })
    }
}

impl RareChest {
    pub fn generate(seed: i32, settings: GameSettings) -> Self {
        let mut rng = rng::DotnetRng::new(seed);
        rng.next(); // one roll used for rare/normal check
        let ind = loop {
            let ind = rng.next_range(9);
            if ind == 3 && !settings.cracked_golden_coconut {
                continue;
            }
            break ind;
        };
        match ind {
            0 => Self::CinderShards,
            1 => Self::MermaidBoots,
            2 => Self::DragonscaleBoots,
            3 => Self::GoldenCoconuts,
            4 => Self::PhoenixRing,
            5 => Self::HotJavaRing,
            6 => [
                Self::DragontoothCutlass,
                Self::DragontoothClub,
                Self::DragontoothShiv,
            ][rng.next_range(3) as usize],
            7 => Self::DeluxePirateHat,
            8 => Self::OstrichEgg,
            _ => unreachable!(),
        }
    }

    fn get_icon(&self) -> &'static str {
        match self {
            RareChest::CinderShards => "cinder_shard",
            RareChest::MermaidBoots => "mermaid_boots",
            RareChest::DragonscaleBoots => "dragonscale_boots",
            RareChest::GoldenCoconuts => "golden_coconut",
            RareChest::PhoenixRing => "phoenix_ring",
            RareChest::HotJavaRing => "hot_java_ring",
            RareChest::DragontoothCutlass => "dragontooth_cutlass",
            RareChest::DragontoothClub => "dragontooth_club",
            RareChest::DragontoothShiv => "dragontooth_shiv",
            RareChest::DeluxePirateHat => "deluxe_pirate_hat",
            RareChest::OstrichEgg => "ostrich_egg",
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Goodie {
    DragonTooth,
    CommonChest(CommonChest),
    RareChest(RareChest),
    ChanceChest {
        minluck: f64,
        common: CommonChest,
        rare: RareChest,
    },
}

impl Display for Goodie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Goodie::DragonTooth => write!(f, "Dragon Tooth"),
            Goodie::CommonChest(c) => write!(f, "common chest: {}", c),
            Goodie::RareChest(c) => write!(f, "rare chest: {}", c),
            Goodie::ChanceChest {
                minluck,
                common,
                rare,
            } => {
                write!(
                    f,
                    "luck boost > {:.4}: rare: {}, else common: {}",
                    minluck, rare, common
                )
            }
        }
    }
}

impl Goodie {
    pub fn to_html(&self) -> String {
        match self {
            Goodie::DragonTooth => format!("{} Dragon Tooth", format_icon("dragon_tooth")),
            Goodie::CommonChest(c) => {
                format!(
                    "{} {} {}",
                    format_icon("common_chest"),
                    format_icon(c.get_icon()),
                    c
                )
            }
            Goodie::RareChest(c) => {
                format!(
                    "{} {} {}",
                    format_icon("rare_chest"),
                    format_icon(c.get_icon()),
                    c
                )
            }
            // shouldn't ever be turned into html
            Goodie::ChanceChest { .. } => self.to_string(),
        }
    }
}
