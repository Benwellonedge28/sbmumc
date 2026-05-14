//! # SBMUMC Module 1527: Enchantment Craft
//!
//! Systems for enchantment craft and magical infusion.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnchantmentCraftTopic {
    MagicalInfusion,
    ObjectEnchantment,
    ItemEnchanting,
    SpellInfusion,
    MagicalBinding,
    EnchantedCreation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnchantmentCraftSystem {
    pub system_id: String,
    pub enchantment_craft_topic: EnchantmentCraftTopic,
    pub magical_enchantment: f64,
    pub enchanted_items: f64,
    pub spell_infusion: f64,
    pub magical_crafting: f64,
}

impl EnchantmentCraftSystem {
    pub fn new(enchantment_craft_topic: EnchantmentCraftTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            enchantment_craft_topic,
            magical_enchantment: 0.0,
            enchanted_items: 0.0,
            spell_infusion: 0.0,
            magical_crafting: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.enchantment_craft_topic {
            EnchantmentCraftTopic::MagicalInfusion => {
                self.magical_enchantment = 0.95 + rand_simple() * 0.05;
                self.enchanted_items = 0.90 + rand_simple() * 0.10;
                self.spell_infusion = 0.85 + rand_simple() * 0.14;
            },
            EnchantmentCraftTopic::ObjectEnchantment => {
                self.magical_crafting = 0.95 + rand_simple() * 0.05;
                self.spell_infusion = 0.90 + rand_simple() * 0.10;
                self.enchanted_items = 0.85 + rand_simple() * 0.14;
            },
            EnchantmentCraftTopic::ItemEnchanting => {
                self.enchanted_items = 0.95 + rand_simple() * 0.05;
                self.magical_enchantment = 0.90 + rand_simple() * 0.10;
                self.magical_crafting = 0.85 + rand_simple() * 0.14;
            },
            EnchantmentCraftTopic::SpellInfusion => {
                self.spell_infusion = 0.95 + rand_simple() * 0.05;
                self.magical_crafting = 0.90 + rand_simple() * 0.10;
                self.magical_enchantment = 0.85 + rand_simple() * 0.14;
            },
            EnchantmentCraftTopic::MagicalBinding => {
                self.magical_enchantment = 0.95 + rand_simple() * 0.05;
                self.enchanted_items = 0.90 + rand_simple() * 0.10;
                self.magical_crafting = 0.85 + rand_simple() * 0.14;
            },
            EnchantmentCraftTopic::EnchantedCreation => {
                self.magical_crafting = 0.95 + rand_simple() * 0.05;
                self.spell_infusion = 0.90 + rand_simple() * 0.10;
                self.enchanted_items = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.spell_infusion == 0.0 {
            self.spell_infusion = (self.magical_enchantment + self.enchanted_items) / 2.0 * (0.6 + rand_simple() * 0.3);
        }
        Ok(())
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magical_infusion() {
        let mut system = EnchantmentCraftSystem::new(EnchantmentCraftTopic::MagicalInfusion);
        system.analyze_system().unwrap();
        assert!(system.magical_enchantment > 0.8);
    }
}