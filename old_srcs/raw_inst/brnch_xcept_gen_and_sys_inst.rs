use std::fmt::{Debug, Display, Formatter};

use crate::utils::GetBits;

#[derive(Clone, Copy)]
pub struct BranchExceptionGenAndSysInst(pub u32);

impl BranchExceptionGenAndSysInst {
    pub fn get_op0(&self) -> u32 {
        self.0.get_bits(29..32)
    }

    pub fn get_op1(&self) -> u32 {
        self.0.get_bits(12..26)
    }

    pub fn get_op2(&self) -> u32 {
        self.0.get_bits(0..5)
    }
}

impl<T> From<T> for BranchExceptionGenAndSysInst
where
    T: Into<u32>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Display for BranchExceptionGenAndSysInst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BranchExceptionGenAndSysInst")
            .field("self", &self.0)
            .field("op0", &format_args!("{:#b}", self.get_op0()))
            .field("op1", &format_args!("{:#b}", self.get_op1()))
            .field("op2", &format_args!("{:#b}", self.get_op2()))
            .finish()
    }
}

impl Debug for BranchExceptionGenAndSysInst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BranchExceptionGenAndSysInst")
            .field("self", &self.0)
            .field("op0", &format_args!("{:#b}", self.get_op0()))
            .field("op1", &format_args!("{:#b}", self.get_op1()))
            .field("op2", &format_args!("{:#b}", self.get_op2()))
            .finish()
    }
}
