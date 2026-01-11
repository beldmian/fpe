use clap::{Parser, Subcommand};
use fpe::cli::{output, validation};
use fpe::models::{GameState, Position};
use fpe::solver;
use std::str::FromStr;

/// Poker GTO Strategy Engine
///
/// Calculate Nash equilibrium strategies for poker decision points.
#[derive(Parser)]
#[command(name = "fpe")]
#[command(about = "Poker GTO Strategy Engine", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate GTO strategy for a decision point
    Analyze {
        /// Hero's hole cards (e.g., "AhKd")
        #[arg(long)]
        hero: String,

        /// Community cards (e.g., "Ts9s2h")
        #[arg(long, default_value = "")]
        board: String,

        /// Opponent's range in Equilab notation
        #[arg(long)]
        villain_range: String,

        /// Pot size in big blinds
        #[arg(long)]
        pot: f64,

        /// Effective stack size in big blinds
        #[arg(long)]
        stack: f64,

        /// Amount to call in big blinds
        #[arg(long, default_value = "0")]
        to_call: f64,

        /// Hero position: IP or OOP
        #[arg(long, default_value = "IP")]
        position: String,

        /// Solver iterations
        #[arg(long, default_value = "10000")]
        iterations: u32,

        /// Output as JSON
        #[arg(long, default_value = "false")]
        json: bool,

        /// Show solver progress
        #[arg(long, default_value = "false")]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            hero,
            board,
            villain_range,
            pot,
            stack,
            to_call,
            position,
            iterations,
            json,
            verbose: _, // Not used yet
        } => {
            // Parse inputs
            let hero_hand = match validation::validate_hand(&hero) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("Error parsing hero hand: {}", e);
                    std::process::exit(1);
                }
            };

            // Parse board
            let mut board_cards = Vec::new();
            if !board.is_empty() {
                let chars: Vec<char> = board.chars().collect();
                if !chars.len().is_multiple_of(2) {
                    eprintln!("Error: Board string length must be even");
                    std::process::exit(1);
                }
                for chunk in chars.chunks(2) {
                    let s: String = chunk.iter().collect();
                    match validation::validate_card(&s) {
                        Ok(c) => board_cards.push(c),
                        Err(e) => {
                            eprintln!("Error parsing board card '{}': {}", s, e);
                            std::process::exit(1);
                        }
                    }
                }
            }

            // Validate duplicates
            if let Err(e) = validation::check_duplicates(&hero_hand, &board_cards) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }

            let position_enum = match Position::from_str(&position) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error parsing position: {}", e);
                    std::process::exit(1);
                }
            };

            // Parse Range
            let mut v_range = match validation::validate_range(&villain_range) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error parsing villain range: {}", e);
                    std::process::exit(1);
                }
            };

            // Remove blockers from range
            let mut blockers = Vec::new();
            blockers.extend(hero_hand.cards);
            blockers.extend(&board_cards);
            v_range.remove_blockers(&blockers);

            // Create GameState
            let game_state = match GameState::new(
                hero_hand.clone(),
                board_cards.clone(),
                pot,
                stack,
                to_call,
                position_enum,
                v_range,
            ) {
                Ok(gs) => gs,
                Err(e) => {
                    eprintln!("Error creating game state: {}", e);
                    std::process::exit(1);
                }
            };

            // Solve
            match solver::solve(game_state, iterations) {
                Ok(strategy) => {
                    if json {
                        println!("{}", serde_json::to_string_pretty(&strategy).unwrap());
                    } else {
                        // Summary
                        println!("Input Summary:");
                        println!("  Hero: {}", hero_hand.notation());
                        if !board_cards.is_empty() {
                            let board_str: Vec<String> =
                                board_cards.iter().map(|c| c.to_string()).collect();
                            println!("  Board: {}", board_str.join(" "));
                        } else {
                            println!("  Board: (none)");
                        }
                        println!(
                            "  Pot: {:.1} BB, Stack: {:.1} BB, To Call: {:.1} BB",
                            pot, stack, to_call
                        );

                        println!();
                        println!(
                            "Strategy computed in {} iterations (convergence: {})",
                            strategy.iterations, strategy.convergence
                        );
                        println!();

                        // Table output
                        println!("{}", output::format_strategy_table(&strategy));
                    }
                }
                Err(e) => {
                    eprintln!("Solver error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
