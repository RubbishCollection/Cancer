#[derive(Clone, Copy)]
pub enum DataProcessingRegister {
    DataProcessing2Source,
    DataProcessing1Source,
    LogicalShiftedRegister,
    AddSubtractShiftedRegister,
    AddSubtractExtendedRegister,
    AddSubtractWithCarry,
    RotateRightIntoFlags,
    EvaluateInfoFlags,
    ConditionalCompareRegister,
    ConditionalCompareImmediate,
    ConditionalSelect,
    DataProcessing3Source,
}

#[derive(Clone, Copy)]
pub enum AddSubtractShiftedRegister {
    Unallocated0,
    Unallocated1,
    ADDShiftedRegister32,
    ADDSShiftedReigster32,
    SUBShiftedRegister32,
    SUBSShiftedRegister32,
    ADDShiftedRegister64,
    ADDSShiftedRegister64,
    SUBShiftedRegister64,
    SUBSShiftedRegister64,
}

#[derive(Debug)]
pub struct AddSubtractShiftedRegisterData {
    pub sf: u32,
    pub op: u32,
    pub s: u32,
    pub shift: u32,
    pub imm6: u32,
    pub rm: u32,
    pub rn: u32,
    pub rd: u32,
}
