//! # SBMUMC Module 1524: Symbolic Magick
//!
//! Systems for symbolic magic and sigil craft.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbolicMagickTopic {
    SigilCraft,
    SymbolicBinding,
    RunicMagic,
    TalismanicMagic,
    SymbolicInvocation,
    EnchantedSymbols,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolicMagickSystem {
    pub system_id: String,
    pub symbolic_magick_topic: SymbolicMagickTopic,
    pub symbolic_power: f64,
    pub sigil_magic: f64,
    pub binding_symbols: f64,
    pub magical_inscription: f64,
}

impl SymbolicMagickSystem {
    pub fn new(symbolic_magick_topic: SymbolicMagickTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            symbolic_magick_topic,
            symbolic_power: 0.0,
            sigil_magic: 0.0,
            binding_symbols: 0.0,
            magical_inscription: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.symbolic_magick_topic {
            SymbolicMagickTopic::SigilCraft => {
                self.symbolic_power = 0.95 + rand_simple() * 0.05;
                self.sigil_magic = 0.90 + rand_simple() * 0.10;
                self.binding_symbols = 0.85 + rand_simple() * 0.14;
            },
            SymbolicMagickTopic::SymbolicBinding => {
                self.magical_inscription = 0.95 + rand_simple() * 0.05;
                self.binding_symbols = 0.90 + rand_simple() * 0.10;
                self.sigil_magic = 0.85 + rand_simple() * 0.14;
            },
            SymbolicMagickTopic::RunicMagic => {
                self.sigil_magic = 0.95 + rand_simple() * 0.05;
                self.symbolic_power = 0.90 + rand_simple() * 0.10;
                self.magical_inscription = 0.85 + rand_simple() * 0.14;
            },
            SymbolicMagickTopic::TalismanicMagic => {
                self.binding_symbols = 0.95 + rand_simple() * 0.05;
                self.magical_inscription = 0.90 + rand_simple() * 0.10;
                self.symbolic_power = 0.85 + rand_simple() * 0.14;
            },
            SymbolicMagickTopic::SymbolicInvocation => {
                self.symbolic_power = 0.95 + rand_simple() * 0.05;
                self.sigil_magic = 0.90 + rand_simple() * 0.10;
                self.magical_inscription = 0.85 + rand_simple() * 0.14;
            },
            SymbolicMagickTopic::EnchantedSymbols => {
                self.magical_inscription = 0.95 + rand_simple() * 0.05;
                self.binding_symbols = 0.90 + rand_simple() * 0.10;
                self.sigil_magic = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.binding_symbols == 0.0 {
            self.binding_symbols = (self.symbolic_power + self.sigil_magic) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_sigil_craft() {
        let mut system = SymbolicMagickSystem::new(SymbolicMagickTopic::SigilCraft);
        system.analyze_system().unwrap();
        assert!(system.symbolic_power > 0.8);
    }
}