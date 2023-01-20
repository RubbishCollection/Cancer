use crate::utils::GetBits;

#[derive(Clone, Copy, Debug)]
struct DataProcessingRegister(pub u32);

impl DataProcessingRegister {
    pub fn get_op0(&self) -> u32 {
        self.0.get_bits(30..31)
    }

    pub fn get_op1(&self) -> u32 {
        self.0.get_bits(28..29)
    }

    pub fn get_op2(&self) -> u32 {
        self.0.get_bits(21..25)
    }

    pub fn get_op3(&self) -> u32 {
        self.0.get_bits(10..16)
    }
}
