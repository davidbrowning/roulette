// src/game/bets.rs

//! Defines bet types, placement logic, winning conditions, and payouts.

use super::wheel::{Color, Pocket};
use std::collections::HashSet;
use std::fmt;

/// Represents the different types of bets a player can make.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BetType {
    // Inside Bets
    StraightUp(u8),              // Bet on a single number (0-36)
    Split(u8, u8),             // Bet on two adjacent numbers
    Street(u8, u8, u8),        // Bet on three numbers in a horizontal line
    Corner(u8, u8, u8, u8),    // Bet on four numbers that meet at a corner
    SixLine(u8, u8, u8, u8, u8, u8), // Bet on two adjacent streets (six numbers)

    // Outside Bets
    Column(u8),                // Bet on one of the three vertical columns (1, 2, or 3)
    Dozen(u8),                 // Bet on one of the three dozens (1-12, 13-24, 25-36) (1, 2, or 3)
    Red,                       // Bet on all red numbers
    Black,                     // Bet on all black numbers
    Odd,                       // Bet on all odd numbers (excluding 0)
    Even,                      // Bet on all even numbers (excluding 0)
    Low,                       // Bet on numbers 1-18
    High,                      // Bet on numbers 19-36
}

impl fmt::Display for BetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BetType::StraightUp(n) => write!(f, "Straight Up ({})", n),
            BetType::Split(n1, n2) => write!(f, "Split ({}, {})", n1, n2),
            BetType::Street(n1, n2, n3) => write!(f, "Street ({}, {}, {})", n1, n2, n3),
            BetType::Corner(n1, n2, n3, n4) => write!(f, "Corner ({}, {}, {}, {})", n1, n2, n3, n4),
            BetType::SixLine(n1, n2, n3, n4, n5, n6) => write!(f, "Six Line ({}, {}, {}, {}, {}, {})", n1, n2, n3, n4, n5, n6),
            BetType::Column(c) => write!(f, "Column {}", c),
            BetType::Dozen(d) => write!(f, "Dozen {}", d),
            BetType::Red => write!(f, "Red"),
            BetType::Black => write!(f, "Black"),
            BetType::Odd => write!(f, "Odd"),
            BetType::Even => write!(f, "Even"),
            BetType::Low => write!(f, "Low (1-18)"),
            BetType::High => write!(f, "High (19-36)"),
        }
    }
}


/// Represents a single bet placed by the player.
#[derive(Debug, Clone)]
pub struct Bet {
    pub bet_type: BetType,
    pub amount: u32,
}

impl Bet {
    /// Creates a new bet.
    pub fn new(bet_type: BetType, amount: u32) -> Self {
        // Basic validation: ensure amount is positive
        if amount == 0 {
            // In a real app, this might return Result<Bet, Error>
            panic!("Bet amount must be positive.");
        }
        Bet { bet_type, amount }
    }

    /// Calculates the payout for this bet if it wins.
    /// The payout includes the original stake.
    /// Example: A $10 Straight Up bet wins $360 ($10 * 35 + $10 stake).
    pub fn calculate_payout(&self) -> u32 {
        self.amount * payout_multiplier(&self.bet_type) + self.amount
    }

    /// Checks if this bet wins based on the winning pocket.
    pub fn check_win(&self, winning_pocket: &Pocket) -> bool {
        let winning_number = winning_pocket.number;
        let winning_color = winning_pocket.color;

        // Zero never counts for even/odd, red/black, high/low, columns, dozens
        if winning_number == 0 {
            return match &self.bet_type {
                BetType::StraightUp(n) => *n == 0,
                // Add specific cases if bets involving 0 are allowed (e.g., 0/1/2 street)
                _ => false, // 0 loses for all standard outside bets
            };
        }

        // Check based on bet type
        match &self.bet_type {
            // --- Inside Bets ---
            BetType::StraightUp(n) => winning_number == *n,
            BetType::Split(n1, n2) => winning_number == *n1 || winning_number == *n2,
            BetType::Street(n1, n2, n3) => {
                winning_number == *n1 || winning_number == *n2 || winning_number == *n3
            }
            BetType::Corner(n1, n2, n3, n4) => {
                winning_number == *n1 || winning_number == *n2 || winning_number == *n3 || winning_number == *n4
            }
            BetType::SixLine(n1, n2, n3, n4, n5, n6) => {
                 winning_number == *n1 || winning_number == *n2 || winning_number == *n3 ||
                 winning_number == *n4 || winning_number == *n5 || winning_number == *n6
            }

            // --- Outside Bets ---
            BetType::Column(col) => {
                // Column 1: 1, 4, 7, ..., 34 (numbers where n % 3 == 1)
                // Column 2: 2, 5, 8, ..., 35 (numbers where n % 3 == 2)
                // Column 3: 3, 6, 9, ..., 36 (numbers where n % 3 == 0)
                match col {
                    1 => winning_number % 3 == 1,
                    2 => winning_number % 3 == 2,
                    3 => winning_number % 3 == 0,
                    _ => false, // Invalid column
                }
            }
            BetType::Dozen(doz) => {
                // Dozen 1: 1-12
                // Dozen 2: 13-24
                // Dozen 3: 25-36
                match doz {
                    1 => winning_number >= 1 && winning_number <= 12,
                    2 => winning_number >= 13 && winning_number <= 24,
                    3 => winning_number >= 25 && winning_number <= 36,
                    _ => false, // Invalid dozen
                }
            }
            BetType::Red => winning_color == Color::Red,
            BetType::Black => winning_color == Color::Black,
            BetType::Odd => winning_number % 2 != 0,
            BetType::Even => winning_number % 2 == 0,
            BetType::Low => winning_number >= 1 && winning_number <= 18,
            BetType::High => winning_number >= 19 && winning_number <= 36,
        }
    }
}


/// Returns the payout multiplier (odds) for a given bet type.
/// This is the amount won *per unit bet*, not including the stake return.
/// Example: Straight Up pays 35 to 1.
pub fn payout_multiplier(bet_type: &BetType) -> u32 {
    match bet_type {
        // Inside Bets
        BetType::StraightUp(_) => 35,
        BetType::Split(_, _) => 17,
        BetType::Street(_, _, _) => 11,
        BetType::Corner(_, _, _, _) => 8,
        BetType::SixLine(_, _, _, _, _, _) => 5,
        // Outside Bets
        BetType::Column(_) => 2,
        BetType::Dozen(_) => 2,
        BetType::Red => 1,
        BetType::Black => 1,
        BetType::Odd => 1,
        BetType::Even => 1,
        BetType::Low => 1,
        BetType::High => 1,
    }
}

// --- Helper functions for creating valid bets ---
// In a real application, these would perform more rigorous validation
// based on the layout of the roulette table.

/// Creates a Straight Up bet if the number is valid (0-36).
pub fn create_straight_up(number: u8, amount: u32) -> Option<Bet> {
    if number <= 36 {
        Some(Bet::new(BetType::StraightUp(number), amount))
    } else {
        println!("Invalid number for Straight Up bet (must be 0-36).");
        None
    }
}

/// Creates a Red bet.
pub fn create_red_bet(amount: u32) -> Bet {
     Bet::new(BetType::Red, amount)
}

/// Creates a Black bet.
pub fn create_black_bet(amount: u32) -> Bet {
     Bet::new(BetType::Black, amount)
}

/// Creates an Even bet.
pub fn create_even_bet(amount: u32) -> Bet {
     Bet::new(BetType::Even, amount)
}

/// Creates an Odd bet.
pub fn create_odd_bet(amount: u32) -> Bet {
     Bet::new(BetType::Odd, amount)
}

/// Creates a Low (1-18) bet.
pub fn create_low_bet(amount: u32) -> Bet {
     Bet::new(BetType::Low, amount)
}

/// Creates a High (19-36) bet.
pub fn create_high_bet(amount: u32) -> Bet {
     Bet::new(BetType::High, amount)
}

/// Creates a Column bet (1, 2, or 3).
pub fn create_column_bet(column: u8, amount: u32) -> Option<Bet> {
    if column >= 1 && column <= 3 {
        Some(Bet::new(BetType::Column(column), amount))
    } else {
         println!("Invalid column number (must be 1, 2, or 3).");
        None
    }
}

/// Creates a Dozen bet (1, 2, or 3).
pub fn create_dozen_bet(dozen: u8, amount: u32) -> Option<Bet> {
    if dozen >= 1 && dozen <= 3 {
        Some(Bet::new(BetType::Dozen(dozen), amount))
    } else {
        println!("Invalid dozen number (must be 1, 2, or 3).");
        None
    }
}

// TODO: Add creation functions and VALIDATION for Split, Street, Corner, SixLine.
// This requires knowledge of the table layout (which numbers are adjacent).
// For now, we will focus on the simpler bets in the main game loop.

