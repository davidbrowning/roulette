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


mod stock_categories {
    pub const MAG7: &str = "Magnificent Seven";
    pub const TECH: &str = "Technology";
    pub const SP500_HEAVY_A: &str = "S&P 500 Heavy A";
    pub const GROWTH_DOZEN_A: &str = "Growth Dozen A";
    pub const OIL_MAJOR: &str = "Oil & Gas Major";
    pub const ENERGY: &str = "Energy";
    pub const VALUE_FOCUS_B: &str = "Value Focus B";
    pub const VALUE_DOZEN_B: &str = "Value Dozen B";
    pub const BIG_FINANCE: &str = "Big Finance";
    pub const FINANCIALS: &str = "Financials";
    pub const BLUE_CHIP_DOZEN_C: &str = "Blue Chip Dozen C";
    // Add other categories as needed...
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
//#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//pub struct Pocket {
//    /// The number displayed on the pocket (0-36).
//    pub number: u8,
//    /// The color of the pocket.
//    pub color: Color,
//}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pocket {
    pub ticker: String,
    pub display_name: String,
    pub categories: Vec<String>,
    /// The number displayed on the pocket (0-36).
    pub number: u8,
    /// The color of the pocket.
    pub color: Color,
}

impl fmt::Display for Pocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.number, self.color, self.ticker, self.display_name)
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

        let red_numbers: [u8; 18] = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];
        let wheel_order: [u8; 37] = [
            0, 32, 15, 19, 4, 21, 2, 25, 17, 34, 6, 27, 13, 36, 11, 30, 8, 23,
            10, 5, 24, 16, 33, 1, 20, 14, 31, 9, 22, 18, 29, 7, 28, 12, 35, 3, 26,
        ];

        let pocket_defs = Self::get_pocket_definitions();

        // Ensure we have exactly 37 pockets
        if pocket_defs.len() != 37 {
            panic!("Expected 37 pocket definitions, got {}", pocket_defs.len());
        }

        for (i, &number) in wheel_order.iter().enumerate() {
            let mut pocket = pocket_defs[i].clone();
            pocket.number = number;
            pocket.color = if number == 0 {
                Color::Green
            } else if red_numbers.contains(&number) {
                Color::Red
            } else {
                Color::Black
            };

            pockets.push(pocket.clone());
            pocket_map.insert(number, pocket);
        }

        Wheel { pockets, pocket_map }
    }

    pub fn get_pocket_definitions() -> Vec<Pocket> {
        let ticker_data: HashMap<&str, (&str, Vec<&str>)> = HashMap::from([
            // Magnificent Seven
            ("AAPL", ("Apple Inc.", vec![
                stock_categories::MAG7, stock_categories::TECH,
                stock_categories::SP500_HEAVY_A,
                stock_categories::GROWTH_DOZEN_A,
                "AAPL"
            ])),
            ("MSFT", ("Microsoft Corp.", vec![
                stock_categories::MAG7, stock_categories::TECH,
                stock_categories::SP500_HEAVY_A,
                stock_categories::GROWTH_DOZEN_A,
                "MSFT"
            ])),
            ("GOOGL", ("Alphabet Inc.", vec![
                stock_categories::MAG7, stock_categories::TECH,
                stock_categories::SP500_HEAVY_A,
                stock_categories::GROWTH_DOZEN_A,
                "GOOGL"
            ])),
            ("AMZN", ("Amazon.com Inc.", vec![
                stock_categories::MAG7, stock_categories::TECH,
                stock_categories::SP500_HEAVY_A,
                stock_categories::GROWTH_DOZEN_A,
                "AMZN"
            ])),
            ("NVDA", ("NVIDIA Corp.", vec![
                stock_categories::MAG7, stock_categories::TECH,
                stock_categories::SP500_HEAVY_A,
                stock_categories::GROWTH_DOZEN_A,
                "NVDA"
            ])),
            ("META", ("Meta Platforms Inc.", vec![
                stock_categories::MAG7, stock_categories::TECH,
                stock_categories::SP500_HEAVY_A,
                stock_categories::GROWTH_DOZEN_A,
                "META"
            ])),
            ("TSLA", ("Tesla Inc.", vec![
                stock_categories::MAG7, stock_categories::TECH,
                stock_categories::SP500_HEAVY_A,
                stock_categories::GROWTH_DOZEN_A,
                "TSLA"
            ])),
    
            // Oil & Gas Majors
            ("XOM", ("Exxon Mobil Corp.", vec![
                stock_categories::OIL_MAJOR, stock_categories::ENERGY,
                stock_categories::VALUE_FOCUS_B,
                stock_categories::VALUE_DOZEN_B,
                "XOM"
            ])),
            ("CVX", ("Chevron Corp.", vec![
                stock_categories::OIL_MAJOR, stock_categories::ENERGY,
                stock_categories::VALUE_FOCUS_B,
                stock_categories::VALUE_DOZEN_B,
                "CVX"
            ])),
            ("COP", ("ConocoPhillips", vec![
                stock_categories::OIL_MAJOR, stock_categories::ENERGY,
                stock_categories::VALUE_FOCUS_B,
                stock_categories::VALUE_DOZEN_B,
                "COP"
            ])),
            ("2222.SR", ("Saudi Aramco", vec![
                stock_categories::OIL_MAJOR, stock_categories::ENERGY,
                stock_categories::VALUE_FOCUS_B,
                stock_categories::VALUE_DOZEN_B,
                "2222.SR"
            ])),
            ("PTR", ("PetroChina Co.", vec![
                stock_categories::OIL_MAJOR, stock_categories::ENERGY,
                stock_categories::VALUE_FOCUS_B,
                stock_categories::VALUE_DOZEN_B,
                "PTR"
            ])),
    
            // Big Finance (Banks & Payment Processors)
            ("JPM", ("JPMorgan Chase & Co.", vec![
                stock_categories::BIG_FINANCE, stock_categories::FINANCIALS,
                stock_categories::BLUE_CHIP_DOZEN_C,
                stock_categories::SP500_HEAVY_A,
                "JPM"
            ])),
            ("BRK-A", ("Berkshire Hathaway Inc.", vec![
                stock_categories::BIG_FINANCE, stock_categories::FINANCIALS,
                stock_categories::BLUE_CHIP_DOZEN_C,
                stock_categories::SP500_HEAVY_A,
                "BRK-A"
            ])),
            ("WFC", ("Wells Fargo & Co.", vec![
                stock_categories::BIG_FINANCE, stock_categories::FINANCIALS,
                stock_categories::BLUE_CHIP_DOZEN_C,
                stock_categories::SP500_HEAVY_A,
                "WFC"
            ])),
            ("V", ("Visa Inc.", vec![
                stock_categories::BIG_FINANCE, stock_categories::FINANCIALS,
                stock_categories::BLUE_CHIP_DOZEN_C,
                stock_categories::SP500_HEAVY_A,
                "V"
            ])),
            ("MA", ("Mastercard Inc.", vec![
                stock_categories::BIG_FINANCE, stock_categories::FINANCIALS,
                stock_categories::BLUE_CHIP_DOZEN_C,
                stock_categories::SP500_HEAVY_A,
                "MA"
            ])),
    
            // Pharma/Healthcare
            ("PFE", ("Pfizer Inc.", vec![
                "Pharma", "Healthcare", "Dividend Aristocrats", "PFE"
            ])),
            ("JNJ", ("Johnson & Johnson", vec![
                "Pharma", "Healthcare", "Dividend Aristocrats", "JNJ"
            ])),
            ("UNH", ("UnitedHealth Group", vec![
                "Pharma", "Healthcare", "Dividend Aristocrats", "UNH"
            ])),
    
            // Industrial
            ("GE", ("General Electric", vec![
                "Industrial", "Dividend Aristocrats", "GE"
            ])),
    
            // Legacy Tech
            ("IBM", ("IBM Corp.", vec![
                "Legacy Tech", "Dividend Aristocrats", "IBM"
            ])),
            ("INTC", ("Intel Corp.", vec![
                "Legacy Tech", "Dividend Aristocrats", "INTC"
            ])),
            ("CSCO", ("Cisco Systems", vec![
                "Legacy Tech", "Dividend Aristocrats", "CSCO"
            ])),
    
            // Telecom
            ("T", ("AT&T Inc.", vec![
                "Telecom", "Dividend Aristocrats", "T"
            ])),
            ("VZ", ("Verizon Communications", vec![
                "Telecom", "Dividend Aristocrats", "VZ"
            ])),
    
            // Retail/Consumer
            ("HD", ("Home Depot", vec![
                "Retail", "Consumer", "Dividend Aristocrats", "HD"
            ])),
            ("WMT", ("Walmart Inc.", vec![
                "Retail", "Consumer", "Dividend Aristocrats", "WMT"
            ])),
            ("KO", ("Coca-Cola Co.", vec![
                "Retail", "Consumer", "Dividend Aristocrats", "KO"
            ])),
            ("PEP", ("PepsiCo Inc.", vec![
                "Retail", "Consumer", "Dividend Aristocrats", "PEP"
            ])),
            ("PG", ("Procter & Gamble", vec![
                "Retail", "Consumer", "Dividend Aristocrats", "PG"
            ])),
            ("MCD", ("McDonald's Corp.", vec![
                "Retail", "Consumer", "Dividend Aristocrats", "MCD"
            ])),
            ("NKE", ("Nike Inc.", vec![
                "Retail", "Consumer", "Dividend Aristocrats", "NKE"
            ])),
            ("COST", ("Costco Wholesale", vec![
                "Retail", "Consumer", "Dividend Aristocrats", "COST"
            ])),
    
            // Other Automotive
            ("F", ("Ford Motor Co.", vec![
                "Automotive", "Dividend Aristocrats", "F"
            ])),
            ("GM", ("General Motors Co.", vec![
                "Automotive", "Dividend Aristocrats", "GM"
            ])),
            // Green Spaces 
            ("RCSN", ("Recession", vec![
                "Recession", "Recession", "RCSN"
            ])),
        ]);
    
        // Convert the hashmap entries into a Vec<Pocket>
        ticker_data.into_iter().map(|(ticker, (display_name, categories))| {
            Pocket {
                ticker: ticker.to_string(),
                display_name: display_name.to_string(),
                categories: categories.iter().map(|&s| s.to_string()).collect(),
                color: Color::Red,
                number: 0,
            }
        }).collect()
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
        self.pockets[winning_index].clone()
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
