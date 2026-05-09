//! # SBMUMC Module 1049: Market Design
//!
//! Mechanisms and frameworks for designing effective markets.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketMechanismType {
    Auction,
    DoubleAuction,
    Matching,
    Brokerage,
    Exchange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDesign {
    pub design_id: String,
    pub market_name: String,
    pub mechanism_type: MarketMechanismType,
    pub participant_count: usize,
    pub clearing_efficiency: f64,
    pub price_discovery_speed: f64,
    pub welfare_generated: f64,
}

impl MarketDesign {
    pub fn new(name: String, mechanism: MarketMechanismType) -> Self {
        Self {
            design_id: crate::core::uuid_simple(),
            market_name: name,
            mechanism_type: mechanism,
            participant_count: 0,
            clearing_efficiency: 0.0,
            price_discovery_speed: 0.0,
            welfare_generated: 0.0,
        }
    }

    pub fn evaluate_design(&mut self, participants: usize) -> Result<()> {
        self.participant_count = participants;

        match self.mechanism_type {
            MarketMechanismType::Auction => {
                self.clearing_efficiency = 0.8 + rand_simple() * 0.2;
                self.price_discovery_speed = 0.6 + rand_simple() * 0.3;
            },
            MarketMechanismType::DoubleAuction => {
                self.clearing_efficiency = 0.9 + rand_simple() * 0.1;
                self.price_discovery_speed = 0.8 + rand_simple() * 0.15;
            },
            MarketMechanismType::Matching => {
                self.clearing_efficiency = 0.85 + rand_simple() * 0.15;
                self.price_discovery_speed = 0.5 + rand_simple() * 0.4;
            },
            _ => {
                self.clearing_efficiency = 0.7 + rand_simple() * 0.25;
                self.price_discovery_speed = 0.6 + rand_simple() * 0.3;
            }
        }

        self.welfare_generated = self.clearing_efficiency * (self.participant_count as f64 / 100.0);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionDesign {
    pub auction_id: String,
    pub auction_type: String,
    pub reserve_price: f64,
    pub expected_revenue: f64,
    pub bidder_participation_rate: f64,
    pub efficiency_score: f64,
}

impl AuctionDesign {
    pub fn new(auction_type: String, reserve: f64) -> Self {
        Self {
            auction_id: crate::core::uuid_simple(),
            auction_type,
            reserve_price: reserve,
            expected_revenue: 0.0,
            bidder_participation_rate: 0.0,
            efficiency_score: 0.0,
        }
    }

    pub fn analyze_auction(&mut self) -> Result<()> {
        match self.auction_type.as_str() {
            "First_Price_Sealed_Bid" => {
                self.expected_revenue = self.reserve_price * 1.2 + rand_simple() * self.reserve_price * 0.8;
                self.bidder_participation_rate = 0.6 + rand_simple() * 0.3;
            },
            "Vickrey_Second_Price" => {
                self.expected_revenue = self.reserve_price * 1.5 + rand_simple() * self.reserve_price * 0.5;
                self.bidder_participation_rate = 0.8 + rand_simple() * 0.15;
            },
            "English" => {
                self.expected_revenue = self.reserve_price * 1.8 + rand_simple() * self.reserve_price * 0.7;
                self.bidder_participation_rate = 0.75 + rand_simple() * 0.2;
            },
            _ => {
                self.expected_revenue = self.reserve_price * 1.3 + rand_simple() * self.reserve_price * 0.6;
                self.bidder_participation_rate = 0.65 + rand_simple() * 0.25;
            }
        }

        self.efficiency_score = self.bidder_participation_rate * 0.9 + rand_simple() * 0.1;
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

pub fn optimize_market_mechanism(market_type: &str) -> Result<MarketMechanismType> {
    let mechanism = match market_type {
        "Labor" => MarketMechanismType::Matching,
        "Goods" => MarketMechanismType::DoubleAuction,
        "Services" => MarketMechanismType::Auction,
        _ => MarketMechanismType::Exchange,
    };
    Ok(mechanism)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_auction_market() {
        let mut market = MarketDesign::new(
            "Commodity_Exchange".to_string(),
            MarketMechanismType::DoubleAuction,
        );
        market.evaluate_design(1000).unwrap();
        assert!(market.clearing_efficiency > 0.8);
    }

    #[test]
    fn test_vickrey_auction() {
        let mut auction = AuctionDesign::new("Vickrey_Second_Price".to_string(), 1000.0);
        auction.analyze_auction().unwrap();
        assert!(auction.expected_revenue > auction.reserve_price);
    }
}