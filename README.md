# Functional & Parallel Programming

This repository contains three primary projects, where each is an implementation of search algorithms; Minimax, Parallel Minimax and Alpha-Beta Pruning with applications in TicTacToe and Reversi using Scala and Rust. The primary goal is to explore the different performance by applying parallelism in such algorithms and evaluate their performance in said games.

## Project Structure
```
.
├── ab-pruning-scala // Working Scala Implementation of TicTacToe Minimax and Alpha-Beta
│   └── src
│       ├── AlphaBetaChecker.scala
│       ├── AlphaBetaReversi.scala
│       └── AlphaBetaTicTacToe.scala
├── reversi // Not working Reversi game implementation
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── reversi.iml
│   └── src
│       └── main.rs
└── tictactoeRust // Working released TicTacToe in Rust with parallelism Minimax
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   ├── main.rs
    │   ├── tictactoeAlphaBeta.rs
    │   ├── tictactoeAlphaBetaPar.rs
    │   ├── tictactoeMinimax.rs
    │   └── tictactoeMinimaxPar.rs
```
