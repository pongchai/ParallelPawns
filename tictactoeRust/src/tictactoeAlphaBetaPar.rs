use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::io;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};


pub struct TicTacToeAlphaBetaPar {
    board: HashMap<i32, String>,
}

impl TicTacToeAlphaBetaPar {
    pub(crate) fn new() -> Self {
        let board = (1..=9).map(|i| (i, " ".to_string())).collect();
        Self {
            board,
        }
    }

    pub fn alphabetapar(
        &self,
        depth: i32,
        is_maximizer: bool,
        ai_xo: &str,
        player_xo: &str,
        alpha: Arc<Mutex<AtomicI32>>,
        beta: Arc<Mutex<AtomicI32>>,
        simulation_count: &AtomicI32,
    ) -> (i32, i32) {
        let checked_win = self.check_win().unwrap_or("".to_string());
        if checked_win == ai_xo {
            simulation_count.fetch_add(1, Ordering::Relaxed);
            return (1,1);
        } else if checked_win == player_xo {
            simulation_count.fetch_add(1, Ordering::Relaxed);
            return (-1,1)
        } else if self.check_draw() {
            simulation_count.fetch_add(1, Ordering::Relaxed);
            return (0, 1);
        }

        let moves: Vec<i32> = (1..=9).filter(|&move_| self.is_valid_move(move_)).collect();
        
        fn alpha_beta_helper(
            is_maximizer: bool,
            cloned_self: &TicTacToeAlphaBetaPar,
            move_: i32,
            ai_xo: &str,
            player_xo: &str,
            alpha: Arc<Mutex<AtomicI32>>,
            beta: Arc<Mutex<AtomicI32>>,
            simulation_count: &AtomicI32,
        ) -> (i32, i32, i32) {
            let cloned_self = cloned_self.make_move(move_, if is_maximizer { ai_xo } else { player_xo });
            let (score, simulations) =
                cloned_self.alphabetapar(1, !is_maximizer, ai_xo, player_xo, Arc::clone(&alpha), Arc::clone(&beta), simulation_count);
    
            (move_, score, simulations)
        }
    
        let (best_move, best_score, total_simulations_result) = moves
        .par_iter()
        .map(|&move_| alpha_beta_helper(is_maximizer, self, move_, ai_xo, player_xo, Arc::clone(&alpha), Arc::clone(&beta), simulation_count))
        .reduce(
            || (0, if is_maximizer { i32::MIN } else { i32::MAX }, 0),
            |left, right| {
                if is_maximizer {
                    if left.1 > right.1 { left } else { right }
                } else {
                    if left.1 < right.1 { left } else { right }
                }
            },
        );
    
        (best_score, total_simulations_result)
    }

    pub fn calculate_move_ab(&self, ai_xo: &str, player_xo: &str) -> i32 {
        let moves: Vec<i32> = (1..=9).filter(|&move_| self.is_valid_move(move_)).collect();
        let simulation_count = AtomicI32::new(0);
        let alpha = Arc::new(Mutex::new(AtomicI32::new(i32::MIN)));
        let beta = Arc::new(Mutex::new(AtomicI32::new(i32::MAX)));

        let results = moves
            .par_iter()
            .map(|&move_| {
                let cloned_self = self.make_move(move_, ai_xo);
                let (eval, simulations) = cloned_self.alphabetapar(
                    0,
                    false,
                    ai_xo,
                    player_xo,
                    Arc::clone(&alpha),
                    Arc::clone(&beta),
                    &simulation_count,
                );
                (move_, eval, simulations)
            })
            .collect::<Vec<(i32, i32, i32)>>();

        let best_move = results
            .iter()
            .max_by_key(|&(_, eval, _)| *eval)
            .map(|&(move_, _, _)| move_)
            .unwrap_or(0);

        println!("End game simulations went thru: {}", simulation_count.load(Ordering::Relaxed));

        best_move
    }

    fn check_win(&self) -> Option<String> {
        let winning_combinations = vec![
            (1, 2, 3),
            (4, 5, 6),
            (7, 8, 9),
            (1, 4, 7),
            (2, 5, 8),
            (3, 6, 9),
            (1, 5, 9),
            (3, 5, 7),
        ];

        winning_combinations
            .iter()
            .find(|&&(a, b, c)| {
                self.board[&a] == self.board[&b]
                    && self.board[&b] == self.board[&c]
                    && self.board[&a] != " "
            })
            .map(|&combination| self.board[&combination.0].clone())
    }
    pub fn check_draw(&self) -> bool {
        self.board.values().all(|v| v != " ")
    }

    pub fn make_move(&self, move_: i32, xo: &str) -> Self {
        let mut new_board = self.board.clone();
        new_board.insert(move_, xo.to_string());
        Self {
            board: new_board,
        }
    }
    

    pub fn is_valid_move(&self, move_: i32) -> bool {
        self.board[&move_] == " "
    }

    pub fn current_board(&self) {
        println!(
            "\n{} | {} | {}",
            self.board[&1], self.board[&2], self.board[&3]
        );
        println!("--+---+--");
        println!(
            "{} | {} | {}",
            self.board[&4], self.board[&5], self.board[&6]
        );
        println!("--+---+--");
        println!(
            "{} | {} | {}\n",
            self.board[&7], self.board[&8], self.board[&9]
        );
    }

    pub fn coordination_board() {
        println!("\n1 | 2 | 3");
        println!("--+---+--");
        println!("4 | 5 | 6");
        println!("--+---+--");
        println!("7 | 8 | 9\n");
    }

    pub fn help_command() {
        println!("\nAVAILABLE COMMANDS:");
        println!("- 'help'");
        println!("- 'quit'");
        println!("- select between 1-9 corresponding to board spots below");
        Self::coordination_board();
        println!();
    }

    pub fn play_game_alphabeta_par(&mut self) {
        let human_symbol = loop {
            println!("Do you want to be 'X' or 'O'?");
            let mut human_symbol = String::new();
            io::stdin()
                .read_line(&mut human_symbol)
                .expect("Failed to read input.");
            human_symbol = human_symbol.trim().to_uppercase();
        
            if human_symbol == "X" || human_symbol == "O" {
                break human_symbol;
            } else if human_symbol.to_lowercase() == "quit" {
                println!("Exiting the game.");
                std::process::exit(0);
            } else if human_symbol.to_lowercase() == "help" {
                Self::help_command();
            } else {
                println!("Invalid input. Please enter 'X' or 'O'.");
            }
        };
        

        let ai_symbol = if human_symbol == "X" { "O" } else { "X" };

        println!("You are '{}'", human_symbol);
        if human_symbol == "X" {
            println!("You are '{}'. Your move first. Good luck! ;)", human_symbol);
        } else {
            println!("AI is 'X'. AI moves first. Good luck! ;)");
        }

        let mut is_human_turn = human_symbol == "X";


        loop {
            self.current_board();
            if is_human_turn {
                let mut move_ = String::new();
                io::stdin()
                    .read_line(&mut move_)
                    .expect("Failed to read input.");
                let move_: i32 = move_
                    .trim()
                    .parse()
                    .expect("Invalid input. Please enter a number.");

                if self.is_valid_move(move_) {
                    *self = self.make_move(move_, &human_symbol);
                } else {
                    println!("Invalid move. Please try again.");
                    continue;
                }
            } else {
                let best_move = self.calculate_move_ab(&ai_symbol, &human_symbol);
                println!("AI move: {}", best_move);
                *self = self.make_move(best_move, &ai_symbol);
            }

            // Print the simulation count

            if let Some(winner) = self.check_win() {
                self.current_board();
                if winner == human_symbol {
                    println!("Congratulations! You won!");
                } else {
                    println!("AI won!");
                }
                break;
            } else if self.check_draw() {
                self.current_board();
                println!("It's a draw!");
                break;
            }

            is_human_turn = !is_human_turn;
        }

        println!("Thank you for playing.");
    }
}