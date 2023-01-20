use crate::raw_inst::RawInst;
mod raw_inst;

use crate::raw_inst::{
    data_processing_immediate::*, data_processing_register::*, loads_and_stores::*,
};


impl TryInto<AArch64Inst> for RawInst {
    type Error = Error;

    fn try_into(self) -> Result<AArch64Inst, Self::Error> {
        match (self.get_op0(), self.get_op1()) {
            (0b0, 0b0000) => todo!("reserved on page"),
            (0b1, 0b0000) => todo!("SME Encodings"),
            (_, 0b0001 | 0b0011) => todo!("Unallocated"),
            (_, 0b0010) => todo!("SVE Encodings"),
            (_, 0b1000 | 0b1001) => DataProcessingImmediate(self.into()).try_into(),
            (_, 0b1010 | 0b1011) => todo!("Branches, Exception Generating and System Instructions"),
            (_, 0b0100 | 0b0110 | 0b1100 | 0b1110) => LoadsAndStores(self.into()).try_into(),
            (_, 0b0101 | 0b1101) => todo!("Data Processing -- Register"),
            (_, 0b0111 | 0b1111) => {
                todo!("Data Processing -- Scalar Floating-Point and Advanced SIMD")
            }

            _ => unreachable!(),
        }
    }
}

impl TryInto<AArch64Inst> for DataProcessingImmediate {
    type Error = Error;

    fn try_into(self) -> Result<AArch64Inst, Self::Error> {
        match self.get_op0() {
            0b000 | 0b001 => todo!("PC-rel addressing"),
            0b010 => AddSubtractImmediate(self.0).try_into(),
            0b011 => todo!("Add/subtract (immediate, with tags)"),
            0b100 => todo!("Logical(immediate)"),
            0b101 => todo!("Move wide(immediate"),
            0b110 => todo!("Bitfield"),
            0b111 => todo!("Extract"),
            _ => unreachable!(),
        }
    }
}

impl TryInto<AArch64Inst> for AddSubtractImmediate {
    type Error = Error;

    fn try_into(self) -> Result<AArch64Inst, Self::Error> {
        let result = match (self.get_sf(), self.get_op(), self.get_S()) {
            (0, 0, 0) => todo!("32-bit ADD(immediate)"),
            (0, 0, 1) => todo!("32-bit ADDS(immediate)"),
            (0, 1, 0) => todo!("32-bit SUB(immediate)"),
            (0, 1, 1) => todo!("32-bit SUBS(immediate"),

            (1, 0, 0) => todo!("64-bit ADD(immediate)"),
            (1, 0, 1) => todo!("64-bit ADDS(immediate)"),
            (1, 1, 0) => AArch64Inst::SubImmediate64(self),
            (1, 1, 1) => todo!("64-bit SUBS(immediate"),
            _ => unreachable!(),
        };

        Ok(result)
    }
}

impl TryInto<AArch64Inst> for LoadsAndStores {
    type Error = Error;

    fn try_into(self) -> Result<AArch64Inst, Self::Error> {
        match (
            self.get_op0(),
            self.get_op1(),
            self.get_op2(),
            self.get_op3(),
            self.get_op4(),
        ) {
            (op0, op1, op2, op3, _) if op0.test_bits(0b0000, 0b1011) => match (op1, op2, op3) {
                (0b0, 0b00, op3) if op3.test_bits(0b100_000, 0b100_000) => {
                    todo!("Compare and swap pair")
                }
                (0b1, 0b00, 0b000_000) => {
                    todo!("Advanced SIMD load/store multiple structures")
                }
                (0b1, 0b01, op3) if op3.test_bits(0b000_000, 0b100_000) => {
                    todo!("Advanced SIMD load/store multiple structures(post-indexed")
                }
                (0b1, op2, op3)
                    if op2.test_bits(0b00, 0b10) && op3.test_bits(0b100_000, 0b100_000) =>
                {
                    todo!("Unallocated")
                }
                (0b1, 0b10, op3) if op3.test_bits(0b000_000, 0b011_111) => {
                    todo!("Advanced SIMD load/store single structure")
                }
                (0b1, 0b11, _) => {
                    todo!("Advanced SIMD load/store single structure(post-indexed)")
                }
                (0b1, op2, _) if op2.test_bits(0b00, 0b01) => {
                    todo!("Unallocated")
                }
                _ => unreachable!(),
            },

            (0b1101, 0b0, op2, op3, _)
                if op2.test_bits(0b10, 0b10) && op3.test_bits(0b100_000, 0b100_000) =>
            {
                todo!("Load/store memory tags")
            }
            (op0, 0b0, 0b00, op3, _)
                if op0.test_bits(0b1000, 0b1011) && op3.test_bits(0b100_000, 0b100_000) =>
            {
                todo!("Load/store memory pair")
            }
            (op0, 0b1, _, _, _) if op0.test_bits(0b1000, 0b1011) => {
                todo!("Unallocated")
            }
            (op0, 0b0, 0b00, op3, _)
                if op0.test_bits(0b0000, 0b0011) && op3.test_bits(0b000_000, 0b100_000) =>
            {
                todo!("Load/store exclusive register")
            }
            (op0, 0b0, 0b01, op3, _) if op0.test_bits(0b0000, 0b0011) => {
                todo!("Load/store ordered")
            }
            (op0, 0b0, 0b01, op3, _) if op0.test_bits(0b0000, 0b0011) => {
                todo!("Compare and swap")
            }
            (op0, 0b0, op2, op3, 0b00)
                if op0.test_bits(0b0001, 0b0011)
                    && op2.test_bits(0b10, 0b10)
                    && op3.test_bits(0b000_000, 0b100_000) =>
            {
                todo!("LDAPR/STLR(unscaled immediate")
            }
            (op0, _, op2, _, _) if op0.test_bits(0b0001, 0b0011) && op2.test_bits(0b00, 0b10) => {
                todo!("Load register (literal)")
            }
            (op0, _, op2, op3, 0b01)
                if op0.test_bits(0b0001, 0b0011)
                    && op2.test_bits(0b10, 0b10)
                    && op3.test_bits(0b000_000, 0b100_000) =>
            {
                todo!("Memory Copy and Memory Set")
            }
            (op0, _, 0b00, _, _) if op0.test_bits(0b0010, 0b0011) => {
                todo!("Load/store no-allocate pair(offset)")
            }
            (op0, _, 0b01, _, _) if op0.test_bits(0b0010, 0b0011) => {
                todo!("Load/store register pair(post indexed)")
            }
            (op0, _, 0b10, _, _) if op0.test_bits(0b0010, 0b0011) => {
                todo!("Load/store register pair(offset)")
            }
            (op0, _, 0b11, _, _) if op0.test_bits(0b0010, 0b0011) => {
                todo!("Load/store register pair(pre-indexed)")
            }
            (op0, _, op2, op3, op4)
                if op0.test_bits(0b0011, 0b0011)
                    && op2.test_bits(0b00, 0b10)
                    && op3.test_bits(0b000_000, 0b100_000) =>
            {
                match op4 {
                    0b00 => todo!("Load/store register(unscalaed immediate)"),
                    0b01 => todo!("Load/store register(immediate post-indexed)"),
                    0b10 => todo!("Load/store register(unprivileged)"),
                    0b11 => todo!("Load/store register(iimmediate pre-indexed)"),
                    _ => unreachable!(),
                }
            }
            (op0, _, op2, op3, op4)
                if op0.test_bits(0b0011, 0b0011)
                    && op2.test_bits(0b00, 0b10)
                    && op3.test_bits(0b100_000, 0b100_000) =>
            {
                match op4 {
                    0b00 => todo!("Atomic memory operations on page"),
                    0b10 => todo!("Load/store register(register offset"),
                    0b01 | 0b11 => todo!("Load/store register(pac)"),
                    _ => unreachable!(),
                }
            }
            (op0, _, op2, _, _) if op0.test_bits(0b0011, 0b0011) & op2.test_bits(0b10, 0b10) => {
                todo!("Load/store register(unsigned immediate)")
            }
            _ => unreachable!(),
        }
    }
}

impl TryInto<AArch64Inst> for LoadNStoreRegisterUnsignedImdt {
    type Error = Error;

    fn try_into(self) -> Result<AArch64Inst, Self::Error> {
        todo!()
    }
}