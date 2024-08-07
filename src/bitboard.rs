use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BitBoard(u16);

impl BitBoard {
    pub fn new(n: u16) -> Self {
        debug_assert!(n < 512, "BitBoard value out of bounds");
        BitBoard(n)
    }

    pub const EMPTY: BitBoard = BitBoard(0);

    pub fn first_square(&self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            Some(self.0.trailing_zeros() as u8)
        }
    }

    pub fn pop_first_square(&mut self) -> Option<u8> {
        let square = self.first_square();
        square.inspect(|s| self.0 ^= 1 << *s as u16);
        square
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

pub struct BitBoardIterator {
    board: BitBoard,
}

impl Iterator for BitBoardIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.board.pop_first_square()
    }
}
