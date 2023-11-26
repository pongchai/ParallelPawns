mod tictactoeMinimaxPar;
use tictactoeMinimaxPar::TicTacToePar;
mod tictactoeMinimax;
use tictactoeMinimax::TicTacToe;
mod tictactoeAlphaBeta;
use tictactoeAlphaBeta::TicTacToeAlphaBeta;
mod tictactoeAlphaBetaPar;
use std::io;
use tictactoeAlphaBetaPar::TicTacToeAlphaBetaPar;

fn main() {
    loop {
        println!("\nWhich version would you like to play?");
        println!("1. Minimax");
        println!("2. Parallel Minimax");
        println!("3. Alpha-Beta Pruning");
        // println!("4. Parallel Alpha-Beta Pruning");
        println!("Quit the game by typing 'quit'.");
        println!("Type 'help' for a list of commands.");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input.");
        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "1" => {
                let mut game = TicTacToe::new();
                game.play_game_minimax();
            }
            "2" => {
                let mut game = TicTacToePar::new();
                game.play_game_minimax_par();
            }
            "3" => {
                let mut game = TicTacToeAlphaBeta::new();
                game.play_game_alphabeta();}
            // }
            // "4" => {
            //     let mut game = TicTacToeAlphaBetaPar::new();
            //     game.play_game_alphabeta_par();
            // }
            "quit" => {
                println!("Exiting the game.");
                break;
            }
            "help" => {
                TicTacToe::help_command();
            }
            _ => {
                println!("Invalid input. Please enter 1, 2, or 3.");
            }
        }
    }
}
