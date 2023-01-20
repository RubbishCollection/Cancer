use std::fmt::{Debug, Display, Formatter};

use crate::utils::GetBits;

#[derive(Clone, Copy)]
pub struct DataProcessingImmediate(pub u32);
impl DataProcessingImmediate {
    pub fn get_op0(&self) -> u32 {
        self.0.get_bits(23..26)
    }
}

impl<T> From<T> for DataProcessingImmediate
where
    T: Into<u32>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Display for DataProcessingImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataProcessingImmediate")
            .field("self", &self.0)
            .field("op0", &format_args!("{:#b}", self.get_op0()))
            .finish()
    }
}

impl Debug for DataProcessingImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataProcessingImmediate")
            .field("self", &self.0)
            .field("op0", &format_args!("{:#b}", self.get_op0()))
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct AddSubtractImmediate(pub u32);
impl AddSubtractImmediate {
    pub fn get_sf(&self) -> u32 {
        self.0.get_bits(31..32)
    }

    pub fn get_op(&self) -> u32 {
        self.0.get_bits(30..31)
    }

    pub fn get_S(&self) -> u32 {
        self.0.get_bits(29..30)
    }

    pub fn get_sh(&self) -> u32 {
        self.0.get_bits(22..23)
    }

    pub fn get_imm12(&self) -> u32 {
        self.0.get_bits(10..22)
    }

    pub fn get_rn(&self) -> u32 {
        self.0.get_bits(5..10)
    }

    pub fn get_rd(&self) -> u32 {
        self.0.get_bits(0..5)
    }
}

impl Debug for AddSubtractImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataProcessingImmediate")
            .field("self", &self.0)
            .field("sf", &format_args!("{:#b}", self.get_sf()))
            .field("op", &format_args!("{:#b}", self.get_op()))
            .field("S", &format_args!("{:#b}", self.get_S()))
            .finish()
    }
}
