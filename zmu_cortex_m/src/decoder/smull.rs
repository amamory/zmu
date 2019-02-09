use crate::core::instruction::Instruction;
use crate::core::bits::Bits;

#[allow(non_snake_case)]
pub fn decode_SMULL_t1(opcode: u32) -> Instruction {
    Instruction::SMULL {
        rm: opcode.get_bits(0..4).into(),
        rdlo: opcode.get_bits(12..16).into(),
        rdhi: opcode.get_bits(8..12).into(),
        rn: opcode.get_bits(16..20).into(),
    }
}
