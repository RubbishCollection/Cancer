#[derive(Clone, Copy)]
pub enum DataProcessingImmediate {
    PCrelAddressing,
    AddSubtractImmediate,
    AddSubtractImmediateWithTags,
    LogicalImmediate,
    MoveWideImmediate,
    Bitfield,
    Extract,
}

#[derive(Clone, Copy)]
pub enum AddSubtractImmediate {
    ADDImmediate32,
    ADDSImmediate32,
    SUBImmediate32,
    SUBSImmediate32,
    ADDImmediate64,
    ADDSImmediate64,
    SUBImmediate64,
    SUBSImmediate64,
}

#[derive(Debug)]
pub struct AddSubtractImmediateData {
    pub sf: u32,
    pub op: u32,
    pub s: u32,
    pub sh: u32,
    pub imm12: u32,
    pub rn: u32,
    pub rd: u32,
}
