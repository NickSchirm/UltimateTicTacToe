use crate::board::Board;
use crate::player::Player::{One, Two};

mod bitboard;
mod board;
mod game_result;
mod player;
mod ultimate_board;

fn main() {
    println!("Hello, world!");

    let mut board = Board::new(8);

    board.set(0, One);
    board.set(1, Two);
    board.set(2, One);
    board.set(3, Two);
    board.set(4, One);
    board.set(5, Two);

    println!("{:?}", board.check_if_won());

    for i in board.get_possible_moves() {
        println!("{}", i);
    }

    let mut moves: Vec<_> = ultimate_board::UltimateBoard::new()
        .get_possible_moves()
        .collect();

    moves.sort();

    for i in moves {
        println!("{}", i);
    }
}
