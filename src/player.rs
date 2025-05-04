// src/game/player.rs

//! Defines the player structure and associated methods.

/// Represents a player in the game.
#[derive(Debug)]
pub struct Player {
    /// The current balance of the player.
    balance: u32,
}

impl Player {
    /// Creates a new player with a starting balance.
    ///
    /// # Arguments
    ///
    /// * `starting_balance` - The initial amount of money the player has.
    pub fn new(starting_balance: u32) -> Self {
        Player { balance: starting_balance }
    }

    /// Returns the current balance of the player.
    pub fn balance(&self) -> u32 {
        self.balance
    }

    /// Adds winnings to the player's balance.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to add.
    pub fn add_winnings(&mut self, amount: u32) {
        self.balance += amount;
        println!("You won ${}! New balance: ${}", amount, self.balance);
    }

    /// Deducts a bet amount from the player's balance.
    /// Returns true if the player has enough balance, false otherwise.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to deduct.
    pub fn place_bet(&mut self, amount: u32) -> bool {
        if amount > self.balance {
            println!("Insufficient balance. You have ${}, but tried to bet ${}", self.balance, amount);
            false
        } else {
            self.balance -= amount;
            println!("Bet ${} placed. Remaining balance: ${}", amount, self.balance);
            true
        }
    }

     /// Adds back the bet amount if the bet was invalid or cancelled.
     ///
     /// # Arguments
     ///
     /// * `amount` - The amount to refund.
     pub fn refund_bet(&mut self, amount: u32) {
         self.balance += amount;
         println!("Bet ${} refunded. Balance: ${}", amount, self.balance);
     }
}
