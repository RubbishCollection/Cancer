pub(crate) mod brnch_xcept_gen_sys_instr;
pub(crate) mod data_processing_immediate;
pub(crate) mod data_processing_register;
pub(crate) mod loads_and_stores;

#[derive(Clone, Copy)]
pub(crate) enum MainEncodingTable {
    Reserved,
    SmeEncodings,
    Unallocated0,
    SveEncodings,
    Unallocated1,
    DataProcessingImmediate,
    BranchesExceptionGenNSysInstr,
    LoadsAndStores,
    DataProcessingRegister,
    DataProcessingScalarFloatingPointAndAdvancedSIMD,
}
