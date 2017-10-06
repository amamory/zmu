use core::register::Reg;
use core::condition::Condition;
use enum_set::EnumSet;
use std::fmt;


#[allow(non_camel_case_types)]
pub enum Instruction {
    ADC,
    ADDS { rm: Reg, rn: Reg, rd: Reg },
    ADD { rm: Reg, rdn: Reg },
    ADDS_imm { rn: Reg, rd: Reg, imm32: u32 },
    ADR,
    AND,
    ASR,
    B { cond: Condition, imm32: i32 },
    BIC,
    BKPT,
    BL { imm32: i32 },
    BLX { rm: Reg },
    BX { rm: Reg },
    CMN,
    CMP_imm { rn: Reg, imm32: u32 },
    CMP { rm: Reg, rn: Reg },
    CPS,
    CPY,
    DMB,
    DSB,
    EOR,
    ISB,
    LDM,
    LDMIA,
    LDMFD,
    LDR_imm { rt: Reg, rn: Reg, imm32: u32 },
    LDR_lit { rt: Reg, imm32: u32 },
    LDR_reg { rt: Reg, rn: Reg, rm: Reg },
    LDRB_imm,
    LDRB_reg,
    LDRH_imm,
    LDRH_reg,
    LDRSB_reg,
    LDRSH_reg,
    LSL_imm,
    LSL_reg,
    LSR_imm,
    LSR_reg,
    MOV_reg { rd: Reg, rm: Reg, setflags: bool },
    MOV_imm { rd: Reg, imm32: u32 },
    MRS,
    MRS_reg,
    MUL,
    MVN_reg,
    NOP,
    ORR,
    POP { registers: EnumSet<Reg> },
    PUSH { registers: EnumSet<Reg> },
    REV,
    REV16,
    REVSH,
    ROR,
    RSB,
    SBC,
    SEV,
    STM,
    STMIA,
    STMEA,
    STR_imm { rn: Reg, rt: Reg, imm32: u32 },
    STR_reg { rm: Reg, rn: Reg, rt: Reg },
    STRB_imm,
    STRB_reg,
    STRH_imm,
    STRH_reg,
    SUBS_imm { rd: Reg, rn: Reg, imm32: u32 },
    SUBS_reg { rm: Reg, rn: Reg, rd: Reg },
    SUB_SP_imm,
    SVC,
    SXTB,
    SXTH,
    TST_reg { rn: Reg, rm: Reg },
    UDF,
    UXTB,
    UXTH,
    WFE,
    WFI,
    YIELD,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::ADD { rdn, rm } => write!(f, "ADD"),
            Instruction::ADDS_imm { rn, rd, imm32 } => write!(f, "ADDS {},{},#{}", rn, rd, imm32),
            Instruction::ADDS { rm, rn, rd } => write!(f, "ADDS {},{},{}", rn, rd, rm),
            Instruction::ADC => write!(f, "ADC"),
            Instruction::ADR => write!(f, "ADR"),
            Instruction::AND => write!(f, "AND"),
            Instruction::ASR => write!(f, "ASR"),
            Instruction::B { ref cond, imm32 } => write!(f, "B{} {}", cond, imm32),
            Instruction::BIC => write!(f, "BIC"),
            Instruction::BL { imm32 } => write!(f, "BL"),
            Instruction::BX { rm } => write!(f, "BX"),
            Instruction::BLX { rm } => write!(f, "BLX {}", rm),
            Instruction::BKPT => write!(f, "BKPT"),
            Instruction::CMN => write!(f, "CMN"),
            Instruction::CMP_imm { rn, imm32 } => write!(f, "CMP {}, #{}", rn, imm32),
            Instruction::CMP { rn, rm } => write!(f, "CMP {}, {}", rn, rm),
            Instruction::CPS => write!(f, "CPS"),
            Instruction::CPY => write!(f, "CPY"),
            Instruction::DMB => write!(f, "DMB"),
            Instruction::DSB => write!(f, "DSB"),
            Instruction::EOR => write!(f, "EOR"),
            Instruction::ISB => write!(f, "ISB"),
            Instruction::LDM => write!(f, "LDM"),
            Instruction::LDMIA => write!(f, "LDMIA"),
            Instruction::LDMFD => write!(f, "LDMFD"),
            Instruction::LDR_reg { rt, rn, rm } => write!(f, "LDR regs"),
            Instruction::LDR_imm { rt, rn, imm32 } => write!(f, "LDR {},{},#{}", rt, rn, imm32),
            Instruction::LDR_lit { rt, imm32 } => write!(f, "LDR {},[PC, #{}]", rt, imm32),
            Instruction::LDRB_imm => write!(f, "LDRB"),
            Instruction::LDRB_reg => write!(f, "LDRB reg"),
            Instruction::LDRH_imm => write!(f, "LDRH imm"),
            Instruction::LDRSB_reg => write!(f, "LDRSB reg"),
            Instruction::LDRSH_reg => write!(f, "LDRSH reg"),
            Instruction::LSL_imm => write!(f, "LSL imm"),
            Instruction::LDRH_reg => write!(f, "LDRH reg"),
            Instruction::LSL_reg => write!(f, "LSL reg"),
            Instruction::LSR_imm => write!(f, "LSR imm"),
            Instruction::LSR_reg => write!(f, "LSR reg"),
            Instruction::MRS_reg => write!(f, "MSR reg"),
            Instruction::MRS => write!(f, "MSR"),
            Instruction::MUL => write!(f, "MUL"),
            Instruction::MOV_reg { rd, rm, setflags } => write!(f, "MOV {},{}", rd, rm),
            Instruction::MOV_imm { rd, imm32 } => write!(f, "MOV {},#{}", rd, imm32),
            Instruction::MVN_reg => write!(f, "MVN"),
            Instruction::NOP => write!(f, "NOP"),
            Instruction::ORR => write!(f, "ORR"),
            Instruction::POP { registers } => write!(f, "POP"),
            Instruction::PUSH { registers } => write!(f, "PUSH"),
            Instruction::REV => write!(f, "REV"),
            Instruction::REV16 => write!(f, "REV16"),
            Instruction::REVSH => write!(f, "REVSH"),
            Instruction::ROR => write!(f, "ROR"),
            Instruction::RSB => write!(f, "RSB"),
            Instruction::SBC => write!(f, "SBC"),
            Instruction::SEV => write!(f, "SEV"),
            Instruction::STM => write!(f, "STM"),
            Instruction::STMIA => write!(f, "STMIA"),
            Instruction::STMEA => write!(f, "STMEA"),
            Instruction::STR_imm { rn, rt, imm32 } => write!(f, "STR {}, [{}, #{}]", rt, rn, imm32),            
            Instruction::STR_reg { rn, rm, rt } => write!(f, "STR {}, [{}, {}]", rt, rn, rm),
            Instruction::STRB_imm => write!(f, "STRB"),
            Instruction::STRB_reg => write!(f, "STRB_reg"),
            Instruction::STRH_imm => write!(f, "STRH_imm"),
            Instruction::STRH_reg => write!(f, "STRH_reg"),
            Instruction::SUBS_imm { rd, rn, imm32 } => write!(f, "SUBS_imm"),
            Instruction::SUBS_reg { rm, rn, rd } => write!(f, "SUBS_reg"),
            Instruction::SUB_SP_imm => write!(f, "SUB_SP_imm"),
            Instruction::SVC => write!(f, "SVC"),
            Instruction::SXTB => write!(f, "SXTB"),
            Instruction::SXTH => write!(f, "SXTH"),
            Instruction::TST_reg { rn, rm } => write!(f, "TST {},{}", rn, rm),
            Instruction::UDF => write!(f, "UDF"),
            Instruction::UXTB => write!(f, "UXTB"),
            Instruction::UXTH => write!(f, "UXTH"),
            Instruction::WFE => write!(f, "WFE"),
            Instruction::WFI => write!(f, "WFI"),
            Instruction::YIELD => write!(f, "YIELD"),

        }
    }
}