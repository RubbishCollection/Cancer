use crate::instructions::brnch_xcept_gen_sys_instr::{
    BranchesExceptionGenNSysInstr, ConditionalBranchImmediate, ConditionalBranchImmediateData,
};
use crate::instructions::data_processing_register::{
    AddSubtractShiftedRegister, AddSubtractShiftedRegisterData, DataProcessingRegister,
};
use crate::instructions::loads_and_stores::{
    LoadStoreRegisterUnsignedImmediate, LoadStoreRegisterUnsignedImmediateData, LoadsAndStores,
};
use crate::instructions::{self, data_processing_immediate::*};
use crate::instructions::{brnch_xcept_gen_sys_instr, MainEncodingTable};
use crate::pattern_matcher::PatternMatcher;
use crate::AArch64Inst;

pub struct AArch64Parser {
    main_encoding_pm: PatternMatcher<MainEncodingTable>,

    data_processing_immediate_pm: PatternMatcher<DataProcessingImmediate>,
    add_subtract_immediate_pm: PatternMatcher<AddSubtractImmediate>,

    brnch_xcept_gen_sys_instr_pm: PatternMatcher<BranchesExceptionGenNSysInstr>,
    conditional_branch_immediate_pm: PatternMatcher<ConditionalBranchImmediate>,

    loads_and_stores_pm: PatternMatcher<LoadsAndStores>,
    load_store_register_unsigned_immediate_pm: PatternMatcher<LoadStoreRegisterUnsignedImmediate>,

    data_processing_register_pm: PatternMatcher<DataProcessingRegister>,
    add_subtract_shifted_register_pm: PatternMatcher<AddSubtractShiftedRegister>,
}

impl AArch64Parser {
    pub fn new() -> Self {
        let main_encoding_pm = PatternMatcher::<MainEncodingTable>::builder()
            .args("op0", 31..32)
            .args("op1", 25..29)
            .inst(MainEncodingTable::Reserved)
            .with("0")
            .with("0000")
            .inst(MainEncodingTable::SmeEncodings)
            .with("1")
            .with("0000")
            .inst(MainEncodingTable::Unallocated0)
            .with("x")
            .with("0001")
            .inst(MainEncodingTable::SveEncodings)
            .with("x")
            .with("0010")
            .inst(MainEncodingTable::Unallocated1)
            .with("x")
            .with("0011")
            .inst(MainEncodingTable::DataProcessingImmediate)
            .with("x")
            .with("100x")
            .inst(MainEncodingTable::BranchesExceptionGenNSysInstr)
            .with("x")
            .with("101x")
            .inst(MainEncodingTable::LoadsAndStores)
            .with("x")
            .with("x1x0")
            .inst(MainEncodingTable::DataProcessingRegister)
            .with("x")
            .with("x101")
            .inst(MainEncodingTable::DataProcessingScalarFloatingPointAndAdvancedSIMD)
            .with("x")
            .with("x111")
            .build();

        let data_processing_immediate_pm = PatternMatcher::<DataProcessingImmediate>::builder()
            .args("op0", 23..26)
            .inst(DataProcessingImmediate::PCrelAddressing)
            .with("00x")
            .inst(DataProcessingImmediate::AddSubtractImmediate)
            .with("010")
            .inst(DataProcessingImmediate::AddSubtractImmediateWithTags)
            .with("011")
            .inst(DataProcessingImmediate::LogicalImmediate)
            .with("100")
            .inst(DataProcessingImmediate::MoveWideImmediate)
            .with("101")
            .inst(DataProcessingImmediate::Bitfield)
            .with("110")
            .inst(DataProcessingImmediate::Extract)
            .with("111")
            .build();

        let add_subtract_immediate_pm = PatternMatcher::<AddSubtractImmediate>::builder()
            .args("sf", 31..32)
            .args("op", 30..31)
            .args("S", 29..30)
            .args("sh", 22..23)
            .args("imm12", 10..22)
            .args("Rn", 5..10)
            .args("Rd", 0..5)
            .inst(AddSubtractImmediate::ADDImmediate32)
            .with("0")
            .with("0")
            .with("0")
            .inst(AddSubtractImmediate::ADDSImmediate32)
            .with("0")
            .with("0")
            .with("1")
            .inst(AddSubtractImmediate::SUBImmediate32)
            .with("0")
            .with("1")
            .with("0")
            .inst(AddSubtractImmediate::SUBSImmediate32)
            .with("0")
            .with("1")
            .with("1")
            .inst(AddSubtractImmediate::ADDImmediate64)
            .with("1")
            .with("0")
            .with("0")
            .inst(AddSubtractImmediate::ADDSImmediate64)
            .with("1")
            .with("0")
            .with("1")
            .inst(AddSubtractImmediate::SUBImmediate64)
            .with("1")
            .with("1")
            .with("0")
            .inst(AddSubtractImmediate::SUBSImmediate64)
            .with("1")
            .with("1")
            .with("1")
            .build();

        let brnch_xcept_gen_sys_instr_pm =
            PatternMatcher::<BranchesExceptionGenNSysInstr>::builder()
                .args("op0", 29..32)
                .args("op1", 12..26)
                .args("op2", 0..5)
                .inst(BranchesExceptionGenNSysInstr::ConditionalBranchImmediate)
                .with("010")
                .with("0x_xxxx_xxxx_xxxx")
                .with("xxxxx")
                .inst(BranchesExceptionGenNSysInstr::ExceptionGeneration)
                .with("110")
                .with("00_xxxx_xxxx_xxxx")
                .with("xxxxx")
                .inst(BranchesExceptionGenNSysInstr::SystemInstructionsWithRegisterArgument)
                .with("110")
                .with("01_0000_0011_0001")
                .with("xxxxx")
                .inst(BranchesExceptionGenNSysInstr::Hints)
                .with("110")
                .with("01_0000_0011_0010")
                .with("11111")
                .inst(BranchesExceptionGenNSysInstr::Barriers)
                .with("110")
                .with("01_0000_0011_0011")
                .with("xxxxx")
                .inst(BranchesExceptionGenNSysInstr::PSTATE)
                .with("110")
                .with("01_0000_0xxx_0100")
                .with("xxxxx")
                .inst(BranchesExceptionGenNSysInstr::SystemWithResult)
                .with("110")
                .with("01_0010_0xxx_xxxx")
                .with("xxxxx")
                .inst(BranchesExceptionGenNSysInstr::SystemInstructions)
                .with("110")
                .with("01_00x0_1xxx_xxxx")
                .with("xxxxx")
                .inst(BranchesExceptionGenNSysInstr::SystemRegisterMove)
                .with("110")
                .with("01_00x1_xxxx_xxxx")
                .with("xxxxx")
                .inst(BranchesExceptionGenNSysInstr::UnconditionalBranchRegister)
                .with("110")
                .with("1x_xxxx_xxxx_xxxx")
                .with("xxxxx")
                .inst(BranchesExceptionGenNSysInstr::UnconditionalBranchImmediate)
                .with("x00")
                .with("xx_xxxx_xxxx_xxxx")
                .with("xxxxx")
                .inst(BranchesExceptionGenNSysInstr::CompareAndBranchImmediate)
                .with("x01")
                .with("0x_xxxx_xxxx_xxxx")
                .with("xxxxx")
                .inst(BranchesExceptionGenNSysInstr::TestAndBranchImmediate)
                .with("x01")
                .with("1x_xxxx_xxxx_xxxx")
                .with("xxxxx")
                .build();
        let conditional_branch_immediate_pm =
            PatternMatcher::<ConditionalBranchImmediate>::builder()
                .args("o1", 24..25)
                .args("o0", 4..5)
                .args("imm19", 5..24)
                .args("cond", 0..4)
                .inst(ConditionalBranchImmediate::BCond)
                .with("0")
                .with("0")
                .inst(ConditionalBranchImmediate::BcCond)
                .with("0")
                .with("1")
                .inst(ConditionalBranchImmediate::Unallocated)
                .with("1")
                .with("x")
                .build();

        let loads_and_stores_pm = PatternMatcher::<LoadsAndStores>::builder()
            .args("op0", 28..32)
            .args("op1", 26..27)
            .args("op2", 23..25)
            .args("op3", 16..22)
            .args("op4", 10..12)
            .inst(LoadsAndStores::CompareAndSwapPair)
            .with("0x00")
            .with("0")
            .with("00")
            .with("1xx_xxx")
            .with("xx")
            .inst(LoadsAndStores::AdvancedSIMDLoadStoreMultipleStructures)
            .with("0x00")
            .with("1")
            .with("00")
            .with("000_000")
            .with("xx")
            .inst(LoadsAndStores::AdvancedSIMDLoadStoreMultipleStructuresPostIndexed)
            .with("0x00")
            .with("1")
            .with("01")
            .with("0xx_xxx")
            .with("xx")
            .inst(LoadsAndStores::Unallocated0)
            .with("0x00")
            .with("1")
            .with("0x")
            .with("1xx_xxx")
            .with("xx")
            .inst(LoadsAndStores::AdvancedSIMDLoadStoreSingleStructure)
            .with("0x00")
            .with("1")
            .with("10")
            .with("x00_000")
            .with("xx")
            .inst(LoadsAndStores::AdvancedSIMDLoadStoreSingleStructurePostIndexed)
            .with("0x00")
            .with("1")
            .with("11")
            .with("xxx_xxx")
            .with("xx")
            .inst(LoadsAndStores::Unallocated1)
            .with("0x00")
            .with("1")
            .with("x0")
            .with("x1x_xxx")
            .with("xx")
            .inst(LoadsAndStores::Unallocated2)
            .with("0x00")
            .with("1")
            .with("x0")
            .with("xx1_xxx")
            .with("xx")
            .inst(LoadsAndStores::Unallocated3)
            .with("0x00")
            .with("1")
            .with("x0")
            .with("xxx_1xx")
            .with("xx")
            .inst(LoadsAndStores::Unallocated4)
            .with("0x00")
            .with("1")
            .with("x0")
            .with("xxx_x1x")
            .with("xx")
            .inst(LoadsAndStores::Unallocated5)
            .with("0x00")
            .with("1")
            .with("x0")
            .with("xxx_xx1")
            .with("xx")
            .inst(LoadsAndStores::LoadStoreMemoryTags)
            .with("1101")
            .with("0")
            .with("1x")
            .with("1xx_xxx")
            .with("xx")
            .inst(LoadsAndStores::LoadStoreExclusivePair)
            .with("1x00")
            .with("0")
            .with("00")
            .with("1xx_xxx")
            .with("xx")
            .inst(LoadsAndStores::Unallocated6)
            .with("1x00")
            .with("1")
            .with("xx")
            .with("xxx_xxx")
            .with("xx")
            .inst(LoadsAndStores::LoadStoreExclusiveRegister)
            .with("xx00")
            .with("0")
            .with("00")
            .with("0xx_xxx")
            .with("xx")
            .inst(LoadsAndStores::LoadStoreOrdered)
            .with("xx00")
            .with("0")
            .with("01")
            .with("0xx_xxx")
            .with("xx")
            .inst(LoadsAndStores::CompareAndSwap)
            .with("xx00")
            .with("0")
            .with("01")
            .with("1xx_xxx")
            .with("xx")
            .inst(LoadsAndStores::LDAPRSTLRUnscalaedImmediate)
            .with("xx01")
            .with("0")
            .with("1x")
            .with("0xx_xxx")
            .with("00")
            .inst(LoadsAndStores::LoadRegisterLiteral)
            .with("xx01")
            .with("x")
            .with("0x")
            .with("xxx_xxx")
            .with("xx")
            .inst(LoadsAndStores::MemoryCopyAndMemorySet)
            .with("xx01")
            .with("x")
            .with("1x")
            .with("0xx_xxx")
            .with("01")
            .inst(LoadsAndStores::LoadStoreNoAllocatePairOffset)
            .with("xx10")
            .with("x")
            .with("00")
            .with("xxx_xxx")
            .with("xx")
            .inst(LoadsAndStores::LoadStoreRegisterPairPostIndexed)
            .with("xx10")
            .with("x")
            .with("01")
            .with("xxx_xxx")
            .with("xx")
            .inst(LoadsAndStores::LoadStoreRegisterPairOffset)
            .with("xx10")
            .with("x")
            .with("10")
            .with("xxx_xxx")
            .with("xx")
            .inst(LoadsAndStores::LoadStoreRegisterPairPreIndexed)
            .with("xx10")
            .with("x")
            .with("11")
            .with("xxx_xxx")
            .with("xx")
            .inst(LoadsAndStores::LoadStoreRegisterUnscalaedImmediate)
            .with("xx11")
            .with("x")
            .with("0x")
            .with("0xx_xxx")
            .with("00")
            .inst(LoadsAndStores::LoadStoreRegisterImmediatePostIndexed)
            .with("xx11")
            .with("x")
            .with("0x")
            .with("0xx_xxx")
            .with("01")
            .inst(LoadsAndStores::LoadStoreRegisterUnprivileged)
            .with("xx11")
            .with("x")
            .with("0x")
            .with("0xx_xxx")
            .with("10")
            .inst(LoadsAndStores::LoadStoreRegisterImmediatePreIndexed)
            .with("xx11")
            .with("x")
            .with("0x")
            .with("0xx_xxx")
            .with("11")
            .inst(LoadsAndStores::AtomicMemoryOperations)
            .with("xx11")
            .with("x")
            .with("0x")
            .with("1xx_xxx")
            .with("00")
            .inst(LoadsAndStores::LoadStoreRegisterRegisterOffset)
            .with("xx11")
            .with("x")
            .with("0x")
            .with("1xx_xxx")
            .with("10")
            .inst(LoadsAndStores::LoadStoreRegisterPac)
            .with("xx11")
            .with("x")
            .with("0x")
            .with("1xx_xxx")
            .with("x1")
            .inst(LoadsAndStores::LoadStoreRegisterUnsignedImmediate)
            .with("xx11")
            .with("x")
            .with("1x")
            .with("xxx_xxx")
            .with("xx")
            .build();

        let load_store_register_unsigned_immediate_pm =
            PatternMatcher::<LoadStoreRegisterUnsignedImmediate>::builder()
                .args("size", 30..32)
                .args("V", 26..27)
                .args("opc", 22..24)
                .args("imm12", 10..22)
                .args("Rn", 5..10)
                .args("Rt", 0..5)
                .inst(LoadStoreRegisterUnsignedImmediate::Unallocated0)
                .with("x1")
                .with("1")
                .with("1x")
                .inst(LoadStoreRegisterUnsignedImmediate::STRBImmediate)
                .with("00")
                .with("0")
                .with("00")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRBImmediate)
                .with("00")
                .with("0")
                .with("01")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRSBImmediate64)
                .with("00")
                .with("0")
                .with("10")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRSBImmediate32)
                .with("00")
                .with("0")
                .with("11")
                .inst(LoadStoreRegisterUnsignedImmediate::STRImmediateSIMDFP8)
                .with("00")
                .with("1")
                .with("00")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRImmediateSIMDFP8)
                .with("00")
                .with("1")
                .with("01")
                .inst(LoadStoreRegisterUnsignedImmediate::STRImmediateSIMDFP128)
                .with("00")
                .with("1")
                .with("10")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRImmediateSIMDFP128)
                .with("00")
                .with("1")
                .with("11")
                .inst(LoadStoreRegisterUnsignedImmediate::STRHImmediate)
                .with("01")
                .with("0")
                .with("00")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRHImmediate)
                .with("01")
                .with("0")
                .with("01")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRSHImmediate64)
                .with("01")
                .with("0")
                .with("10")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRSHImmediate32)
                .with("01")
                .with("0")
                .with("11")
                .inst(LoadStoreRegisterUnsignedImmediate::STRImmediateSIMDFP16)
                .with("01")
                .with("1")
                .with("00")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRImmediateSIMDFP16)
                .with("01")
                .with("1")
                .with("01")
                .inst(LoadStoreRegisterUnsignedImmediate::Unallocated1)
                .with("1x")
                .with("0")
                .with("11")
                .inst(LoadStoreRegisterUnsignedImmediate::Unallocated2)
                .with("1x")
                .with("1")
                .with("1x")
                .inst(LoadStoreRegisterUnsignedImmediate::STRImmediate32)
                .with("10")
                .with("0")
                .with("00")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRImmediate32)
                .with("10")
                .with("0")
                .with("01")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRSWImmediate)
                .with("10")
                .with("0")
                .with("10")
                .inst(LoadStoreRegisterUnsignedImmediate::STRImmediateSIMDFP32)
                .with("10")
                .with("1")
                .with("00")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRImmediateSIMDFP32)
                .with("10")
                .with("1")
                .with("01")
                .inst(LoadStoreRegisterUnsignedImmediate::STRImmediate64)
                .with("11")
                .with("0")
                .with("00")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRImmediate64)
                .with("11")
                .with("0")
                .with("01")
                .inst(LoadStoreRegisterUnsignedImmediate::PRFMImmediate)
                .with("11")
                .with("0")
                .with("10")
                .inst(LoadStoreRegisterUnsignedImmediate::STRImmediateSIMDFP64)
                .with("11")
                .with("1")
                .with("00")
                .inst(LoadStoreRegisterUnsignedImmediate::LDRImmediateSIMDFP64)
                .with("11")
                .with("1")
                .with("01")
                .build();
        let data_processing_register_pm = PatternMatcher::<DataProcessingRegister>::builder()
            .args("op0", 30..31)
            .args("op1", 28..29)
            .args("op2", 21..25)
            .args("op3", 10..16)
            .inst(DataProcessingRegister::DataProcessing2Source)
            .with("0")
            .with("1")
            .with("0110")
            .with("xxx_xxx")
            .inst(DataProcessingRegister::DataProcessing1Source)
            .with("1")
            .with("1")
            .with("0110")
            .with("xxx_xxx")
            .inst(DataProcessingRegister::LogicalShiftedRegister)
            .with("x")
            .with("0")
            .with("0xxx")
            .with("xxx_xxx")
            .inst(DataProcessingRegister::AddSubtractShiftedRegister)
            .with("x")
            .with("0")
            .with("1xx0")
            .with("xxx_xxx")
            .inst(DataProcessingRegister::AddSubtractExtendedRegister)
            .with("x")
            .with("0")
            .with("1xx1")
            .with("xxx_xxx")
            .inst(DataProcessingRegister::AddSubtractWithCarry)
            .with("x")
            .with("1")
            .with("0000")
            .with("000_000")
            .inst(DataProcessingRegister::RotateRightIntoFlags)
            .with("x")
            .with("1")
            .with("0000")
            .with("x00_001")
            .inst(DataProcessingRegister::EvaluateInfoFlags)
            .with("x")
            .with("1")
            .with("0000")
            .with("xx0_010")
            .inst(DataProcessingRegister::ConditionalCompareRegister)
            .with("x")
            .with("1")
            .with("0010")
            .with("xxx_x0x")
            .inst(DataProcessingRegister::ConditionalCompareImmediate)
            .with("x")
            .with("1")
            .with("0010")
            .with("xxx_x1x")
            .inst(DataProcessingRegister::ConditionalSelect)
            .with("x")
            .with("1")
            .with("0100")
            .with("xxx_xxx")
            .inst(DataProcessingRegister::ConditionalSelect)
            .with("x")
            .with("1")
            .with("1xxx")
            .with("xxx_xxx")
            .build();
        let add_subtract_shifted_register_pm =
            PatternMatcher::<AddSubtractShiftedRegister>::builder()
                .args("sf", 31..32)
                .args("op", 30..31)
                .args("S", 29..30)
                .args("shift", 22..24)
                .args("imm6", 10..16)
                .args("Rm", 16..21)
                .args("Rn", 5..10)
                .args("Rd", 0..5)
                .inst(AddSubtractShiftedRegister::Unallocated0)
                .with("x")
                .with("x")
                .with("x")
                .with("11")
                .with("xxx_xxx")
                .inst(AddSubtractShiftedRegister::Unallocated1)
                .with("0")
                .with("x")
                .with("x")
                .with("xx")
                .with("1xx_xxx")
                .inst(AddSubtractShiftedRegister::ADDShiftedRegister32)
                .with("0")
                .with("0")
                .with("0")
                .with("xx")
                .with("xxx_xxx")
                .inst(AddSubtractShiftedRegister::ADDSShiftedRegister64)
                .with("0")
                .with("0")
                .with("1")
                .with("xx")
                .with("xxx_xxx")
                .inst(AddSubtractShiftedRegister::SUBShiftedRegister32)
                .with("0")
                .with("1")
                .with("0")
                .with("xx")
                .with("xxx_xxx")
                .inst(AddSubtractShiftedRegister::SUBSShiftedRegister32)
                .with("0")
                .with("1")
                .with("1")
                .with("xx")
                .with("xxx_xxx")
                .inst(AddSubtractShiftedRegister::ADDShiftedRegister64)
                .with("1")
                .with("0")
                .with("0")
                .with("xx")
                .with("xxx_xxx")
                .inst(AddSubtractShiftedRegister::ADDSShiftedRegister64)
                .with("1")
                .with("0")
                .with("1")
                .with("xx")
                .with("xxx_xxx")
                .inst(AddSubtractShiftedRegister::SUBShiftedRegister64)
                .with("1")
                .with("1")
                .with("0")
                .with("xx")
                .with("xxx_xxx")
                .inst(AddSubtractShiftedRegister::SUBSShiftedRegister64)
                .with("1")
                .with("1")
                .with("1")
                .with("xx")
                .with("xxx_xxx")
                .build();

        Self {
            main_encoding_pm,

            data_processing_immediate_pm,
            add_subtract_immediate_pm,

            brnch_xcept_gen_sys_instr_pm,
            conditional_branch_immediate_pm,

            loads_and_stores_pm,
            load_store_register_unsigned_immediate_pm,

            data_processing_register_pm,
            add_subtract_shifted_register_pm,
        }
    }

    pub fn parse(&self, instr: u32) -> AArch64Inst {
        match self.main_encoding_pm.match_pattern(instr) {
            Some(pat) => match pat {
                MainEncodingTable::Reserved => todo!(),
                MainEncodingTable::SmeEncodings => todo!(),
                MainEncodingTable::Unallocated0 => todo!(),
                MainEncodingTable::SveEncodings => todo!(),
                MainEncodingTable::Unallocated1 => todo!(),
                MainEncodingTable::DataProcessingImmediate => {
                    self.parse_data_processing_immediate(instr)
                }
                MainEncodingTable::BranchesExceptionGenNSysInstr => {
                    self.parse_brnch_xcept_gen_sys_instr(instr)
                }
                MainEncodingTable::LoadsAndStores => self.parse_load_and_stores(instr),
                MainEncodingTable::DataProcessingRegister => {
                    self.parse_data_processing_register(instr)
                }
                MainEncodingTable::DataProcessingScalarFloatingPointAndAdvancedSIMD => todo!(),
            },
            None => todo!(),
        }
    }
    //==============================Data processing Immediate==============================
    fn parse_data_processing_immediate(&self, instr: u32) -> AArch64Inst {
        match self.data_processing_immediate_pm.match_pattern(instr) {
            Some(pat) => match pat {
                DataProcessingImmediate::PCrelAddressing => todo!(),
                DataProcessingImmediate::AddSubtractImmediate => {
                    self.parse_add_subtract_immediate(instr)
                }
                DataProcessingImmediate::AddSubtractImmediateWithTags => todo!(),
                DataProcessingImmediate::LogicalImmediate => todo!(),
                DataProcessingImmediate::MoveWideImmediate => todo!(),
                DataProcessingImmediate::Bitfield => todo!(),
                DataProcessingImmediate::Extract => todo!(),
            },
            None => todo!(),
        }
    }

    fn parse_add_subtract_immediate(&self, instr: u32) -> AArch64Inst {
        let data = AddSubtractImmediateData {
            sf: self.add_subtract_immediate_pm.get_arg(instr, 0),
            op: self.add_subtract_immediate_pm.get_arg(instr, 1),
            s: self.add_subtract_immediate_pm.get_arg(instr, 2),
            sh: self.add_subtract_immediate_pm.get_arg(instr, 3),
            imm12: self.add_subtract_immediate_pm.get_arg(instr, 4),
            rn: self.add_subtract_immediate_pm.get_arg(instr, 5),
            rd: self.add_subtract_immediate_pm.get_arg(instr, 6),
        };

        match self.add_subtract_immediate_pm.match_pattern(instr) {
            Some(pat) => match pat {
                AddSubtractImmediate::ADDImmediate32 => todo!(),
                AddSubtractImmediate::ADDSImmediate32 => todo!(),
                AddSubtractImmediate::SUBImmediate32 => todo!(),
                AddSubtractImmediate::SUBSImmediate32 => todo!(),
                AddSubtractImmediate::ADDImmediate64 => todo!(),
                AddSubtractImmediate::ADDSImmediate64 => todo!(),
                AddSubtractImmediate::SUBImmediate64 => AArch64Inst::SubImmediate64(data),
                AddSubtractImmediate::SUBSImmediate64 => todo!(),
            },
            None => todo!(),
        }
    }
    //==============================Branches, Exception Generating and System instructions==============================
    fn parse_brnch_xcept_gen_sys_instr(&self, instr: u32) -> AArch64Inst {
        match self.brnch_xcept_gen_sys_instr_pm.match_pattern(instr) {
            Some(pat) => match pat {
                BranchesExceptionGenNSysInstr::ConditionalBranchImmediate => self.parse_conditional_branch_immediate(instr),
                BranchesExceptionGenNSysInstr::ExceptionGeneration => todo!(),
                BranchesExceptionGenNSysInstr::SystemInstructionsWithRegisterArgument => todo!(),
                BranchesExceptionGenNSysInstr::Hints => todo!(),
                BranchesExceptionGenNSysInstr::Barriers => todo!(),
                BranchesExceptionGenNSysInstr::PSTATE => todo!(),
                BranchesExceptionGenNSysInstr::SystemWithResult => todo!(),
                BranchesExceptionGenNSysInstr::SystemInstructions => todo!(),
                BranchesExceptionGenNSysInstr::SystemRegisterMove => todo!(),
                BranchesExceptionGenNSysInstr::UnconditionalBranchRegister => todo!(),
                BranchesExceptionGenNSysInstr::UnconditionalBranchImmediate => todo!(),
                BranchesExceptionGenNSysInstr::CompareAndBranchImmediate => todo!(),
                BranchesExceptionGenNSysInstr::TestAndBranchImmediate => todo!(),
            },
            None => todo!(),
        }
    }

    fn parse_conditional_branch_immediate(&self, instr: u32) -> AArch64Inst {
        let data = ConditionalBranchImmediateData {
            o1: self.conditional_branch_immediate_pm.get_arg(instr, 0),
            o0: self.conditional_branch_immediate_pm.get_arg(instr, 1),
            imm19: self.conditional_branch_immediate_pm.get_arg(instr, 2),
            cond: self.conditional_branch_immediate_pm.get_arg(instr, 3),
        };

        match self.conditional_branch_immediate_pm.match_pattern(instr) {
            Some(pat) => match pat {
                ConditionalBranchImmediate::BCond => AArch64Inst::BCond(data),
                ConditionalBranchImmediate::BcCond => todo!(),
                ConditionalBranchImmediate::Unallocated => todo!(),
            },
            None => todo!(),
        }
    }
    //==============================Load and Stores==============================
    fn parse_load_and_stores(&self, instr: u32) -> AArch64Inst {
        match self.loads_and_stores_pm.match_pattern(instr) {
            Some(pat) => match pat {
                LoadsAndStores::CompareAndSwapPair => todo!(),
                LoadsAndStores::AdvancedSIMDLoadStoreMultipleStructures => todo!(),
                LoadsAndStores::AdvancedSIMDLoadStoreMultipleStructuresPostIndexed => todo!(),
                LoadsAndStores::Unallocated0 => todo!(),
                LoadsAndStores::AdvancedSIMDLoadStoreSingleStructure => todo!(),
                LoadsAndStores::AdvancedSIMDLoadStoreSingleStructurePostIndexed => todo!(),
                LoadsAndStores::Unallocated1 => todo!(),
                LoadsAndStores::Unallocated2 => todo!(),
                LoadsAndStores::Unallocated3 => todo!(),
                LoadsAndStores::Unallocated4 => todo!(),
                LoadsAndStores::Unallocated5 => todo!(),
                LoadsAndStores::LoadStoreMemoryTags => todo!(),
                LoadsAndStores::LoadStoreExclusivePair => todo!(),
                LoadsAndStores::Unallocated6 => todo!(),
                LoadsAndStores::LoadStoreExclusiveRegister => todo!(),
                LoadsAndStores::LoadStoreOrdered => todo!(),
                LoadsAndStores::CompareAndSwap => todo!(),
                LoadsAndStores::LDAPRSTLRUnscalaedImmediate => todo!(),
                LoadsAndStores::LoadRegisterLiteral => todo!(),
                LoadsAndStores::MemoryCopyAndMemorySet => todo!(),
                LoadsAndStores::LoadStoreNoAllocatePairOffset => todo!(),
                LoadsAndStores::LoadStoreRegisterPairPostIndexed => todo!(),
                LoadsAndStores::LoadStoreRegisterPairOffset => todo!(),
                LoadsAndStores::LoadStoreRegisterPairPreIndexed => todo!(),
                LoadsAndStores::LoadStoreRegisterUnscalaedImmediate => todo!(),
                LoadsAndStores::LoadStoreRegisterImmediatePostIndexed => todo!(),
                LoadsAndStores::LoadStoreRegisterUnprivileged => todo!(),
                LoadsAndStores::LoadStoreRegisterImmediatePreIndexed => todo!(),
                LoadsAndStores::AtomicMemoryOperations => todo!(),
                LoadsAndStores::LoadStoreRegisterRegisterOffset => todo!(),
                LoadsAndStores::LoadStoreRegisterPac => todo!(),
                LoadsAndStores::LoadStoreRegisterUnsignedImmediate => {
                    self.parse_load_store_register_unsigned_immediate(instr)
                }
            },
            None => todo!(),
        }
    }

    fn parse_load_store_register_unsigned_immediate(&self, instr: u32) -> AArch64Inst {
        let data = LoadStoreRegisterUnsignedImmediateData {
            size: self.add_subtract_immediate_pm.get_arg(instr, 0),
            v: self.add_subtract_immediate_pm.get_arg(instr, 1),
            opc: self.add_subtract_immediate_pm.get_arg(instr, 2),
            imm12: self.add_subtract_immediate_pm.get_arg(instr, 3),
            rn: self.add_subtract_immediate_pm.get_arg(instr, 4),
            rt: self.add_subtract_immediate_pm.get_arg(instr, 5),
        };

        match self
            .load_store_register_unsigned_immediate_pm
            .match_pattern(instr)
        {
            Some(pat) => match pat {
                LoadStoreRegisterUnsignedImmediate::Unallocated0 => todo!(),
                LoadStoreRegisterUnsignedImmediate::STRBImmediate => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRBImmediate => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRSBImmediate64 => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRSBImmediate32 => todo!(),
                LoadStoreRegisterUnsignedImmediate::STRImmediateSIMDFP8 => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRImmediateSIMDFP8 => todo!(),
                LoadStoreRegisterUnsignedImmediate::STRImmediateSIMDFP128 => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRImmediateSIMDFP128 => todo!(),
                LoadStoreRegisterUnsignedImmediate::STRHImmediate => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRHImmediate => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRSHImmediate64 => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRSHImmediate32 => todo!(),
                LoadStoreRegisterUnsignedImmediate::STRImmediateSIMDFP16 => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRImmediateSIMDFP16 => todo!(),
                LoadStoreRegisterUnsignedImmediate::Unallocated1 => todo!(),
                LoadStoreRegisterUnsignedImmediate::Unallocated2 => todo!(),
                LoadStoreRegisterUnsignedImmediate::STRImmediate32 => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRImmediate32 => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRSWImmediate => todo!(),
                LoadStoreRegisterUnsignedImmediate::STRImmediateSIMDFP32 => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRImmediateSIMDFP32 => todo!(),
                LoadStoreRegisterUnsignedImmediate::STRImmediate64 => {
                    AArch64Inst::STRImmediate64(data)
                }
                LoadStoreRegisterUnsignedImmediate::LDRImmediate64 => todo!(),
                LoadStoreRegisterUnsignedImmediate::PRFMImmediate => todo!(),
                LoadStoreRegisterUnsignedImmediate::STRImmediateSIMDFP64 => todo!(),
                LoadStoreRegisterUnsignedImmediate::LDRImmediateSIMDFP64 => todo!(),
            },
            None => todo!(),
        }
    }

    //==============================Data processing Register==============================
    fn parse_data_processing_register(&self, instr: u32) -> AArch64Inst {
        match self.data_processing_register_pm.match_pattern(instr) {
            Some(pat) => match pat {
                DataProcessingRegister::DataProcessing2Source => todo!(),
                DataProcessingRegister::DataProcessing1Source => todo!(),
                DataProcessingRegister::LogicalShiftedRegister => todo!(),
                DataProcessingRegister::AddSubtractShiftedRegister => {
                    self.parse_add_subtract_shifted_register(instr)
                }
                DataProcessingRegister::AddSubtractExtendedRegister => todo!(),
                DataProcessingRegister::AddSubtractWithCarry => todo!(),
                DataProcessingRegister::RotateRightIntoFlags => todo!(),
                DataProcessingRegister::EvaluateInfoFlags => todo!(),
                DataProcessingRegister::ConditionalCompareRegister => todo!(),
                DataProcessingRegister::ConditionalCompareImmediate => todo!(),
                DataProcessingRegister::ConditionalSelect => todo!(),
                DataProcessingRegister::DataProcessing3Source => todo!(),
            },
            None => todo!(),
        }
    }

    fn parse_add_subtract_shifted_register(&self, instr: u32) -> AArch64Inst {
        let data = AddSubtractShiftedRegisterData {
            sf: self.add_subtract_shifted_register_pm.get_arg(instr, 0),
            op: self.add_subtract_shifted_register_pm.get_arg(instr, 1),
            s: self.add_subtract_shifted_register_pm.get_arg(instr, 2),
            shift: self.add_subtract_shifted_register_pm.get_arg(instr, 3),
            imm6: self.add_subtract_shifted_register_pm.get_arg(instr, 4),
            rm: self.add_subtract_shifted_register_pm.get_arg(instr, 5),
            rn: self.add_subtract_shifted_register_pm.get_arg(instr, 6),
            rd: self.add_subtract_shifted_register_pm.get_arg(instr, 7),
        };

        match self.add_subtract_shifted_register_pm.match_pattern(instr) {
            Some(pat) => match pat {
                AddSubtractShiftedRegister::Unallocated0 => todo!(),
                AddSubtractShiftedRegister::Unallocated1 => todo!(),
                AddSubtractShiftedRegister::ADDShiftedRegister32 => todo!(),
                AddSubtractShiftedRegister::ADDSShiftedReigster32 => todo!(),
                AddSubtractShiftedRegister::SUBShiftedRegister32 => todo!(),
                AddSubtractShiftedRegister::SUBSShiftedRegister32 => todo!(),
                AddSubtractShiftedRegister::ADDShiftedRegister64 => todo!(),
                AddSubtractShiftedRegister::ADDSShiftedRegister64 => todo!(),
                AddSubtractShiftedRegister::SUBShiftedRegister64 => todo!(),
                AddSubtractShiftedRegister::SUBSShiftedRegister64 => {
                    AArch64Inst::SUBSShiftedRegister64(data)
                }
            },
            None => todo!(),
        }
    }
}
