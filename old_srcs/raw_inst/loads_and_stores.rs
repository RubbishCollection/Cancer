use crate::utils::GetBits;

#[derive(Clone, Copy, Debug)]
pub struct LoadsAndStores(pub u32);

impl LoadsAndStores {
    pub fn get_op0(&self) -> u32 {
        self.0.get_bits(28..32)
    }

    pub fn get_op1(&self) -> u32 {
        self.0.get_bits(26..27)
    }

    pub fn get_op2(&self) -> u32 {
        self.0.get_bits(23..25)
    }

    pub fn get_op3(&self) -> u32 {
        self.0.get_bits(16..22)
    }

    pub fn get_op4(&self) -> u32 {
        self.0.get_bits(10..12)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LoadNStoreRegisterUnsignedImdt(pub u32);

impl LoadNStoreRegisterUnsignedImdt {
    pub fn get_size(&self) -> u32 {
        self.0.get_bits(30..32)
    }

    pub fn get_V(&self) -> u32 {
        self.0.get_bits(26..27)
    }

    pub fn get_opc(&self) -> u32 {
        self.0.get_bits(22..24)
    }

    pub fn get_imm12(&self) -> u32 {
        self.0.get_bits(10..22)
    }

    pub fn get_rn(&self) -> u32 {
        self.0.get_bits(5..10)
    }

    pub fn get_rt(&self) -> u32 {
        self.0.get_bits(0..5)
    }
}
