use crate::player::Player::{One, Two};

mod bitboard;
mod game_result;
mod player;

fn main() {
    println!("Hello, world!");

    let mut board = bitboard::Bitboard::new(0);

    board.set(0, One);
    board.set(1, Two);
    board.set(2, One);
    board.set(3, Two);
    board.set(4, One);
    board.set(5, Two);
    board.set(6, One);
    board.set(7, Two);
    board.set(8, One);

    println!("{:?}", board.check_if_won());

    for i in board.get_possible_moves() {
        println!("{}", i);
    }
}
