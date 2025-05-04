// src/game/mod.rs

//! The core game logic module for Roulette.

// Make components public within the crate or publicly as needed
pub mod bets;
pub mod player;
pub mod wheel;

use bets::{Bet, BetType};
use player::Player;
use wheel::{Pocket, Wheel};

/// Represents the state of the roulette game.
pub struct Game {
    player: Player,
    wheel: Wheel,
    current_bets: Vec<Bet>,
}

impl Game {
    /// Creates a new game instance with a player having the specified starting balance.
    pub fn new(starting_balance: u32) -> Self {
        Game {
            player: Player::new(starting_balance),
            wheel: Wheel::new(), // Create a standard European wheel
            current_bets: Vec::new(),
        }
    }

    /// Gets the player's current balance.
    pub fn get_player_balance(&self) -> u32 {
        self.player.balance()
    }

    /// Adds a bet to the current round.
    /// Checks if the player has sufficient balance before adding.
    ///
    /// # Arguments
    ///
    /// * `bet` - The bet to be placed.
    ///
    /// # Returns
    ///
    /// * `true` if the bet was successfully placed.
    /// * `false` if the player had insufficient funds.
    pub fn place_bet(&mut self, bet: Bet) -> bool {
        if self.player.place_bet(bet.amount) {
            println!("Placing bet: {} for ${}", bet.bet_type, bet.amount);
            self.current_bets.push(bet);
            true
        } else {
            // Balance check failed in player.place_bet, message already printed
            false
        }
    }

    /// Spins the wheel, resolves all placed bets, and updates the player's balance.
    pub fn spin_wheel_and_resolve(&mut self) {
        if self.current_bets.is_empty() {
            println!("No bets placed for this round.");
            return;
        }

        println!("\nSpinning the wheel...");
        // Simulate delay
        // std::thread::sleep(std::time::Duration::from_secs(2));

        let winning_pocket = self.wheel.spin();
        println!("------------------------------------");
        println!(">>>>> The ball landed on: {} <<<<<", winning_pocket);
        println!("------------------------------------");

        let mut total_winnings = 0;
        let mut total_bet_amount = 0;

        // Iterate through the placed bets and check for wins
        for bet in &self.current_bets {
             total_bet_amount += bet.amount; // Track total amount wagered this round
            if bet.check_win(&winning_pocket) {
                let payout = bet.calculate_payout();
                println!(
                    "  WIN! Bet on {} won! Payout: ${} (includes ${} stake)",
                    bet.bet_type, payout, bet.amount
                );
                total_winnings += payout;
            } else {
                 println!("  LOSE! Bet on {} for ${} lost.", bet.bet_type, bet.amount);
            }
        }

        // Add total winnings to player balance
        if total_winnings > 0 {
            // Note: calculate_payout includes the stake, so we just add the total_winnings
            self.player.add_winnings(total_winnings);
        } else {
             println!("No winning bets this round.");
             // The bet amounts were already deducted when placed.
        }

        println!("Round Summary:");
        println!("  Total Wagered: ${}", total_bet_amount);
        println!("  Total Won (incl. stakes): ${}", total_winnings);
        println!("  Net Gain/Loss: ${}", (total_winnings as i64) - (total_bet_amount as i64)); // Calculate net change
        println!("Current Balance: ${}", self.player.balance());


        // Clear bets for the next round
        self.current_bets.clear();
        println!("\nBets cleared. Ready for the next round.");
    }

     /// Clears all currently placed bets and refunds the player.
     pub fn clear_bets(&mut self) {
         if self.current_bets.is_empty() {
             println!("No bets to clear.");
             return;
         }
         let mut total_refund = 0;
         for bet in self.current_bets.iter() {
             total_refund += bet.amount;
         }
         self.player.refund_bet(total_refund); // Refund the total amount
         self.current_bets.clear();
         println!("All bets cleared and refunded.");
     }

     /// Returns a slice of the currently placed bets.
     pub fn get_current_bets(&self) -> &[Bet] {
         &self.current_bets
     }
}
