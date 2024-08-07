//! # Contains the [BitBoard] struct
//! The BitBoard struct represents a bitboard used for storing the state of the board.
//! The bitboard is a 9-bit integer where each bit represents a square on the board.
//!
//! Two bitboards are used in [Board](crate::game::board::Board) to represent the state.
//!
//! The BitBoard struct implements the [Not], [BitOr], [BitAnd], [BitXor], [BitOrAssign], [BitAndAssign], [BitXorAssign] traits.

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// # Struct representing a bitboard
///
/// A bitboard is a 9-bit integer where each bit represents a square on the board.
/// # Fields
/// * `0` - The bitboard value as an u16
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BitBoard(
    /// 9-bit integer encoded as an u16, the upper 7 bits are always unset
    u16,
);

impl BitBoard {
    /// Creates a new BitBoard with the given value
    /// # Arguments
    /// * `n` - The value of the bitboard, must be in 0..2^9
    pub fn new(n: u16) -> Self {
        debug_assert!(n < u16::pow(2, 9), "BitBoard value out of bounds");
        BitBoard(n)
    }

    /// Contains a BitBoard with no squares set
    pub const EMPTY: Self = BitBoard(0);

    /// Returns the index of the first set square in the board.
    /// If no square is set, returns None.
    /// # Returns
    /// The index of the first set square in the board
    pub fn first_square(&self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            Some(self.0.trailing_zeros() as u8)
        }
    }

    /// Pops the first set square from the board.
    /// If no square is set, returns None.
    /// # Returns
    /// The index of the first set square in the board
    pub fn pop_first_square(&mut self) -> Option<u8> {
        let square = self.first_square();
        square.inspect(|s| self.0 ^= 1 << *s as u16);
        square
    }
}

impl From<BitBoard> for u32 {
    fn from(board: BitBoard) -> Self {
        board.0 as u32
    }
}

impl From<BitBoard> for u16 {
    fn from(board: BitBoard) -> Self {
        board.0
    }
}

impl IntoIterator for BitBoard {
    type Item = u8;
    type IntoIter = BitBoardIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIterator { board: self }
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitBoard(!self.0 & 0b111111111)
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        BitBoard::EMPTY
    }
}

/// # Iterator over the set squares in a BitBoard
///
/// The iterator returns the index of the set squares in the board.
pub struct BitBoardIterator {
    board: BitBoard,
}

impl Iterator for BitBoardIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.board.pop_first_square()
    }
}

#[test]
fn test_iterator() {
    let one = BitBoard::new(187);
    let two = BitBoard::new(64);
    let b = !(one | two);
    println!("{:?}", b.into_iter().collect::<Vec<_>>());
}
