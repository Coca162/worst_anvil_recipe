use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Enchants {
    inclusive: HashMap<EnchantTypes, Enchant>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Enchant {
    enchantment: Enchantment,
    level: u32,
}

impl Enchant {
    pub fn new(enchantment: Enchantment, level: u32) -> Self {
        Self { enchantment, level }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Enchantment {
    pub name: EnchantTypes,
    pub max_level: u32,
    pub item_mult: u32,
    pub book_mult: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ItemType {
    #[default]
    Other,
    Book,
}

impl Enchants {
    pub fn new(enchants: impl IntoIterator<Item = Enchant>) -> Self {
        Self {
            inclusive: enchants
                .into_iter()
                .map(|e| (e.enchantment.name, e))
                .collect(),
        }
    }

    pub fn combine(&mut self, (other, r#type): (&Self, ItemType)) -> Result<u32, AnvilError> {
        let mut cost: u32 = 0;

        for enchant in other.inclusive.values() {
            match self.inclusive.entry(enchant.enchantment.name) {
                Entry::Occupied(occupied_entry) => {
                    cost += occupied_entry.into_mut().combine(r#type, enchant.level)?
                }
                Entry::Vacant(vacant_entry) => {
                    cost += enchant.enchantment.calculate_score(r#type, enchant.level);
                    vacant_entry.insert(enchant.clone());
                }
            }
        }

        Ok(cost)
    }
}

impl Enchant {
    pub fn combine(&mut self, r#type: ItemType, level: u32) -> Result<u32, AnvilError> {
        if level > self.level && level <= self.enchantment.max_level {
            self.level = level;
        } else if level == self.level && level < self.enchantment.max_level {
            self.level += 1;
        } else {
            return Err(AnvilError::RedundantSequence);
        }

        Ok(self.enchantment.calculate_score(r#type, self.level))
    }
}

impl Enchantment {
    pub const fn new(name: EnchantTypes, max_level: u32, item_mult: u32, book_mult: u32) -> Self {
        Self {
            name,
            max_level,
            item_mult,
            book_mult,
        }
    }

    pub fn calculate_score(&self, r#type: ItemType, levels: u32) -> u32 {
        let multiplier = match r#type {
            ItemType::Other => self.item_mult,
            ItemType::Book => self.book_mult,
        };

        levels * multiplier
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AnvilError {
    RedundantSequence,
    TooExpensive,
}

// Protection Exclusives
const BLAST_PROTECTION: Enchantment = Enchantment::new(EnchantTypes::BlastProtection, 4, 4, 2);
const FIRE_PROTECTION: Enchantment = Enchantment::new(EnchantTypes::FireProtection, 4, 2, 1);
const PROTECTION: Enchantment = Enchantment::new(EnchantTypes::Protection, 4, 1, 1);
// Practically the same as Fire Protection
// const PROJECTILE_PROTECTION: Enchantment<ProtectionExclusive> = Enchantment::new(EnchantTypes::ProjectileProtection, 4, 2, 1);

pub const PROTECTIONS: [Enchantment; 3] = [BLAST_PROTECTION, FEATHER_FALLING, PROTECTION];

// For the purposes of this frost walker is strictly worse then
// depth strider due to having the same mulitlpiers and a lower max level
// const FROST_WALKER: Enchantment<WaterExclusive> = Enchantment::new(EnchantTypes::FrostWalker, 2, 4, 2);
const DEPTH_STRIDER: Enchantment = Enchantment::new(EnchantTypes::DepthStrider, 3, 4, 2);

// Inclusive
// Helmet
const AQUA_AFFINITY: Enchantment = Enchantment::new(EnchantTypes::AquaAffinity, 1, 4, 2);
const RESPIRATION: Enchantment = Enchantment::new(EnchantTypes::Respiration, 3, 4, 2);
// Leggings
const SWIFT_SNEAK: Enchantment = Enchantment::new(EnchantTypes::SwiftSneak, 3, 8, 4);
// Boots
const SOUL_SPEED: Enchantment = Enchantment::new(EnchantTypes::SoulSpeed, 3, 8, 4);
const FEATHER_FALLING: Enchantment = Enchantment::new(EnchantTypes::FeatherFalling, 4, 2, 1);

// Generic
const MENDING: Enchantment = Enchantment::new(EnchantTypes::Mending, 1, 4, 2);
const THORNS: Enchantment = Enchantment::new(EnchantTypes::Thorns, 3, 8, 4);
const UNBREAKING: Enchantment = Enchantment::new(EnchantTypes::Unbreaking, 3, 2, 1);
const CURSE_OF_BINDING: Enchantment = Enchantment::new(EnchantTypes::CurseOfBinding, 1, 8, 4);
const CURSE_OF_VANISHING: Enchantment = Enchantment::new(EnchantTypes::CurseOfVanishing, 1, 8, 4);

pub const INCLUSIVE_ENCHANTMENTS: [Enchantment; 5] = [
    // DEPTH_STRIDER,
    // AQUA_AFFINITY,
    // RESPIRATION,
    // SWIFT_SNEAK,
    // SOUL_SPEED,
    // FEATHER_FALLING,
    MENDING,
    THORNS,
    UNBREAKING,
    CURSE_OF_BINDING,
    CURSE_OF_VANISHING,
];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EnchantTypes {
    AquaAffinity,
    BlastProtection,
    CurseOfBinding,
    CurseOfVanishing,
    DepthStrider,
    FeatherFalling,
    FireProtection,
    FrostWalker,
    Mending,
    ProjectileProtection,
    Protection,
    Respiration,
    SoulSpeed,
    SwiftSneak,
    Thorns,
    Unbreaking,
}

#[test]
fn basic_combination() {
    let mut enchants = Enchants::new([Enchant::new(THORNS, 1)]);
    let book = Enchants::new([Enchant::new(THORNS, 1)]);

    assert_eq!(Ok(8), enchants.combine((&book, ItemType::Book)));

    assert_eq!(enchants.inclusive[&EnchantTypes::Thorns].level, 2);
}

#[test]
fn overtake() {
    let mut weaker = Enchants::new([Enchant::new(SOUL_SPEED, 1)]);
    let stronger_book = Enchants::new([Enchant::new(SOUL_SPEED, 3)]);

    assert_eq!(Ok(12), weaker.combine((&stronger_book, ItemType::Book)));

    assert_eq!(weaker.inclusive[&EnchantTypes::SoulSpeed].level, 3);
}

#[test]
fn complex() {
    let mut item1 = Enchants::new([
        Enchant::new(UNBREAKING, 2),
        Enchant::new(PROTECTION, 2),
        Enchant::new(DEPTH_STRIDER, 1),
    ]);
    let item2 = item1.clone();

    assert_eq!(Ok(17), item1.combine((&item2, ItemType::Other)));

    assert_eq!(
        item1,
        Enchants::new([
            Enchant::new(UNBREAKING, 3),
            Enchant::new(PROTECTION, 3),
            Enchant::new(DEPTH_STRIDER, 2)
        ],)
    );
}
