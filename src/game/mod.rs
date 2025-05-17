// src/game/mod.rs

pub mod bets;
pub mod player;
pub mod wheel;

use bets::{Bet, BetType};
use player::Player;
use wheel::{Pocket, Wheel};

pub struct Game {
    pub wheel: Wheel, // Made public for access in main.rs
    player: Player,
    current_bets: Vec<Bet>,
}

impl Game {
    pub fn new(starting_balance: u32) -> Self {
        Game {
            player: Player::new(starting_balance),
            wheel: Wheel::new(),
            current_bets: Vec::new(),
        }
    }

    pub fn get_player_balance(&self) -> u32 {
        self.player.balance()
    }

    pub fn place_bet(&mut self, bet: Bet) -> bool {
        if self.player.place_bet(bet.amount) {
            println!("Placing bet: {} for ${}", bet.bet_type, bet.amount);
            self.current_bets.push(bet);
            true
        } else {
            false
        }
    }

    pub fn spin_wheel_and_resolve(&mut self) {
        if self.current_bets.is_empty() {
            println!("No bets placed for this round.");
            return;
        }

        println!("\nSpinning the Wall Street wheel...");
        let winning_pocket = self.wheel.spin();
        println!("------------------------------------");
        println!(
            ">>>>> The ball landed on: {} ({}, {}) <<<<<",
            winning_pocket.ticker, winning_pocket.display_name, winning_pocket.color
        );
        println!("Categories: {:?}", winning_pocket.categories);
        println!("------------------------------------");

        let mut total_winnings = 0;
        let mut total_bet_amount = 0;

        for bet in &self.current_bets {
            total_bet_amount += bet.amount;
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

        if total_winnings > 0 {
            self.player.add_winnings(total_winnings);
        } else {
            println!("No winning bets this round.");
        }

        println!("Round Summary:");
        println!("  Total Wagered: ${}", total_bet_amount);
        println!("  Total Won (incl. stakes): ${}", total_winnings);
        println!("  Net Gain/Loss: ${}", (total_winnings as i64) - (total_bet_amount as i64));
        println!("Current Balance: ${}", self.player.balance());

        self.current_bets.clear();
        println!("\nBets cleared. Ready for the next round.");
    }

    pub fn clear_bets(&mut self) {
        if self.current_bets.is_empty() {
            println!("No bets to clear.");
            return;
        }
        let mut total_refund = 0;
        for bet in self.current_bets.iter() {
            total_refund += bet.amount;
        }
        self.player.refund_bet(total_refund);
        self.current_bets.clear();
        println!("All bets cleared and refunded.");
    }

    pub fn get_current_bets(&self) -> &[Bet] {
        &self.current_bets
    }
}
