use core::operation::shift_c;
use core::operation::decode_imm_shift;
use core::operation::add_with_carry;
use core::operation::condition_passed;
use core::operation::SRType;
use bit_field::BitField;
use core::instruction::Instruction;
use core::register::Reg;
use core::register::Apsr;
use core::Core;
use bus::Bus;

fn read_reg<T: Bus>(core: &mut Core<T>, r: Reg) -> u32 {
    match r {
        Reg::PC => core.r[r.value()] + 4,
        _ => core.r[r.value()],
    }
}

pub fn execute<T: Bus>(core: &mut Core<T>, op: Option<Instruction>) {
    match op {
        None => panic!("undefined code"),
        Some(oper) => {
            print!("{} ", oper);
            match oper {
                Instruction::MOV_reg { rd, rm, setflags } => {
                    let result = read_reg(core, rm);
                    core.r[rd.value() as usize] = result;

                    if rd != Reg::PC {
                        if setflags {
                            core.psr.set_n(result.get_bit(31));
                            core.psr.set_z(result == 0);
                        }
                        core.r[Reg::PC.value()] += 2;
                    }
                }
                Instruction::LSL_imm { rd, rm, imm5, setflags } => {
                    let (shift_t, shift_n) = decode_imm_shift(0b00, imm5);
                    let (result, carry) = shift_c(read_reg(core, rm),
                                                  SRType::LSL,
                                                  u32::from(shift_n),
                                                  core.psr.get_c());
                    core.r[rd.value() as usize] = result;

                    if setflags {
                        core.psr.set_n(result.get_bit(31));
                        core.psr.set_z(result == 0);
                        core.psr.set_c(carry);
                    }

                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::BL { imm32 } => {
                    let pc = read_reg(core, Reg::PC);
                    core.r[Reg::LR.value()] = pc | 0x01;
                    core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
                }
                Instruction::BKPT { imm32 } => {
                    core.bkpt(imm32);
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::BX { rm } => {
                    core.r[Reg::PC.value()] = read_reg(core, rm) & 0xffff_fffe;
                }
                Instruction::BLX { rm } => {
                    let pc = read_reg(core, Reg::PC);
                    core.r[Reg::LR.value()] = (((pc - 2) >> 1) << 1) | 1;
                    core.r[Reg::PC.value()] = read_reg(core, rm) & 0xffff_fffe;
                }
                Instruction::MOV_imm { rd, imm32, setflags } => {
                    let result = imm32 as u32;
                    core.r[rd.value()] = result;
                    if setflags {
                        core.psr.set_n(result.get_bit(31));
                        core.psr.set_z(result == 0);
                    }
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::MVN_reg { rd, rm, setflags } => {
                    let result = read_reg(core, rm) ^ 0xFFFF_FFFF;
                    core.r[rd.value()] = result;

                    if setflags {
                        core.psr.set_n(result.get_bit(31));
                        core.psr.set_z(result == 0);
                    }
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::B { cond, imm32 } => {
                    if condition_passed(cond, &core.psr) {
                        let pc = read_reg(core, Reg::PC);
                        core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
                    } else {
                        core.r[Reg::PC.value()] += 2;
                    }
                }

                Instruction::CMP_imm { rn, imm32 } => {
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), imm32 ^ 0xFFFF_FFFF, true);
                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::CMP { rn, rm } => {
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), read_reg(core, rm) ^ 0xFFFF_FFFF, true);
                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);
                    core.r[Reg::PC.value()] += 2;
                }

                Instruction::PUSH { registers } => {
                    let regs_size = 4 * (registers.len() as u32);
                    let sp = core.get_sp();
                    let mut address = sp - regs_size;

                    for reg in registers.iter() {
                        core.bus.write32(address, core.r[reg.value()]);
                        address += 4;
                    }

                    core.set_sp(sp - regs_size);
                    core.r[Reg::PC.value()] += 2;
                }

                Instruction::POP { registers } => {
                    let regs_size = 4 * (registers.len() as u32);
                    let sp = core.get_sp();
                    let mut address = sp;

                    for reg in registers.iter() {
                        if reg == Reg::PC {
                            core.r[reg.value()] = core.bus.read32(address) & 0xffff_fffe;
                        } else {
                            core.r[reg.value()] = core.bus.read32(address);
                        }
                        address += 4;
                    }

                    core.set_sp(sp + regs_size);
                    if !registers.contains(&Reg::PC) {
                        core.r[Reg::PC.value()] += 2;
                    }
                }

                Instruction::LDR_imm { rt, rn, imm32 } => {
                    let address = read_reg(core, rn) + imm32;
                    core.r[rt.value()] = core.bus.read32(address & 0xffff_fffc);
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::LDRB_imm { rt, rn, imm32 } => {
                    let address = read_reg(core, rn) + imm32;
                    core.r[rt.value()] = u32::from(core.bus.read8(address));
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::STR_imm { rt, rn, imm32 } => {
                    let address = (read_reg(core, rn) + imm32) & 0xffff_fffc;
                    let value = read_reg(core, rt);
                    core.bus.write32(address, value);
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::STRB_imm { rt, rn, imm32 } => {
                    let address = (read_reg(core, rn) + imm32) & 0xffff_fffc;
                    let value = read_reg(core, rt);
                    core.bus.write8(address, value.get_bits(0..9) as u8);
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::LDR_lit { rt, imm32 } => {
                    let base = read_reg(core, Reg::PC) & 0xffff_fffc;
                    core.r[rt.value()] = core.bus.read32(base + imm32);
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::ADD { rdn, rm } => {
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rdn), read_reg(core, rm), false);
                    core.r[rdn.value()] = result;
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::ADD_imm { rn, rd, imm32, setflags } => {
                    let r_n = read_reg(core, rn);
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), imm32, false);

                    if setflags {
                        core.psr.set_n(result.get_bit(31));
                        core.psr.set_z(result == 0);
                        core.psr.set_c(carry);
                        core.psr.set_v(overflow);
                    }

                    core.r[rd.value()] = result;
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::ADR { rd, imm32 } => {
                    let result = (read_reg(core, Reg::PC) & 0xffff_fffc) + imm32;
                    core.r[rd.value()] = result;
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::SUB_imm { rn, rd, imm32, setflags } => {
                    let r_n = read_reg(core, rn);
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), imm32 ^ 0xFFFF_FFFF, true);

                    if setflags {
                        core.psr.set_n(result.get_bit(31));
                        core.psr.set_z(result == 0);
                        core.psr.set_c(carry);
                        core.psr.set_v(overflow);
                    }

                    core.r[rd.value()] = result;
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::SUBS_reg { rn, rd, rm } => {
                    let r_n = read_reg(core, rn);
                    let r_m = read_reg(core, rm);
                    let (result, carry, overflow) = add_with_carry(r_n, r_m ^ 0xFFFF_FFFF, true);
                    core.r[rd.value()] = result;

                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);

                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::ADDS { rm, rn, rd } => {
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), read_reg(core, rm), false);
                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);

                    core.r[rd.value()] = result;
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::TST_reg { rn, rm } => {
                    let result = read_reg(core, rn) & read_reg(core, rm);

                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    //core.psr.set_c(carry); carry = shift_c()
                    core.r[Reg::PC.value()] += 2;
                }

                _ => panic!("unimplemented instruction {}", oper),
            }
        }
    }
}
