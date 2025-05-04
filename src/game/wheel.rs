// src/game/wheel.rs

//! Defines the roulette wheel structure, pockets, colors, and spinning logic.

use rand::Rng;
use std::collections::HashMap;
use std::fmt;

/// Represents the possible colors on a roulette wheel pocket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Black,
    Green, // For zero
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Black => write!(f, "Black"),
            Color::Green => write!(f, "Green"),
        }
    }
}

/// Represents a single pocket on the roulette wheel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pocket {
    /// The number displayed on the pocket (0-36).
    pub number: u8,
    /// The color of the pocket.
    pub color: Color,
}

impl fmt::Display for Pocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.number, self.color)
    }
}

/// Represents the European roulette wheel.
pub struct Wheel {
    pockets: Vec<Pocket>,
    pocket_map: HashMap<u8, Pocket>, // For quick lookup by number
}

impl Wheel {
    /// Creates a new European roulette wheel (0-36).
    pub fn new() -> Self {
        let mut pockets = Vec::with_capacity(37);
        let mut pocket_map = HashMap::with_capacity(37);

        // Define colors for numbers (standard European layout)
        let red_numbers: [u8; 18] = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];

        // Add pocket 0 (Green)
        let zero_pocket = Pocket { number: 0, color: Color::Green };
        pockets.push(zero_pocket);
        pocket_map.insert(0, zero_pocket);

        // Add pockets 1-36
        for number in 1..=36 {
            let color = if red_numbers.contains(&number) {
                Color::Red
            } else {
                Color::Black
            };
            let pocket = Pocket { number, color };
            pockets.push(pocket);
            pocket_map.insert(number, pocket);
        }

        Wheel { pockets, pocket_map }
    }

    /// Gets a pocket by its number.
    pub fn get_pocket(&self, number: u8) -> Option<&Pocket> {
        self.pocket_map.get(&number)
    }

    /// Simulates spinning the wheel and returns the winning pocket.
    pub fn spin(&self) -> Pocket {
        let mut rng = rand::thread_rng();
        // Generate a random index from 0 to 36 (inclusive)
        let winning_index = rng.gen_range(0..self.pockets.len());
        // Return a copy of the winning pocket
        self.pockets[winning_index]
    }

    /// Returns a slice of all pockets on the wheel.
    pub fn get_all_pockets(&self) -> &[Pocket] {
        &self.pockets
    }
}

// Default implementation for convenience
impl Default for Wheel {
    fn default() -> Self {
        Self::new()
    }
}
