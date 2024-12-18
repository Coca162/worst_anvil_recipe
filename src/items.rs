use std::iter;

use crate::enchantments::{AnvilError, Enchant, Enchantment, Enchants, ItemType};

fn penalty(anvil_uses: u32) -> u32 {
    (1 << anvil_uses) - 1
}

#[derive(Debug, Clone, Default)]
pub struct Item {
    enchants: Enchants,
    r#type: ItemType,
    recipe: Option<(Box<Item>, Box<Item>)>,
    anvil_uses: u32,
    pub total_cost: u32,
    cost: u32,
}

impl Item {
    pub fn from_enchantment(enchantment: Enchantment, r#type: ItemType) -> Self {
        Self {
            enchants: Enchants::new(iter::once(Enchant::new(enchantment, 1))),
            r#type,
            recipe: None,
            anvil_uses: 0,
            total_cost: 0,
            cost: 0,
        }
    }

    pub fn combine(self, sacrifice: Self) -> Result<Self, AnvilError> {
        let mut enchants = self.enchants.clone();

        let mut cost = enchants.combine((&sacrifice.enchants, sacrifice.r#type))?;

        cost += penalty(self.anvil_uses);
        cost += penalty(sacrifice.anvil_uses);

        if cost > 40 {
            return Err(AnvilError::TooExpensive);
        }

        Ok(Self {
            enchants: self.enchants.clone(),
            r#type: self.r#type,
            anvil_uses: sacrifice.anvil_uses.max(sacrifice.anvil_uses) + 1,
            total_cost: self.total_cost + cost,
            recipe: Some((Box::new(self), Box::new(sacrifice))),
            cost,
        })
    }
}
