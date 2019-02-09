use crate::core::bits::*;
use crate::core::instruction::Instruction;
use crate::core::operation::zero_extend;
use crate::core::register::Reg;

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_ADR_t1(command: u16) -> Instruction {
    Instruction::ADR {
        rd: From::from(bits_8_11(command)),
        imm32: u32::from(bits_0_8(command)) << 2,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_ADR_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
    }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_ADR_t3(opcode: u32) -> Instruction {
    let i: u8 = opcode.get_bit(26) as u8;
    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let imm8: u8 = opcode.get_bits(0..8) as u8;

    let params = [i, imm3, imm8];
    let lengths = [1, 3, 8];
    Instruction::ADR {
        rd: Reg::from(rd),
        imm32: zero_extend(&params, &lengths),
        thumb32: true,
    }
}
