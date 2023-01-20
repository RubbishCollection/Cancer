#[derive(Clone, Copy)]
pub enum BranchesExceptionGenNSysInstr {
    ConditionalBranchImmediate,
    ExceptionGeneration,
    SystemInstructionsWithRegisterArgument,
    Hints,
    Barriers,
    PSTATE,
    SystemWithResult,
    SystemInstructions,
    SystemRegisterMove,
    UnconditionalBranchRegister,
    UnconditionalBranchImmediate,
    CompareAndBranchImmediate,
    TestAndBranchImmediate,
}

#[derive(Clone, Copy)]
pub enum ConditionalBranchImmediate {
    BCond,
    BcCond,
    Unallocated,
}

#[derive(Debug)]
pub struct ConditionalBranchImmediateData {
    pub o1: u32,
    pub o0: u32,
    pub imm19: u32,
    pub cond: u32,
}
