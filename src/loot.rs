use crate::{GameSettings, format_icon, rng};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ChestItem {
    CinderShards3,
    GoldenCoconut,
    TaroTuber,
    PineappleSeeds,
    ProtectionRing,
    SoulSapperRing,
    DwarfSword,
    DwarfHammer,
    DwarfDagger,

    CinderShards10,
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

impl Display for ChestItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::CinderShards3 => "Cinder Shard (3)",
            Self::GoldenCoconut => "Golden Coconut",
            Self::TaroTuber => "Taro Tuber (8)",
            Self::PineappleSeeds => "Pineapple Seeds (5)",
            Self::ProtectionRing => "Protection Ring",
            Self::SoulSapperRing => "Soul Sapper Ring",
            Self::DwarfSword => "Dwarf Sword",
            Self::DwarfHammer => "Dwarf Hammer",
            Self::DwarfDagger => "Dwarf Dagger",

            Self::CinderShards10 => "Cinder Shard (10)",
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

impl ChestItem {
    fn get_icon(&self) -> &'static str {
        match self {
            Self::CinderShards3 => "cinder_shard",
            Self::GoldenCoconut => "golden_coconut",
            Self::TaroTuber => "taro_tuber",
            Self::PineappleSeeds => "pineapple_seeds",
            Self::ProtectionRing => "protection_ring",
            Self::SoulSapperRing => "soul_sapper_ring",
            Self::DwarfSword => "dwarf_sword",
            Self::DwarfHammer => "dwarf_hammer",
            Self::DwarfDagger => "dwarf_dagger",

            Self::CinderShards10 => "cinder_shard",
            Self::MermaidBoots => "mermaid_boots",
            Self::DragonscaleBoots => "dragonscale_boots",
            Self::GoldenCoconuts => "golden_coconut",
            Self::PhoenixRing => "phoenix_ring",
            Self::HotJavaRing => "hot_java_ring",
            Self::DragontoothCutlass => "dragontooth_cutlass",
            Self::DragontoothClub => "dragontooth_club",
            Self::DragontoothShiv => "dragontooth_shiv",
            Self::DeluxePirateHat => "deluxe_pirate_hat",
            Self::OstrichEgg => "ostrich_egg",
        }
    }

    pub fn generate_common(seed: i32, settings: GameSettings) -> Self {
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
            0 => Self::CinderShards3,
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

    pub fn generate_rare(seed: i32, settings: GameSettings) -> Self {
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
            0 => Self::CinderShards10,
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
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Goodie {
    DragonTooth,
    CommonChest(ChestItem),
    RareChest(ChestItem),
    ChanceChest {
        minluck: f64,
        common: ChestItem,
        rare: ChestItem,
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
    pub fn generate(
        chest_seed: i32,
        settings: GameSettings,
        level: i32,
        min_luck: f64,
        max_luck: f64,
    ) -> Self {
        let mut chest_rng = rng::DotnetRng::new(chest_seed);
        // roll < (0.1 or 0.5) + luckboost
        // roll - (0.1 or 0.5) < luckboost
        // roll - (0.1 or 0.5) < luckmult-1
        // roll - (0.1 or 0.5) + 1 < luckmult
        // (though that technically rounds different..)
        let chest_roll = chest_rng.next_f64() - if level == 9 { 0.5 } else { 0.1 } + 1.;
        if chest_roll < min_luck {
            // only rare
            Goodie::RareChest(ChestItem::generate_rare(chest_seed, settings))
        } else if chest_roll >= max_luck {
            // only common
            Goodie::CommonChest(ChestItem::generate_common(chest_seed, settings))
        } else {
            // both possible
            Goodie::ChanceChest {
                minluck: chest_roll,
                common: ChestItem::generate_common(chest_seed, settings),
                rare: ChestItem::generate_rare(chest_seed, settings),
            }
        }
    }

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
