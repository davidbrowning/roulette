// src/game/bets.rs

use super::wheel::{Color, Pocket};
use crate::game::Wheel;
use std::collections::HashSet;
use std::fmt;

/// Represents the different types of bets a player can make.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BetType {
    // Inside Bets
    StraightUp(String),         // Bet on a single ticker (e.g., "AAPL")
    Split(String, String),     // Bet on two tickers
    // Note: Street, Corner, SixLine may need ticker-based equivalents or removal if less relevant

    // Outside Bets (Traditional)
    Red,                       // Bet on all red pockets
    Black,                     // Bet on all black pockets
    Odd,                       // Bet on odd-numbered pockets (excluding 0)
    Even,                      // Bet on even-numbered pockets (excluding 0)
    Low,                       // Bet on numbers 1-18
    High,                      // Bet on numbers 19-36

    // Outside Bets (Wall Street-themed)
    Category(String),          // Bet on a stock category (e.g., "Magnificent Seven")
    GrowthDozen,               // Equivalent to Dozen 1 (Growth-focused stocks)
    ValueDozen,                // Equivalent to Dozen 2 (Value-focused stocks)
    BlueChipDozen,             // Equivalent to Dozen 3 (Blue-chip stocks)
    Column(u8),                // Keep for compatibility, can represent sector groups later
}

impl fmt::Display for BetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BetType::StraightUp(ticker) => write!(f, "Straight Up ({})", ticker),
            BetType::Split(t1, t2) => write!(f, "Split ({}, {})", t1, t2),
            BetType::Red => write!(f, "Red"),
            BetType::Black => write!(f, "Black"),
            BetType::Odd => write!(f, "Odd"),
            BetType::Even => write!(f, "Even"),
            BetType::Low => write!(f, "Low (1-18)"),
            BetType::High => write!(f, "High (19-36)"),
            BetType::Category(cat) => write!(f, "Category ({})", cat),
            BetType::GrowthDozen => write!(f, "Growth Dozen"),
            BetType::ValueDozen => write!(f, "Value Dozen"),
            BetType::BlueChipDozen => write!(f, "Blue Chip Dozen"),
            BetType::Column(c) => write!(f, "Column {}", c),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bet {
    pub bet_type: BetType,
    pub amount: u32,
}

impl Bet {
    pub fn new(bet_type: BetType, amount: u32) -> Self {
        if amount == 0 {
            panic!("Bet amount must be positive.");
        }
        Bet { bet_type, amount }
    }

    pub fn calculate_payout(&self) -> u32 {
        self.amount * payout_multiplier(&self.bet_type) + self.amount
    }

    pub fn check_win(&self, winning_pocket: &Pocket) -> bool {
        let winning_number = winning_pocket.number;
        let winning_color = winning_pocket.color;
        let winning_ticker = &winning_pocket.ticker;
        let winning_categories = &winning_pocket.categories;

        // Zero (Recession/Surge) handling
        if winning_number == 0 {
            return match &self.bet_type {
                BetType::StraightUp(ticker) => ticker == winning_ticker,
                _ => false, // Zero loses for all standard outside bets
            };
        }

        match &self.bet_type {
            // Inside Bets
            BetType::StraightUp(ticker) => winning_ticker == ticker,
            BetType::Split(t1, t2) => winning_ticker == t1 || winning_ticker == t2,

            // Traditional Outside Bets
            BetType::Red => winning_color == Color::Red,
            BetType::Black => winning_color == Color::Black,
            BetType::Odd => winning_number % 2 != 0,
            BetType::Even => winning_number % 2 == 0,
            BetType::Low => winning_number >= 1 && winning_number <= 18,
            BetType::High => winning_number >= 19 && winning_number <= 36,
            BetType::Column(col) => match col {
                1 => winning_number % 3 == 1,
                2 => winning_number % 3 == 2,
                3 => winning_number % 3 == 0,
                _ => false,
            },

            // Wall Street-themed Bets
            BetType::Category(cat) => winning_categories.contains(cat),
            BetType::GrowthDozen => winning_categories.contains(&"Growth Dozen A".to_string()),
            BetType::ValueDozen => winning_categories.contains(&"Value Dozen B".to_string()),
            BetType::BlueChipDozen => winning_categories.contains(&"Blue Chip Dozen C".to_string()),
        }
    }
}

pub fn payout_multiplier(bet_type: &BetType) -> u32 {
    match bet_type {
        // Inside Bets
        BetType::StraightUp(_) => 35,
        BetType::Split(_, _) => 17,
        // Outside Bets
        BetType::Red => 1,
        BetType::Black => 1,
        BetType::Odd => 1,
        BetType::Even => 1,
        BetType::Low => 1,
        BetType::High => 1,
        BetType::Column(_) => 2,
        BetType::Category(_) => 2, // Adjust based on category size if needed
        BetType::GrowthDozen => 2,
        BetType::ValueDozen => 2,
        BetType::BlueChipDozen => 2,
    }
}

// Helper functions for creating bets
pub fn create_straight_up(ticker: &str, amount: u32, wheel: &Wheel) -> Option<Bet> {
    if wheel.get_all_pockets().iter().any(|p| p.ticker == ticker) {
        Some(Bet::new(BetType::StraightUp(ticker.to_string()), amount))
    } else {
        println!("Invalid ticker: {}. Please choose a valid stock ticker.", ticker);
        None
    }
}

pub fn create_category_bet(category: &str, amount: u32, wheel: &Wheel) -> Option<Bet> {
    if wheel.get_all_pockets().iter().any(|p| p.categories.contains(&category.to_string())) {
        Some(Bet::new(BetType::Category(category.to_string()), amount))
    } else {
        println!("Invalid category: {}. Please choose a valid category.", category);
        None
    }
}

pub fn create_red_bet(amount: u32) -> Bet {
    Bet::new(BetType::Red, amount)
}

pub fn create_black_bet(amount: u32) -> Bet {
    Bet::new(BetType::Black, amount)
}

pub fn create_even_bet(amount: u32) -> Bet {
    Bet::new(BetType::Even, amount)
}

pub fn create_odd_bet(amount: u32) -> Bet {
    Bet::new(BetType::Odd, amount)
}

pub fn create_low_bet(amount: u32) -> Bet {
    Bet::new(BetType::Low, amount)
}

pub fn create_high_bet(amount: u32) -> Bet {
    Bet::new(BetType::High, amount)
}

pub fn create_growth_dozen_bet(amount: u32) -> Bet {
    Bet::new(BetType::GrowthDozen, amount)
}

pub fn create_value_dozen_bet(amount: u32) -> Bet {
    Bet::new(BetType::ValueDozen, amount)
}

pub fn create_blue_chip_dozen_bet(amount: u32) -> Bet {
    Bet::new(BetType::BlueChipDozen, amount)
}

pub fn create_column_bet(column: u8, amount: u32) -> Option<Bet> {
    if column >= 1 && column <= 3 {
        Some(Bet::new(BetType::Column(column), amount))
    } else {
        println!("Invalid column number (must be 1, 2, or 3).");
        None
    }
}
