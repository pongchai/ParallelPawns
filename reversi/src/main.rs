extern crate core;

use core::num::dec2flt::parse;
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{Duration, Instant};
use indexmap::IndexSet;
use ansi_term::Color::{Red, Green};
use ansi_term::Style;
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator};
use regex::Regex;

struct Board {
    width: u8,
    height: u8,
    board_dimension: u8,
    board: Vec<u8>,
    player_legal_moves: HashSet<u8>,
    ai_legal_moves: HashSet<u8>,
    is_player_turn: bool,
    walls: Vec<usize>
}

impl Board {
    fn new(w: u8, h: u8) -> Board {
        let dimension = w * h;
        let mut new_board = vec![0; (dimension).into()];
        let mut player_legal_moves: HashSet<u8> = HashSet::new();
        let mut ai_legal_moves: HashSet<u8> = HashSet::new();
        let walls = vec![0,1,2,3,4,5,6,7,8,16,24,32,40,48,56,15,23,31,39,47,55,63,57,58,59,60,61,62,63];

        new_board[28] = 1; // player = 1 = red
        new_board[35] = 1;
        new_board[27] = 2; // ai = 2 = green
        new_board[36] = 2;

        player_legal_moves.insert(26);
        player_legal_moves.insert(19);
        player_legal_moves.insert(37);
        player_legal_moves.insert(44);

        ai_legal_moves.insert(29);
        ai_legal_moves.insert(20);
        ai_legal_moves.insert(34);
        ai_legal_moves.insert(43);


        Board {
            width: w,
            height: h,
            board_dimension: dimension,
            board: new_board, //must convert u8 type -> usize type
            player_legal_moves,
            ai_legal_moves,
            is_player_turn: true,
            walls,
        }
    }

    fn clone(&self) -> Board {
        let new_board: Board = Board {
            width: self.width,
            height: self.height,
            board_dimension: self.board_dimension,
            board: self.board.clone(),
            player_legal_moves: self.player_legal_moves.clone(),
            ai_legal_moves: self.ai_legal_moves.clone(),
            is_player_turn: self.is_player_turn,
            walls: self.walls.clone(),
        };
        new_board
    }

    fn check_game_state(&self) -> i32 {
        let (player_score, ai_score) = self.calculate_score();
        let total = player_score as i32 + ai_score as i32;
        if total == self.board_dimension as i32 {
            if player_score == ai_score { 3 } // draw
            else if player_score > ai_score {// player won
                1
            } else {
                2
            }
        }
        else {
            4
        }
    }

    pub fn alpha_beta(
        &mut self,
        copied_board: Board,
        depth: i32,
        alpha: i32,
        beta: i32,
        is_maximizer: bool,
    ) -> i32 {
        let score = copied_board.check_game_state();
        if depth == 0 {
            let (player_score, ai_score) = self.calculate_score();
            return player_score as i32 - ai_score as i32;
        }
        else if score == 1 {
            self.simulation += 1;
            return 100;
        } else if score == 2 {
            self.simulation += 1;
            return -100;
        } else if score == 3 {
            self.simulation += 1;
            return 0;
        }

        if is_maximizer {
            let mut best_score = i32::MIN;
            let mut alpha = alpha;
            for coord in self.board_dimension {
                if self.board[coord] == "0" {
                    self.board.insert(coord, 2);
                    let score = self.alpha_beta(self.clone(), depth - 1, alpha, beta, false);
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
            for coord in self.board_dimension {
                if self.board.get(&coord).unwrap() == " " {
                    self.board.insert(coord, 1);
                    let score = self.alpha_beta(self.clone(),depth + 1, alpha, beta, true);
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

    pub fn calculate_move_alpha_beta(&mut self) -> i32 {
        let mut best_score = i32::MIN;
        let mut best_move = -1;

        let mut alpha = i32::MIN;
        let beta = i32::MAX;

        for coord in self.board_dimension {
            if self.board[&coord] == 0 {
                self.board.insert(coord, 2);
                let score = self.alpha_beta(self.clone(),0, alpha, beta, false);
                if score > best_score {
                    best_score = score;
                    best_move = coord;
                }
                alpha = alpha.max(best_score);
            }
        }
        println!("End game simulations went thru: {}", self.simulation);
        return best_move;
    }

    pub fn minimax(&mut self, mut copied_board: Board, depth: i32, is_maximizer: bool) -> i32 {
        let score = copied_board.check_game_state();
        if depth == 0 {
            let (player_score, ai_score) = self.calculate_score();
            return player_score as i32 - ai_score as i32;
        }
        else if score == 1 {
            self.simulation += 1;
            return 100;
        } else if score == 2 {
            self.simulation += 1;
            return -100;
        } else if score == 3 {
            self.simulation += 1;
            return 0;
        }

        if is_maximizer {
            let mut best_score = i32::MIN;
            for coord in copied_board.board_dimension {
                if copied_board.board[&coord] == " " {
                    copied_board.board.insert(coord, 2);
                    let score = self.minimax(copied_board.clone(), depth - 1, false);
                    best_score = best_score.max(score);
                }
            }
            return best_score;
        } else {
            let mut best_score = i32::MAX;
            for coord in copied_board.board_dimension {
                if copied_board.board[&coord] == " " {
                    copied_board.board.insert(coord, 1);
                    let score = self.minimax(copied_board.clone(), depth - 1, true);
                    best_score = best_score.min(score);
                }
            }
            return best_score;
        }
    }

    pub fn calculate_move_mini(&mut self) -> i32 {
        let mut best_score = i32::MIN;
        let mut best_move = -1;

        for coord in coords {
            if self.board[&coord] == " " {
                self.board.insert(coord, 2);
                let score = self.minimax(self.clone(), 0, false);
                if score > best_score {
                    best_score = score;
                    best_move = coord;
                }
            }
        }
        println!("End game simulations went thru: {}", self.simulation);
        best_move
    }

    pub fn minimax_par(
        &self,
        mut copied_board: Board,
        depth: i32,
        is_maximizer: bool,
        simulation_count: &AtomicI32,
    ) -> (i32, i32) {
        let score = copied_board.check_game_state();
        if depth == 0 {
            let (player_score, ai_score) = copied_board.calculate_score();
            return (player_score as i32 - ai_score as i32, 1);
        }
        else if score == 1 {
            return (100, 1);
        } else if score == 2 {
            return (-100, 1);
        } else if score == 3 {
            return (0, 1);
        }

        let ai_moves = copied_board.ai_legal_moves;
        let player_moves = copied_board.player_legal_moves;

        let result = if is_maximizer {
            ai_moves
                .par_iter()
                .map(|&move_| {
                    let cloned_self = self.make_move(move_, ai_xo);
                    let (score, simulations) =
                        cloned_self.minimax(depth - 1, false, 2, 1, simulation_count);
                    (score, simulations)
                })
                .max_by_key(|&(score, _)| score)
                .unwrap_or((i32::MIN, 1))
        } else {
            player_moves
                .par_iter()
                .map(|&move_| {
                    let cloned_self = self.make_move(move_, player_xo);
                    let (score, simulations) =
                        cloned_self.minimax(depth - 1, true, 2, 1, simulation_count);
                    (score, simulations)
                })
                .min_by_key(|&(score, _)| score)
                .unwrap_or((i32::MAX, 1))
        };

        (result.0, moves.len() as i32 * result.1)
    }

    pub fn calculate_move_mini_par(&self) -> i32 {
        let simulation_count = AtomicI32::new(0);

        let results = self.ai_legal_moves
            .par_iter()
            .map(|&move_| {
                let cloned_self = self.make_move(move_, ai_xo);
                let (eval, simulations) =
                    cloned_self.minimax_par(0, false, 2, 1, &simulation_count);
                (move_, eval, simulations)
            })
            .collect::<Vec<(i32, i32, i32)>>();

        let best_move = results
            .iter()
            .max_by_key(|&(_, eval, _)| *eval)
            .map(|&(move_, _, _)| move_)
            .unwrap_or(0);

        println!(
            "End game simulations went thru: {}",
            simulation_count.load(Ordering::Relaxed)
        );
        best_move
    }

    fn print_board(&self) {
        let (player_score, ai_score) = self.calculate_score();
        println!("\n          {}", Style::default().bold().paint("A B C D E F G H") );
        let mut count = 0;
        for cell in self.board.iter() {
            if count % self.width == 0 {
                if count != 0 {
                    let row_num: u8 = count / 8;
                    print!("{}\n          ", Style::default().bold().paint(row_num.to_string()));
                } else {
                    print!("          ")
                }
            }
            if cell == &1 {
                print!("{} ", Red.paint("X"));
            } else if cell == &2 {
                print!("{} ", Green.paint("O"));
            } else {
                if self.player_legal_moves.contains(&count) {
                    print!("{} ", Style::default().bold().paint("*"));
                } else {
                    print!("- ");
                }
            }
            count += 1;
        }
        print!("{}\n\n", Style::default().bold().paint("8"));
        print!("          Player: {}, AI: {}\n", Red.paint(player_score.to_string()), Green.paint(ai_score.to_string()));
    }

    fn calculate_score(&self) -> (u8, u8) {
        let mut player_count = 0;
        let mut ai_count = 0;

        for cell in self.board.iter() {
            match *cell {
                1 => player_count += 1,
                2 => ai_count += 1,
                _ => {}
            }
        }
        (player_count, ai_count)
    }

    // get legal moves of current player
    fn get_legal_moves(&mut self, curr_coord: u8, dir: i32) {
        // check current player position and see if it
        // got any same piece across every direction
        let mut new_coord = curr_coord as i32;
        let mut found_enemy_once = false;

        let mut new_player_legal_moves = HashSet::new();
        let mut new_ai_legal_moves = HashSet::new();

        while new_coord >= 0 && new_coord < self.board_dimension as i32 && !self.walls.contains(&(new_coord as usize)){
            new_coord += dir;
            println!("{}", new_coord);
            if new_coord >= 0 && new_coord < self.board_dimension as i32 && !self.walls.contains(&(new_coord as usize)) {
                if self.board[new_coord as usize] == 0 {
                    break;
                }
                else {
                    if self.board[curr_coord as usize] == 1 {// player
                        if self.board[new_coord as usize] == 2 {// found enemy
                            found_enemy_once = true;
                            continue
                        }
                        else if self.board[new_coord as usize] == 1 { // found itself
                            if found_enemy_once {
                                println!("player got legal move: {}", new_coord);
                                new_player_legal_moves.insert(curr_coord as u8);
                                return
                            }
                        }
                    }
                    else if self.board[curr_coord as usize] == 2 { // ai
                        if self.board[new_coord as usize] == 1 {
                            found_enemy_once = true;
                            continue
                        }
                        else if self.board[new_coord as usize] == 2 {
                            if found_enemy_once {
                                println!("ai got legal move: {}", curr_coord);
                                new_ai_legal_moves.insert(curr_coord as u8);
                                return
                            }
                        }
                    }
                }
            }
        }
        self.player_legal_moves = new_player_legal_moves;
        self.ai_legal_moves = new_ai_legal_moves;
    }

    fn check_flip(&mut self, coord: u8, dir: i32) -> bool {
        let mut new_coord: i32 = coord as i32;
        let player = if self.is_player_turn { 1 } else { 2 };
        let opp = if !self.is_player_turn { 2 } else { 1 };

        if self.board[new_coord as usize] == opp {
            while new_coord >= 0 && new_coord < self.board_dimension as i32 {
                new_coord += dir;

                if (new_coord < 0) || (new_coord > self.board_dimension as i32) || self.walls.contains(&(new_coord as usize)) {return false}

                if self.board[new_coord as usize] == 0 { return false }
                if self.board[new_coord as usize] == player { return true }
            }
        }
        false
    }

    fn flip_pieces(&mut self, coord: u8, dir: isize) {
        let mut cell = coord as isize;

        while cell >= 0 && cell < self.board_dimension as isize {
            cell += dir;

            if cell >= 0 && cell < 64 {
                if self.board[cell as usize] == 0 {
                    break;
                }
                else {
                    if self.is_player_turn {
                        self.board[cell as usize] = 1
                    }
                    else {
                        self.board[cell as usize] = 2
                    }
                }
            }
        }
    }

    fn update(&mut self, coord: u8) {
        // check if coord aka move is valid for current player by checking legal moves list
        if self.is_player_turn {
            match self.player_legal_moves.contains(&coord) {
                true => { self.board[coord as usize] = 1 }
                false => {
                    println!("User made an {} invalid move", coord);
                    return
                }
            }
        }
        else {
            match self.ai_legal_moves.contains(&coord) {
                true => { self.board[coord as usize] = 2 }
                false => {
                    println!("AI made an {} invalid move", coord);
                    return
                }
            }
        }

        // if above is true, then we will find all directions of coord user chose
        // that has same piece opposite/across/diagonally across the piece and flip it.
        if self.check_flip(coord, -9) { // up-left
            self.flip_pieces(coord, -9);
        }
        if self.check_flip(coord, -8) { // up
            self.flip_pieces(coord, -8);
        }
        if self.check_flip(coord, -7) { // up-right
            self.flip_pieces(coord, -7);
        }
        if self.check_flip(coord, -1) { // left
            self.flip_pieces(coord, -1);
        }
        if self.check_flip(coord, 1) { // right
            self.flip_pieces(coord, 1);
        }
        if self.check_flip(coord, 7) { // down-left
            self.flip_pieces(coord, 7);
        }
        if self.check_flip(coord, 8) { // down
            self.flip_pieces(coord, 8);
        }
        if self.check_flip(coord, 9) { // down-right
            self.flip_pieces(coord, 9);
        }


        // generate new legal_moves list for both player/ai
        self.player_legal_moves = HashSet::new();
        self.ai_legal_moves = HashSet::new();

        (0..self.board_dimension).into_par_iter().for_each(|i| {
            if self.board[i as usize] != 0 {
                self.get_legal_moves(i, -9); // up-left
                self.get_legal_moves(i, -8); // up
                self.get_legal_moves(i, -7); // up-right
                self.get_legal_moves(i , -1); // left
                self.get_legal_moves(i, 1); // right
                self.get_legal_moves(i, 7); // down-left
                self.get_legal_moves(i, 8); // down
                self.get_legal_moves(i, 9); // down-right
            }
        });

        // println!("{:?}", self.player_legal_moves);
        // println!("{:?}", self.ai_legal_moves);

        // change player turn
        self.is_player_turn = !self.is_player_turn;
    }


}

fn main() {

    const WIDTH: u8 = 8;
    const HEIGHT: u8 = 8;

    let mut select_depth = String::new();
    let mut select_algo = String::new();

    loop {
        println!("select search depth [1,2]: ");
        io::stdin().read_line(&mut select_depth).expect("failure to read line");

        match select_depth.trim().to_string().as_str() {
            "1" => {
                break;
            },
            "2" => {
                break;
            },
            _ => {
                println!("invalid entry. please select");
                select_depth = String::new();
                continue;
            }
        };
    }

    loop {
        println!("select a search algorithm: ");
        println!("1. Minimax");
        println!("2. Parallel Minimax");
        println!("3. Alpha-Beta Pruning");
        io::stdin().read_line(&mut select_algo).expect("failure to read line");

        match select_algo.trim().to_string().as_str() {
            "1" => {
                break;
            },
            "2" => {
                break;
            },
            "3" => {
                break;
            },
            _ => {
                println!("invalid entry. please select");
                select_algo = String::new();
                continue;
            }
        };
    }

    let depth = select_depth.trim().to_string();
    let algo = select_algo.trim().to_string();
    let mut board = Board::new(WIDTH, HEIGHT);

    loop {
        match board.check_game_state() {
            1 => {
                println!("Player has won");
                board.print_board();
                break;
            },
            2 => {
                println!("AI has won");
                board.print_board();
                break;
            },
            3 => {
                println!("A draw");
                board.print_board();
                break;
            },
            _ => ()
        };

        board.print_board();

        let mut inp = String::new();
        if board.is_player_turn { // player
            println!("select coordinate: ");
            io::stdin().read_line(&mut inp).expect("failure to read line");

            let inp = parse_input(&inp);

            board.update(inp)
        }
        else { // ai
            match algo.to_string() {
                String::new("1") => {
                    let ai_move = board.calculate_move_mini();
                    board.update(ai_move as u8)
                }
                String::new("2") => {
                    let ai_move = board.calculate_move_mini_par();
                    board.update(ai_move as u8)
                }
                String::new("3") => {
                    let ai_move = board.calculate_move_alpha_beta();
                    board.update(ai_move as u8)
                }
                _ => ()
            };
        }
    }
}

fn parse_input(inp: &str) -> u8 {
    let mut res = 0;
    let letter = inp.chars().next().unwrap().to_ascii_lowercase();
    let num = inp.chars().nth(1).unwrap();

        let col = match letter {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _   => 99
        };
        let row = match num {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _   => 99
        };
        res = row * 8 + col;
    // println!("{}", res);
    return res
}