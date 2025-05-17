// src/main.rs

use std::io::{self, Write};
mod game;

use game::bets::{
    Bet, BetType,
    create_black_bet, create_blue_chip_dozen_bet, create_category_bet, create_column_bet,
    create_even_bet, create_growth_dozen_bet, create_high_bet, create_low_bet, create_odd_bet,
    create_red_bet, create_straight_up, create_value_dozen_bet,
};
use game::Game;

fn get_u32_input(prompt: &str) -> Option<u32> {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<u32>() {
            Ok(num) => return Some(num),
            Err(_) => {
                if input.trim().is_empty() {
                    return None;
                }
                println!("Invalid input. Please enter a valid positive number.");
            }
        }
    }
}

fn get_string_input(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let trimmed = input.trim().to_uppercase();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn display_wheel(game: &Game) {
    println!("\n=== Wall Street Roulette Wheel ===");
    let pockets = game.wheel.get_all_pockets();
    for pocket in pockets {
        println!(
            "Ticker: {:<6} | Name: {:<20} | Categories: {:?} | Color: {}",
            pocket.ticker, pocket.display_name, pocket.categories, pocket.color
        );
    }
    println!("=================================");
}

fn handle_betting(game: &mut Game) {
    println!("\n--- Place Your Wall Street Bets ---");
    println!("Current Balance: ${}", game.get_player_balance());
    println!("Enter bet type number and follow prompts. Press Enter with no input to finish betting.");
    display_wheel(game); // Show the wheel's stocks and categories

    loop {
        println!("\nAvailable Bet Types:");
        println!(" 1) Straight Up (Single Stock Ticker, e.g., AAPL)");
        println!(" 2) Category (e.g., Magnificent Seven, Technology)");
        println!(" 3) Growth Dozen (Growth Stocks)");
        println!(" 4) Value Dozen (Value Stocks)");
        println!(" 5) Blue Chip Dozen (Blue Chip Stocks)");
        println!(" 6) Red");
        println!(" 7) Black");
        println!(" 8) Odd");
        println!(" 9) Even");
        println!("10) Low (1-18)");
        println!("11) High (19-36)");
        println!("12) Column (1, 2, or 3)");
        println!("13) Clear All Bets for this Round");
        println!(" 0) Finish Betting for this Round");

        let choice = match get_u32_input("Enter bet type number (or 0 to spin): ") {
            Some(c) => c,
            None => 0,
        };

        let mut bet_to_place: Option<Bet> = None;

        match choice {
            1 => {
                if let Some(ticker) = get_string_input("Enter stock ticker (e.g., AAPL): ") {
                    if let Some(amount) = get_u32_input("Enter amount to bet: $") {
                        if amount > 0 {
                            bet_to_place = create_straight_up(&ticker, amount, &game.wheel);
                        } else {
                            println!("Bet amount must be greater than 0.");
                        }
                    }
                }
            }
            2 => {
                if let Some(category) = get_string_input("Enter category (e.g., Magnificent Seven): ") {
                    if let Some(amount) = get_u32_input("Enter amount to bet: $") {
                        if amount > 0 {
                            bet_to_place = create_category_bet(&category, amount, &game.wheel);
                        } else {
                            println!("Bet amount must be greater than 0.");
                        }
                    }
                }
            }
            3 => {
                if let Some(amount) = get_u32_input("Enter amount to bet on Growth Dozen: $") {
                    if amount > 0 {
                        bet_to_place = Some(create_growth_dozen_bet(amount));
                    } else {
                        println!("Bet amount must be greater than 0.");
                    }
                }
            }
            4 => {
                if let Some(amount) = get_u32_input("Enter amount to bet on Value Dozen: $") {
                    if amount > 0 {
                        bet_to_place = Some(create_value_dozen_bet(amount));
                    } else {
                        println!("Bet amount must be greater than 0.");
                    }
                }
            }
            5 => {
                if let Some(amount) = get_u32_input("Enter amount to bet on Blue Chip Dozen: $") {
                    if amount > 0 {
                        bet_to_place = Some(create_blue_chip_dozen_bet(amount));
                    } else {
                        println!("Bet amount must be greater than 0.");
                    }
                }
            }
            6 => {
                if let Some(amount) = get_u32_input("Enter amount to bet on Red: $") {
                    if amount > 0 {
                        bet_to_place = Some(create_red_bet(amount));
                    } else {
                        println!("Bet amount must be greater than 0.");
                    }
                }
            }
            7 => {
                if let Some(amount) = get_u32_input("Enter amount to bet on Black: $") {
                    if amount > 0 {
                        bet_to_place = Some(create_black_bet(amount));
                    } else {
                        println!("Bet amount must be greater than 0.");
                    }
                }
            }
            8 => {
                if let Some(amount) = get_u32_input("Enter amount to bet on Odd: $") {
                    if amount > 0 {
                        bet_to_place = Some(create_odd_bet(amount));
                    } else {
                        println!("Bet amount must be greater than 0.");
                    }
                }
            }
            9 => {
                if let Some(amount) = get_u32_input("Enter amount to bet on Even: $") {
                    if amount > 0 {
                        bet_to_place = Some(create_even_bet(amount));
                    } else {
                        println!("Bet amount must be greater than 0.");
                    }
                }
            }
            10 => {
                if let Some(amount) = get_u32_input("Enter amount to bet on Low (1-18): $") {
                    if amount > 0 {
                        bet_to_place = Some(create_low_bet(amount));
                    } else {
                        println!("Bet amount must be greater than 0.");
                    }
                }
            }
            11 => {
                if let Some(amount) = get_u32_input("Enter amount to bet on High (19-36): $") {
                    if amount > 0 {
                        bet_to_place = Some(create_high_bet(amount));
                    } else {
                        println!("Bet amount must be greater than 0.");
                    }
                }
            }
            12 => {
                if let Some(col) = get_u32_input("Enter column number (1, 2, or 3): ").map(|x| x as u8) {
                    if let Some(amount) = get_u32_input("Enter amount to bet: $") {
                        if amount > 0 {
                            bet_to_place = create_column_bet(col, amount);
                        } else {
                            println!("Bet amount must be greater than 0.");
                        }
                    }
                }
            }
            13 => {
                game.clear_bets();
                continue;
            }
            0 => {
                if game.get_current_bets().is_empty() {
                    println!("No bets placed. Place at least one bet before spinning.");
                    continue;
                }
                println!("--- Betting Finished ---");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }

        if let Some(bet) = bet_to_place {
            if game.place_bet(bet) {
                println!("Current Bets Placed:");
                for placed_bet in game.get_current_bets() {
                    println!("  - {} for ${}", placed_bet.bet_type, placed_bet.amount);
                }
                println!("Total Balance: ${}", game.get_player_balance());
            }
        }

        if game.get_player_balance() == 0 && !game.get_current_bets().is_empty() {
            println!("You've bet your remaining balance!");
            println!("--- Betting Finished ---");
            break;
        }
    }
}

fn main() {
    println!("=================================");
    println!(" Welcome to Wall Street Roulette!");
    println!("=================================");
    println!("Bet on stocks and sectors! Spin the wheel to see which stock wins!");

    let starting_balance = match get_u32_input("Enter your starting balance: $") {
        Some(bal) if bal > 0 => bal,
        _ => {
            println!("Invalid starting balance. Defaulting to $1000.");
            1000
        }
    };

    let mut game = Game::new(starting_balance);

    loop {
        println!("\n------------------------------------");
        println!("Starting new round...");

        handle_betting(&mut game);

        game.spin_wheel_and_resolve();

        if game.get_player_balance() == 0 {
            println!("\n------------------------------------");
            println!("Game Over! You are out of money.");
            println!("------------------------------------");
            break;
        }

        print!("Play another round? (y/n): ");
        io::stdout().flush().unwrap();
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");

        if play_again.trim().to_lowercase() != "y" {
            println!("Thanks for playing! Final Balance: ${}", game.get_player_balance());
            break;
        }
    }
}