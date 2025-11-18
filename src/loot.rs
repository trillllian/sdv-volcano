use crate::{GameSettings, format_icon, rng};
use std::fmt::Display;

// type definitions

/// (Innate) enchantment type
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Enchant {
    Defense,
    Weight,
    SlimeGatherer,
    SlimeSlayer,
    CritPower,
    CritChance,
    Attack,
    Speed,
}

/// List of (enchant, level). The levels are stored as whatever number is shown in the UI, not the
/// "real" internal level number. (i.e. 25-75 for crit.power, and negative for weight)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Enchants(Vec<(Enchant, i32)>);

/// Possible contents of one chest (common or rare)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ChestItem {
    CinderShards3,
    GoldenCoconut,
    TaroTuber,
    PineappleSeeds,
    ProtectionRing,
    SoulSapperRing,
    DwarfSword(Enchants),
    DwarfHammer(Enchants),
    DwarfDagger(Enchants),

    CinderShards10,
    MermaidBoots,
    DragonscaleBoots,
    GoldenCoconuts,
    PhoenixRing,
    HotJavaRing,
    DragontoothCutlass(Enchants),
    DragontoothClub(Enchants),
    DragontoothShiv(Enchants),
    DeluxePirateHat,
    OstrichEgg,
}

/// One row of the loot overview: dragon tooth or chest contents
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

// Display impls / html rendering stuff

impl Display for Enchants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }
        write!(f, " (")?;
        for (i, &(e, lvl)) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            if lvl != 0 {
                write!(f, "{:+} ", lvl)?;
            }
            f.write_str(match e {
                Enchant::Defense => "Defense",
                Enchant::Weight => "Weight",
                Enchant::SlimeGatherer => "Slime Gatherer",
                Enchant::SlimeSlayer => "Slime Slayer",
                Enchant::CritPower => "Crit. Power",
                Enchant::CritChance => "Crit. Chance",
                Enchant::Attack => "Attack",
                Enchant::Speed => "Speed",
            })?;
        }
        write!(f, ")")
    }
}

impl Display for ChestItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CinderShards3 => write!(f, "Cinder Shard (3)"),
            Self::GoldenCoconut => write!(f, "Golden Coconut"),
            Self::TaroTuber => write!(f, "Taro Tuber (8)"),
            Self::PineappleSeeds => write!(f, "Pineapple Seeds (5)"),
            Self::ProtectionRing => write!(f, "Protection Ring"),
            Self::SoulSapperRing => write!(f, "Soul Sapper Ring"),
            Self::DwarfSword(e) => write!(f, "Dwarf Sword{}", e),
            Self::DwarfHammer(e) => write!(f, "Dwarf Hammer{}", e),
            Self::DwarfDagger(e) => write!(f, "Dwarf Dagger{}", e),

            Self::CinderShards10 => write!(f, "Cinder Shard (10)"),
            Self::MermaidBoots => write!(f, "Mermaid Boots"),
            Self::DragonscaleBoots => write!(f, "Dragonscale Boots"),
            Self::GoldenCoconuts => write!(f, "Golden Coconut (3)"),
            Self::PhoenixRing => write!(f, "Phoenix Ring"),
            Self::HotJavaRing => write!(f, "Hot Java Ring"),
            Self::DragontoothCutlass(e) => write!(f, "Dragontooth Cutlass{}", e),
            Self::DragontoothClub(e) => write!(f, "Dragontooth Club{}", e),
            Self::DragontoothShiv(e) => write!(f, "Dragontooth Shiv{}", e),
            Self::DeluxePirateHat => write!(f, "Deluxe Pirate Hat"),
            Self::OstrichEgg => write!(f, "Ostrich Egg"),
        }
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
            Self::DwarfSword(_) => "dwarf_sword",
            Self::DwarfHammer(_) => "dwarf_hammer",
            Self::DwarfDagger(_) => "dwarf_dagger",

            Self::CinderShards10 => "cinder_shard",
            Self::MermaidBoots => "mermaid_boots",
            Self::DragonscaleBoots => "dragonscale_boots",
            Self::GoldenCoconuts => "golden_coconut",
            Self::PhoenixRing => "phoenix_ring",
            Self::HotJavaRing => "hot_java_ring",
            Self::DragontoothCutlass(_) => "dragontooth_cutlass",
            Self::DragontoothClub(_) => "dragontooth_club",
            Self::DragontoothShiv(_) => "dragontooth_shiv",
            Self::DeluxePirateHat => "deluxe_pirate_hat",
            Self::OstrichEgg => "ostrich_egg",
        }
    }
}

impl Display for Goodie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // this impl is only used for debugging
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
            Goodie::ChanceChest { .. } => panic!("Turning chance_chest into html"),
        }
    }
}

// actual generation logic

/// apply innate enchantments to a weapon
fn enchant_item(rng: &mut rng::DotnetRng, weapon_lvl: i32, weapon_speed: i32) -> Enchants {
    // based on StardewValley.Tools.MeleeWeapon.attemptAddRandomInnateEnchantment
    let mut enchs = vec![];
    if rng.next_f64() < 0.5 {
        if rng.next_f64() < 0.125 && weapon_lvl <= 10 {
            let lvl = (rng.next_range(weapon_lvl + 1) / 2 + 1).min(2).max(1);
            enchs.push((Enchant::Defense, lvl));
        } else if rng.next_f64() < 0.125 {
            enchs.push((Enchant::Weight, -(1 + rng.next_range(5))));
        } else if rng.next_f64() < 0.125 {
            enchs.push((Enchant::SlimeGatherer, 0));
        }

        let last_one = match rng.next_range(5) {
            0 => (
                Enchant::Attack,
                (rng.next_range(weapon_lvl + 1) / 2 + 1).clamp(1, 5),
            ),
            1 => (
                Enchant::CritChance,
                (rng.next_range(weapon_lvl) / 3).clamp(1, 3),
            ),
            2 => (
                Enchant::Speed,
                (rng.next_range(weapon_lvl)
                    .clamp(1, i32::max(1, 4 - weapon_speed))),
            ),
            3 => (Enchant::SlimeSlayer, 0),
            4 => (
                Enchant::CritPower,
                (rng.next_range(weapon_lvl) / 3).clamp(1, 3) * 25,
            ),
            _ => unreachable!(),
        };
        enchs.push(last_one);
    }
    Enchants(enchs)
}

impl ChestItem {
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
            6 => match rng.next_range(3) {
                0 => Self::DwarfSword(enchant_item(&mut rng, 13, 4)),
                1 => Self::DwarfHammer(enchant_item(&mut rng, 13, -8)),
                2 => Self::DwarfDagger(enchant_item(&mut rng, 11, 3)),
                _ => unreachable!(),
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
            6 => match rng.next_range(3) {
                0 => Self::DragontoothCutlass(enchant_item(&mut rng, 13, 0)),
                1 => Self::DragontoothClub(enchant_item(&mut rng, 14, -8)),
                2 => Self::DragontoothShiv(enchant_item(&mut rng, 12, 0)),
                _ => unreachable!(),
            }
            7 => Self::DeluxePirateHat,
            8 => Self::OstrichEgg,
            _ => unreachable!(),
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
}
