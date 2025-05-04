// src/main.rs

use std::io::{self, Write}; // Import Write trait for flush

// Declare the game module
mod game;

use game::bets::{
    self, Bet, BetType, // Import Bet and BetType
    create_black_bet, create_column_bet, create_dozen_bet, create_even_bet, create_high_bet,
    create_low_bet, create_odd_bet, create_red_bet, create_straight_up,
};
use game::Game;

// Helper function to get validated u32 input from the user
fn get_u32_input(prompt: &str) -> Option<u32> {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before input

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(num) => return Some(num),
            Err(_) => {
                // Allow empty input to signify cancellation or finishing bets
                if input.trim().is_empty() {
                    return None; // Indicate no number was entered (e.g., finish betting)
                }
                println!("Invalid input. Please enter a valid positive number.");
            }
        }
    }
}

// Helper function to get validated u8 input (used for numbers, columns, dozens)
fn get_u8_input(prompt: &str) -> Option<u8> {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before input

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<u8>() {
            Ok(num) => return Some(num),
            Err(_) => {
                 // Allow empty input to signify cancellation or finishing bets
                if input.trim().is_empty() {
                    return None; // Indicate no number was entered (e.g., finish betting)
                }
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}


// Function to handle the betting phase for a round
fn handle_betting(game: &mut Game) {
    println!("\n--- Place Your Bets ---");
    println!("Current Balance: ${}", game.get_player_balance());
    println!("Enter bet type number and follow prompts. Press Enter with no input to finish betting.");

    loop {
        println!("\nAvailable Bet Types:");
        println!(" 1) Straight Up (Number 0-36)");
        println!(" 2) Red");
        println!(" 3) Black");
        println!(" 4) Odd");
        println!(" 5) Even");
        println!(" 6) Low (1-18)");
        println!(" 7) High (19-36)");
        println!(" 8) Column (1, 2, or 3)");
        println!(" 9) Dozen (1, 2, or 3)");
        // TODO: Add options for Split, Street, Corner, SixLine when implemented
        println!("10) Clear All Bets for this Round");
        println!(" 0) Finish Betting for this Round");

        let choice = match get_u8_input("Enter bet type number (or 0 to spin): ") {
             Some(c) => c,
             None => 0, // Treat empty input as finishing bets
        };

        let mut bet_to_place: Option<Bet> = None;

        match choice {
            1 => { // Straight Up
                if let Some(number) = get_u8_input("Enter number (0-36): ") {
                    if let Some(amount) = get_u32_input("Enter amount to bet: $") {
                         if amount > 0 {
                            bet_to_place = bets::create_straight_up(number, amount);
                         } else { println!("Bet amount must be greater than 0."); }
                    }
                }
            }
            2 => { // Red
                if let Some(amount) = get_u32_input("Enter amount to bet on Red: $") {
                    if amount > 0 { bet_to_place = Some(create_red_bet(amount)); } else { println!("Bet amount must be greater than 0."); }
                }
            }
             3 => { // Black
                if let Some(amount) = get_u32_input("Enter amount to bet on Black: $") {
                    if amount > 0 { bet_to_place = Some(create_black_bet(amount)); } else { println!("Bet amount must be greater than 0."); }
                }
            }
             4 => { // Odd
                if let Some(amount) = get_u32_input("Enter amount to bet on Odd: $") {
                    if amount > 0 { bet_to_place = Some(create_odd_bet(amount)); } else { println!("Bet amount must be greater than 0."); }
                }
            }
             5 => { // Even
                if let Some(amount) = get_u32_input("Enter amount to bet on Even: $") {
                    if amount > 0 { bet_to_place = Some(create_even_bet(amount)); } else { println!("Bet amount must be greater than 0."); }
                }
            }
             6 => { // Low
                if let Some(amount) = get_u32_input("Enter amount to bet on Low (1-18): $") {
                    if amount > 0 { bet_to_place = Some(create_low_bet(amount)); } else { println!("Bet amount must be greater than 0."); }
                }
            }
             7 => { // High
                if let Some(amount) = get_u32_input("Enter amount to bet on High (19-36): $") {
                     if amount > 0 { bet_to_place = Some(create_high_bet(amount)); } else { println!("Bet amount must be greater than 0."); }
                }
            }
             8 => { // Column
                if let Some(col) = get_u8_input("Enter column number (1, 2, or 3): ") {
                    if let Some(amount) = get_u32_input("Enter amount to bet: $") {
                         if amount > 0 { bet_to_place = create_column_bet(col, amount); } else { println!("Bet amount must be greater than 0."); }
                    }
                }
            }
             9 => { // Dozen
                if let Some(doz) = get_u8_input("Enter dozen number (1=1-12, 2=13-24, 3=25-36): ") {
                    if let Some(amount) = get_u32_input("Enter amount to bet: $") {
                         if amount > 0 { bet_to_place = create_dozen_bet(doz, amount); } else { println!("Bet amount must be greater than 0."); }
                    }
                }
            }
            10 => { // Clear Bets
                game.clear_bets();
                // Continue loop to allow placing new bets or finishing
                continue;
            }
            0 => { // Finish Betting
                if game.get_current_bets().is_empty() {
                    println!("No bets placed. Place at least one bet before spinning.");
                    continue; // Go back to betting menu
                }
                println!("--- Betting Finished ---");
                break; // Exit betting loop
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }

        // If a valid bet was created, try to place it
        if let Some(bet) = bet_to_place {
            if !game.place_bet(bet) {
                // place_bet handles the error message (insufficient funds)
                // No need to do anything else here, loop continues
            } else {
                 // Successfully placed bet, show current bets
                 println!("Current Bets Placed:");
                 for placed_bet in game.get_current_bets() {
                     println!("  - {} for ${}", placed_bet.bet_type, placed_bet.amount);
                 }
                 println!("Total Balance: ${}", game.get_player_balance());
            }
        } else {
            // Bet creation might have failed (e.g., invalid number/column/dozen)
            // Message printed by creation function or amount was <= 0
        }

        // Check if player is out of money after placing a bet
        if game.get_player_balance() == 0 && !game.get_current_bets().is_empty() {
             println!("You've bet your remaining balance!");
             println!("--- Betting Finished ---");
             break; // Exit betting loop automatically if balance is 0 after betting
        }

    } // End of betting loop
}


fn main() {
    println!("=========================");
    println!(" Welcome to Rust Roulette!");
    println!("=========================");

    let starting_balance = match get_u32_input("Enter your starting balance: $") {
        Some(bal) if bal > 0 => bal,
        _ => {
            println!("Invalid starting balance. Defaulting to $1000.");
            1000 // Default balance
        }
    };

    let mut game = Game::new(starting_balance);

    // Main game loop
    loop {
        println!("\n------------------------------------");
        println!("Starting new round...");

        // Handle betting phase
        handle_betting(&mut game);

        // Spin the wheel and resolve bets
        game.spin_wheel_and_resolve();

        // Check if player is out of money
        if game.get_player_balance() == 0 {
            println!("\n------------------------------------");
            println!("Game Over! You are out of money.");
            println!("------------------------------------");
            break;
        }

        // Ask to play another round
        println!("\n------------------------------------");
        print!("Play another round? (y/n): ");
        io::stdout().flush().unwrap();
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");

        if play_again.trim().to_lowercase() != "y" {
            println!("Thanks for playing! Final Balance: ${}", game.get_player_balance());
            break; // Exit game loop
        }
    }
}

