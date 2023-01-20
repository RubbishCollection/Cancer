use std::{
    ops::{BitAnd, BitXor, Range, Shl, Shr},
    slice::Iter,
};

impl GetBits for u32 {}
pub trait GetBits
where
    Self: Shl<u8> + Sized + Clone,
    <Self as Shl<u8>>::Output: Shr<u8>,
{
    fn get_bits(&self, range: &Range<u8>) -> <<Self as Shl<u8>>::Output as Shr<u8>>::Output {
        let lshift = 32 - range.end;
        let left_shifted = self.clone() << lshift;

        left_shifted >> (range.start + lshift)
    }
}

pub trait TestBits {
    fn test_bits(&self, pattern: Self, target_mask: Self) -> bool;
}

impl TestBits for u32 {
    fn test_bits(&self, pattern: Self, target_mask: Self) -> bool {
        (!(self ^ pattern) & target_mask) == target_mask
    }
}

pub struct InstReader<T> {
    iter: T,
}

impl<T> InstReader<T> {
    pub fn new(iter: T) -> Self {
        Self { iter }
    }
}

impl<T> Iterator for InstReader<T>
where
    T: Iterator<Item = u8>,
{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inst: u32 = 0;
        for i in 0..4 {
            let byte = self.iter.next()?;
            inst |= (byte as u32) << (i * 8);
        }

        Some(inst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bits_test() {
        let val: u32 = 0b1111_0000_1010_0101_1100_0011_1001_0110;

        assert_eq!(val.get_bits(&(0..4)), 0b0110);
        assert_eq!(val.get_bits(&(4..12)), 0b0011_1001);
        assert_eq!(val.get_bits(&(12..24)), 0b1010_0101_1100);
        assert_eq!(val.get_bits(&(28..32)), 0b1111);
    }

    #[test]
    fn test_bits_test() {
        let val: u32 = 0b11_00_10;

        assert!(val.test_bits(0b11_00_00, 0b11_11_00));
        assert!(val.test_bits(0b00_01_10, 0b00_10_11));
    }
}
