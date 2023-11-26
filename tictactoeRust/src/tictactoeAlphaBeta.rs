use std::collections::HashMap;
use std::i32;
use std::io;

pub struct TicTacToeAlphaBeta {
    board: HashMap<i32, String>,
    simulation: i32,
}

impl TicTacToeAlphaBeta {
    pub(crate) fn new() -> Self {
        let board = (1..=9).map(|i| (i, " ".to_string())).collect();
        Self {
            board,
            simulation: 0,
        }
    }

    pub fn alphabeta(
        &mut self,
        depth: i32,
        alpha: i32,
        beta: i32,
        is_maximizer: bool,
        ai_xo: &str,
        player_xo: &str,
    ) -> i32 {
        let checked_win = self.check_win().unwrap_or("".to_string());
        if checked_win == ai_xo {
            self.simulation += 1;
            return 1;
        } else if checked_win == player_xo {
            self.simulation += 1;
            return -1;
        } else if self.check_draw() {
            self.simulation += 1;
            return 0;
        }

        let coords: Vec<i32> = self.board.keys().cloned().collect();

        if is_maximizer {
            let mut best_score = i32::MIN;
            let mut alpha = alpha;
            for coord in coords {
                if self.board.get(&coord).unwrap() == " " {
                    self.board.insert(coord, ai_xo.to_string());
                    let score = self.alphabeta(depth + 1, alpha, beta, false, ai_xo, player_xo);
                    self.board.insert(coord, " ".to_string());
                    best_score = best_score.max(score);
                    alpha = alpha.max(score);
                    if beta <= alpha {
                        break;
                    }
                }
            }
            return best_score;
        } else {
            let mut best_score = i32::MAX;
            let mut beta = beta;
            for coord in coords {
                if self.board.get(&coord).unwrap() == " " {
                    self.board.insert(coord, player_xo.to_string());
                    let score = self.alphabeta(depth + 1, alpha, beta, true, ai_xo, player_xo);
                    self.board.insert(coord, " ".to_string());
                    best_score = best_score.min(score);
                    beta = beta.min(score);
                    if beta <= alpha {
                        break;
                    }
                }
            }
            return best_score;
        }
    }

    pub fn calculate_move_alphabeta(&mut self, ai_xo: &str, player_xo: &str) -> i32 {
        let mut best_score = i32::MIN;
        let mut best_move = -1;

        let coords: Vec<i32> = self.board.keys().cloned().collect();

        let mut alpha = i32::MIN;
        let beta = i32::MAX;

        for coord in coords {
            if self.board[&coord] == " " {
                self.board.insert(coord, ai_xo.to_string());
                let score = self.alphabeta(0, alpha, beta, false, ai_xo, player_xo);
                self.board.insert(coord, " ".to_string());
                if score > best_score {
                    best_score = score;
                    best_move = coord;
                }
                alpha = alpha.max(best_score);
            }
        }
        println!("End gmae simulations went thru: {}", self.simulation);
        return best_move;
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

    pub fn make_move(&mut self, move_: i32, is_player: bool, xo: &str) {
        if is_player {
            self.board.insert(move_, xo.to_string());
        } else {
            self.board.insert(move_, xo.to_string());
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

    pub fn play_game_alphabeta(&mut self) {
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
                return;
            } else if human_symbol.to_lowercase() == "help" {
                Self::help_command();
            } else {
                println!("Invalid input. Please type 'X', 'O', 'help', or 'quit'.");
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
                    self.make_move(move_, true, &human_symbol);
                } else {
                    println!("Invalid move. Please try again.");
                    continue;
                }
            } else {
                let best_move = self.calculate_move_alphabeta(&ai_symbol, &human_symbol);
                self.make_move(best_move, false, &ai_symbol);
            }

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
