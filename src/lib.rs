use std::convert::TryInto;

mod aarc64_parser;
mod error;
mod instructions;
mod pattern_matcher;
mod utils;

use instructions::data_processing_immediate::*;
use instructions::brnch_xcept_gen_sys_instr::*;
use instructions::data_processing_register::*;
use instructions::loads_and_stores::*;

use crate::aarc64_parser::AArch64Parser;
use crate::error::Error;


#[derive(Debug)]
pub enum AArch64Inst {
    SubImmediate64(AddSubtractImmediateData),

    BCond(ConditionalBranchImmediateData),

    STRImmediate64(LoadStoreRegisterUnsignedImmediateData),

    SUBSShiftedRegister64(AddSubtractShiftedRegisterData),

    UNDEFINED,
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::utils::InstReader;

    use super::*;
    use elf::endian::AnyEndian;
    use elf::section::SectionHeader;
    use elf::ElfBytes;

    #[test]
    fn it_works() {
        let path = std::path::PathBuf::from("a64_example");
        let file_data = std::fs::read(path).unwrap();

        let slice = file_data.as_slice();
        let file = ElfBytes::<AnyEndian>::minimal_parse(slice).unwrap();

        let text_section = file
            .section_header_by_name(".text")
            .expect("section table should be parseable")
            .expect("file should have a .text section");

        let ep_offset = text_section.sh_offset as usize;
        let ep_size = text_section.sh_size as usize;

        let mut inst_reader =
            InstReader::new(file_data[ep_offset..(ep_offset + ep_size)].iter().cloned());

        let parser = AArch64Parser::new();

        while let Some(instr) = inst_reader.next() {
            let result = parser.parse(instr);

            println!("{:#?}", result);
        }
    }
}
