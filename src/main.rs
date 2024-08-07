mod bitboard;

fn main() {
    println!("Hello, world!");

    let mut board = bitboard::Bitboard::new(0);

    board.set(0, 0);
    board.set(1, 1);
    board.set(2, 0);

    for i in board.get_possible_moves() {
        println!("{}", i);
    }
}
