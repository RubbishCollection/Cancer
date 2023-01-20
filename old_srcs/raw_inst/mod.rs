pub mod brnch_xcept_gen_and_sys_inst;
pub mod data_processing_immediate;
pub mod data_processing_register;
pub mod loads_and_stores;

use crate::utils::GetBits;

use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy)]
pub struct RawInst(pub u32);
impl RawInst {
    pub fn get_op0(&self) -> u32 {
        self.0.get_bits(31..32)
    }

    pub fn get_op1(&self) -> u32 {
        self.0.get_bits(25..29)
    }
}

impl Into<u32> for RawInst {
    fn into(self) -> u32 {
        self.0
    }
}

impl Display for RawInst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawInst")
            .field("self", &self.0)
            .field("op0", &format_args!("{:#b}", self.get_op0()))
            .field("op1", &format_args!("{:#b}", self.get_op1()))
            .finish()
    }
}

impl Debug for RawInst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawInst")
            .field("self", &self.0)
            .field("op0", &format_args!("{:#b}", self.get_op0()))
            .field("op1", &format_args!("{:#b}", self.get_op1()))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwise_check_test() {
        let val = RawInst(0b_1111_0000_1010_0101_1100_0011_1001_0110);
    }
}
