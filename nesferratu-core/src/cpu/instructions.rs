pub mod addressing;
pub mod ops;

use crate::cpu::{CpuState};
use crate::BusMessage;

pub struct Instruction {
    pub cycles: usize,
    pub bytes: usize,
    pub addr_delegate: AddrDelegate,
    pub op_delegate: OpDelegate,
    pub mnemonic: &'static str,
    pub addressing: &'static str,
}

pub enum Operand {
    Implied,
    Immediate(u8),
    Address(u16),
}

pub enum OpDelegate {
    Implied(OpDelegateImplied),
    Immediate(OpDelegateImmediate),
    Address(OpDelegateAddress),
}

impl OpDelegate {
    pub fn implied(&self) -> Option<&OpDelegateImplied> {
        if let Self::Implied(delegate) = self {
            Some(&delegate)
        } else {
            None
        }
    }

    pub fn immediate(&self) -> Option<&OpDelegateImmediate> {
        if let Self::Immediate(delegate) = self {
            Some(&delegate)
        } else {
            None
        }
    }

    pub fn address(&self) -> Option<&OpDelegateAddress> {
        if let Self::Address(delegate) = self {
            Some(&delegate)
        } else {
            None
        }
    }
}

pub enum AddrDelegateReturn {
    Yield(BusMessage),
    Return(Operand),
}

pub type AddrDelegate = fn(&mut CpuState, usize) -> AddrDelegateReturn;
pub type OpDelegateImplied = fn(&mut CpuState, usize) -> BusMessage;
pub type OpDelegateImmediate = fn(&mut CpuState, u8, usize) -> BusMessage;
pub type OpDelegateAddress = fn(&mut CpuState, u16, usize) -> BusMessage;

pub static RESET_INSTRUCTION: Instruction = Instruction {
    cycles: 8,
    bytes: 0,
    addr_delegate: addressing::imp, // addressing should be skipped altogether
    op_delegate: OpDelegate::Address(ops::reset),
    mnemonic: "RESET",
    addressing: "Reset Vector",
};

pub static IRQ_INSTRUCTION: Instruction = Instruction {
    cycles: 8,
    bytes: 0,
    addr_delegate: addressing::imp, // addressing should be skipped altogether
    op_delegate: OpDelegate::Address(ops::interrupt),
    mnemonic: "IRQ",
    addressing: "IRQ Vector",
};

pub static NMI_INSTRUCTION: Instruction = Instruction {
    cycles: 8,
    bytes: 0,
    addr_delegate: addressing::imp, // addressing should be skipped altogether
    op_delegate: OpDelegate::Address(ops::interrupt),
    mnemonic: "NMI",
    addressing: "NMI Vector",
};

// ⚠️ here be automatically generated dragons 🐉

#[derive(num_derive::FromPrimitive, Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Opcode {
    BRK_imp = 0x00,
    ORA_ind_x = 0x01,
    ORA_zp = 0x05,
    ASL_zp = 0x06,
    PHP_imp = 0x08,
    ORA_imm = 0x09,
    ASL_acc = 0x0a,
    ORA_abs = 0x0d,
    ASL_abs = 0x0e,
    BPL_rel = 0x10,
    ORA_ind_y = 0x11,
    ORA_zp_x = 0x15,
    ASL_zp_x = 0x16,
    CLC_imp = 0x18,
    ORA_abs_y = 0x19,
    ORA_abs_x = 0x1d,
    ASL_abs_x = 0x1e,
    JSR_abs = 0x20,
    AND_ind_x = 0x21,
    BIT_zp = 0x24,
    AND_zp = 0x25,
    ROL_zp = 0x26,
    PLP_imp = 0x28,
    AND_imm = 0x29,
    ROL_acc = 0x2a,
    BIT_abs = 0x2c,
    AND_abs = 0x2d,
    ROL_abs = 0x2e,
    BMI_rel = 0x30,
    AND_ind_y = 0x31,
    AND_zp_x = 0x35,
    ROL_zp_x = 0x36,
    SEC_imp = 0x38,
    AND_abs_y = 0x39,
    AND_abs_x = 0x3d,
    ROL_abs_x = 0x3e,
    RTI_imp = 0x40,
    EOR_ind_x = 0x41,
    EOR_zp = 0x45,
    LSR_zp = 0x46,
    PHA_imp = 0x48,
    EOR_imm = 0x49,
    LSR_acc = 0x4a,
    JMP_abs = 0x4c,
    EOR_abs = 0x4d,
    LSR_abs = 0x4e,
    BVC_rel = 0x50,
    EOR_ind_y = 0x51,
    EOR_zp_x = 0x55,
    LSR_zp_x = 0x56,
    CLI_imp = 0x58,
    EOR_abs_y = 0x59,
    EOR_abs_x = 0x5d,
    LSR_abs_x = 0x5e,
    RTS_imp = 0x60,
    ADC_ind_x = 0x61,
    ADC_zp = 0x65,
    ROR_zp = 0x66,
    PLA_imp = 0x68,
    ADC_imm = 0x69,
    ROR_acc = 0x6a,
    JMP_ind = 0x6c,
    ADC_abs = 0x6d,
    ROR_abs = 0x6e,
    BVS_rel = 0x70,
    ADC_ind_y = 0x71,
    ADC_zp_x = 0x75,
    ROR_zp_x = 0x76,
    SEI_imp = 0x78,
    ADC_abs_y = 0x79,
    ADC_abs_x = 0x7d,
    ROR_abs_x = 0x7e,
    STA_ind_x = 0x81,
    STY_zp = 0x84,
    STA_zp = 0x85,
    STX_zp = 0x86,
    DEY_imp = 0x88,
    TXA_imp = 0x8a,
    STY_abs = 0x8c,
    STA_abs = 0x8d,
    STX_abs = 0x8e,
    BCC_rel = 0x90,
    STA_ind_y = 0x91,
    STY_zp_x = 0x94,
    STA_zp_x = 0x95,
    STX_zp_y = 0x96,
    TYA_imp = 0x98,
    STA_abs_y = 0x99,
    TXS_imp = 0x9a,
    STA_abs_x = 0x9d,
    LDY_imm = 0xa0,
    LDA_ind_x = 0xa1,
    LDX_imm = 0xa2,
    LDY_zp = 0xa4,
    LDA_zp = 0xa5,
    LDX_zp = 0xa6,
    TAY_imp = 0xa8,
    LDA_imm = 0xa9,
    TAX_imp = 0xaa,
    LDY_abs = 0xac,
    LDA_abs = 0xad,
    LDX_abs = 0xae,
    BCS_rel = 0xb0,
    LDA_ind_y = 0xb1,
    LDY_zp_x = 0xb4,
    LDA_zp_x = 0xb5,
    LDX_zp_y = 0xb6,
    CLV_imp = 0xb8,
    LDA_abs_y = 0xb9,
    TSX_imp = 0xba,
    LDY_abs_x = 0xbc,
    LDA_abs_x = 0xbd,
    LDX_abs_y = 0xbe,
    CPY_imm = 0xc0,
    CMP_ind_x = 0xc1,
    CPY_zp = 0xc4,
    CMP_zp = 0xc5,
    DEC_zp = 0xc6,
    INY_imp = 0xc8,
    CMP_imm = 0xc9,
    DEX_imp = 0xca,
    CPY_abs = 0xcc,
    CMP_abs = 0xcd,
    DEC_abs = 0xce,
    BNE_rel = 0xd0,
    CMP_ind_y = 0xd1,
    CMP_zp_x = 0xd5,
    DEC_zp_x = 0xd6,
    CLD_imp = 0xd8,
    CMP_abs_y = 0xd9,
    CMP_abs_x = 0xdd,
    DEC_abs_x = 0xde,
    CPX_imm = 0xe0,
    SBC_ind_x = 0xe1,
    CPX_zp = 0xe4,
    SBC_zp = 0xe5,
    INC_zp = 0xe6,
    INX_imp = 0xe8,
    SBC_imm = 0xe9,
    NOP_imp = 0xea,
    CPX_abs = 0xec,
    SBC_abs = 0xed,
    INC_abs = 0xee,
    BEQ_rel = 0xf0,
    SBC_ind_y = 0xf1,
    SBC_zp_x = 0xf5,
    INC_zp_x = 0xf6,
    SED_imp = 0xf8,
    SBC_abs_y = 0xf9,
    SBC_abs_x = 0xfd,
    INC_abs_x = 0xfe,
}

impl Opcode {
    pub fn to_instruction(&self) -> &'static Instruction {
        match self {
            
            Opcode::BRK_imp => {
                &Instruction{
                    cycles: 7,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::brk_implied),
                    mnemonic: "BRK",
                    addressing: "Implied",
                }
            }
            
            Opcode::ORA_ind_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::ind_x,
                    op_delegate: OpDelegate::Address(ops::ora_address),
                    mnemonic: "ORA",
                    addressing: "(IND, X)",
                }
            }
            
            Opcode::ORA_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::ora_address),
                    mnemonic: "ORA",
                    addressing: "ZP",
                }
            }
            
            Opcode::ASL_zp => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::asl_address),
                    mnemonic: "ASL",
                    addressing: "ZP",
                }
            }
            
            Opcode::PHP_imp => {
                &Instruction{
                    cycles: 3,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::php_implied),
                    mnemonic: "PHP",
                    addressing: "Implied",
                }
            }
            
            Opcode::ORA_imm => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::imm,
                    op_delegate: OpDelegate::Immediate(ops::ora_immediate),
                    mnemonic: "ORA",
                    addressing: "IMM",
                }
            }
            
            Opcode::ASL_acc => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::acc,
                    op_delegate: OpDelegate::Implied(ops::asl_implied),
                    mnemonic: "ASL",
                    addressing: "Accum",
                }
            }
            
            Opcode::ORA_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::ora_address),
                    mnemonic: "ORA",
                    addressing: "Absolute",
                }
            }
            
            Opcode::ASL_abs => {
                &Instruction{
                    cycles: 6,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::asl_address),
                    mnemonic: "ASL",
                    addressing: "Absolute",
                }
            }
            
            Opcode::BPL_rel => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::rel,
                    op_delegate: OpDelegate::Address(ops::bpl_address),
                    mnemonic: "BPL",
                    addressing: "Relative",
                }
            }
            
            Opcode::ORA_ind_y => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::ind_y_extra,
                    op_delegate: OpDelegate::Address(ops::ora_address),
                    mnemonic: "ORA",
                    addressing: "(IND), Y",
                }
            }
            
            Opcode::ORA_zp_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::ora_address),
                    mnemonic: "ORA",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::ASL_zp_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::asl_address),
                    mnemonic: "ASL",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::CLC_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::clc_implied),
                    mnemonic: "CLC",
                    addressing: "Implied",
                }
            }
            
            Opcode::ORA_abs_y => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_y_extra,
                    op_delegate: OpDelegate::Address(ops::ora_address),
                    mnemonic: "ORA",
                    addressing: "ABS, Y",
                }
            }
            
            Opcode::ORA_abs_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_x_extra,
                    op_delegate: OpDelegate::Address(ops::ora_address),
                    mnemonic: "ORA",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::ASL_abs_x => {
                &Instruction{
                    cycles: 7,
                    bytes: 3,
                    addr_delegate: addressing::abs_x,
                    op_delegate: OpDelegate::Address(ops::asl_address),
                    mnemonic: "ASL",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::JSR_abs => {
                &Instruction{
                    cycles: 6,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::jsr_address),
                    mnemonic: "JSR",
                    addressing: "Absolute",
                }
            }
            
            Opcode::AND_ind_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::ind_x,
                    op_delegate: OpDelegate::Address(ops::and_address),
                    mnemonic: "AND",
                    addressing: "(IND, X)",
                }
            }
            
            Opcode::BIT_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::bit_address),
                    mnemonic: "BIT",
                    addressing: "ZP",
                }
            }
            
            Opcode::AND_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::and_address),
                    mnemonic: "AND",
                    addressing: "ZP",
                }
            }
            
            Opcode::ROL_zp => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::rol_address),
                    mnemonic: "ROL",
                    addressing: "ZP",
                }
            }
            
            Opcode::PLP_imp => {
                &Instruction{
                    cycles: 4,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::plp_implied),
                    mnemonic: "PLP",
                    addressing: "Implied",
                }
            }
            
            Opcode::AND_imm => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::imm,
                    op_delegate: OpDelegate::Immediate(ops::and_immediate),
                    mnemonic: "AND",
                    addressing: "IMM",
                }
            }
            
            Opcode::ROL_acc => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::acc,
                    op_delegate: OpDelegate::Implied(ops::rol_implied),
                    mnemonic: "ROL",
                    addressing: "Accum",
                }
            }
            
            Opcode::BIT_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::bit_address),
                    mnemonic: "BIT",
                    addressing: "Absolute",
                }
            }
            
            Opcode::AND_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::and_address),
                    mnemonic: "AND",
                    addressing: "Absolute",
                }
            }
            
            Opcode::ROL_abs => {
                &Instruction{
                    cycles: 6,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::rol_address),
                    mnemonic: "ROL",
                    addressing: "Absolute",
                }
            }
            
            Opcode::BMI_rel => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::rel,
                    op_delegate: OpDelegate::Address(ops::bmi_address),
                    mnemonic: "BMI",
                    addressing: "Relative",
                }
            }
            
            Opcode::AND_ind_y => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::ind_y_extra,
                    op_delegate: OpDelegate::Address(ops::and_address),
                    mnemonic: "AND",
                    addressing: "(IND), Y",
                }
            }
            
            Opcode::AND_zp_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::and_address),
                    mnemonic: "AND",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::ROL_zp_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::rol_address),
                    mnemonic: "ROL",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::SEC_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::sec_implied),
                    mnemonic: "SEC",
                    addressing: "Implied",
                }
            }
            
            Opcode::AND_abs_y => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_y_extra,
                    op_delegate: OpDelegate::Address(ops::and_address),
                    mnemonic: "AND",
                    addressing: "ABS, Y",
                }
            }
            
            Opcode::AND_abs_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_x_extra,
                    op_delegate: OpDelegate::Address(ops::and_address),
                    mnemonic: "AND",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::ROL_abs_x => {
                &Instruction{
                    cycles: 7,
                    bytes: 3,
                    addr_delegate: addressing::abs_x,
                    op_delegate: OpDelegate::Address(ops::rol_address),
                    mnemonic: "ROL",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::RTI_imp => {
                &Instruction{
                    cycles: 6,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::rti_implied),
                    mnemonic: "RTI",
                    addressing: "Implied",
                }
            }
            
            Opcode::EOR_ind_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::ind_x,
                    op_delegate: OpDelegate::Address(ops::eor_address),
                    mnemonic: "EOR",
                    addressing: "(IND, X)",
                }
            }
            
            Opcode::EOR_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::eor_address),
                    mnemonic: "EOR",
                    addressing: "ZP",
                }
            }
            
            Opcode::LSR_zp => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::lsr_address),
                    mnemonic: "LSR",
                    addressing: "ZP",
                }
            }
            
            Opcode::PHA_imp => {
                &Instruction{
                    cycles: 3,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::pha_implied),
                    mnemonic: "PHA",
                    addressing: "Implied",
                }
            }
            
            Opcode::EOR_imm => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::imm,
                    op_delegate: OpDelegate::Immediate(ops::eor_immediate),
                    mnemonic: "EOR",
                    addressing: "IMM",
                }
            }
            
            Opcode::LSR_acc => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::acc,
                    op_delegate: OpDelegate::Implied(ops::lsr_implied),
                    mnemonic: "LSR",
                    addressing: "Accum",
                }
            }
            
            Opcode::JMP_abs => {
                &Instruction{
                    cycles: 3,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::jmp_address),
                    mnemonic: "JMP",
                    addressing: "Absolute",
                }
            }
            
            Opcode::EOR_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::eor_address),
                    mnemonic: "EOR",
                    addressing: "Absolute",
                }
            }
            
            Opcode::LSR_abs => {
                &Instruction{
                    cycles: 6,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::lsr_address),
                    mnemonic: "LSR",
                    addressing: "Absolute",
                }
            }
            
            Opcode::BVC_rel => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::rel,
                    op_delegate: OpDelegate::Address(ops::bvc_address),
                    mnemonic: "BVC",
                    addressing: "Relative",
                }
            }
            
            Opcode::EOR_ind_y => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::ind_y_extra,
                    op_delegate: OpDelegate::Address(ops::eor_address),
                    mnemonic: "EOR",
                    addressing: "(IND), Y",
                }
            }
            
            Opcode::EOR_zp_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::eor_address),
                    mnemonic: "EOR",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::LSR_zp_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::lsr_address),
                    mnemonic: "LSR",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::CLI_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::cli_implied),
                    mnemonic: "CLI",
                    addressing: "Implied",
                }
            }
            
            Opcode::EOR_abs_y => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_y_extra,
                    op_delegate: OpDelegate::Address(ops::eor_address),
                    mnemonic: "EOR",
                    addressing: "ABS, Y",
                }
            }
            
            Opcode::EOR_abs_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_x_extra,
                    op_delegate: OpDelegate::Address(ops::eor_address),
                    mnemonic: "EOR",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::LSR_abs_x => {
                &Instruction{
                    cycles: 7,
                    bytes: 3,
                    addr_delegate: addressing::abs_x,
                    op_delegate: OpDelegate::Address(ops::lsr_address),
                    mnemonic: "LSR",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::RTS_imp => {
                &Instruction{
                    cycles: 6,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::rts_implied),
                    mnemonic: "RTS",
                    addressing: "Implied",
                }
            }
            
            Opcode::ADC_ind_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::ind_x,
                    op_delegate: OpDelegate::Address(ops::adc_address),
                    mnemonic: "ADC",
                    addressing: "(IND, X)",
                }
            }
            
            Opcode::ADC_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::adc_address),
                    mnemonic: "ADC",
                    addressing: "ZP",
                }
            }
            
            Opcode::ROR_zp => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::ror_address),
                    mnemonic: "ROR",
                    addressing: "ZP",
                }
            }
            
            Opcode::PLA_imp => {
                &Instruction{
                    cycles: 4,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::pla_implied),
                    mnemonic: "PLA",
                    addressing: "Implied",
                }
            }
            
            Opcode::ADC_imm => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::imm,
                    op_delegate: OpDelegate::Immediate(ops::adc_immediate),
                    mnemonic: "ADC",
                    addressing: "IMM",
                }
            }
            
            Opcode::ROR_acc => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::acc,
                    op_delegate: OpDelegate::Implied(ops::ror_implied),
                    mnemonic: "ROR",
                    addressing: "Accum",
                }
            }
            
            Opcode::JMP_ind => {
                &Instruction{
                    cycles: 5,
                    bytes: 3,
                    addr_delegate: addressing::ind,
                    op_delegate: OpDelegate::Address(ops::jmp_address),
                    mnemonic: "JMP",
                    addressing: "Indirect",
                }
            }
            
            Opcode::ADC_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::adc_address),
                    mnemonic: "ADC",
                    addressing: "Absolute",
                }
            }
            
            Opcode::ROR_abs => {
                &Instruction{
                    cycles: 6,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::ror_address),
                    mnemonic: "ROR",
                    addressing: "Absolute",
                }
            }
            
            Opcode::BVS_rel => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::rel,
                    op_delegate: OpDelegate::Address(ops::bvs_address),
                    mnemonic: "BVS",
                    addressing: "Relative",
                }
            }
            
            Opcode::ADC_ind_y => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::ind_y_extra,
                    op_delegate: OpDelegate::Address(ops::adc_address),
                    mnemonic: "ADC",
                    addressing: "(IND), Y",
                }
            }
            
            Opcode::ADC_zp_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::adc_address),
                    mnemonic: "ADC",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::ROR_zp_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::ror_address),
                    mnemonic: "ROR",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::SEI_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::sei_implied),
                    mnemonic: "SEI",
                    addressing: "Implied",
                }
            }
            
            Opcode::ADC_abs_y => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_y_extra,
                    op_delegate: OpDelegate::Address(ops::adc_address),
                    mnemonic: "ADC",
                    addressing: "ABS, Y",
                }
            }
            
            Opcode::ADC_abs_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_x_extra,
                    op_delegate: OpDelegate::Address(ops::adc_address),
                    mnemonic: "ADC",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::ROR_abs_x => {
                &Instruction{
                    cycles: 7,
                    bytes: 3,
                    addr_delegate: addressing::abs_x,
                    op_delegate: OpDelegate::Address(ops::ror_address),
                    mnemonic: "ROR",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::STA_ind_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::ind_x,
                    op_delegate: OpDelegate::Address(ops::sta_address),
                    mnemonic: "STA",
                    addressing: "(IND, X)",
                }
            }
            
            Opcode::STY_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::sty_address),
                    mnemonic: "STY",
                    addressing: "ZP",
                }
            }
            
            Opcode::STA_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::sta_address),
                    mnemonic: "STA",
                    addressing: "ZP",
                }
            }
            
            Opcode::STX_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::stx_address),
                    mnemonic: "STX",
                    addressing: "ZP",
                }
            }
            
            Opcode::DEY_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::dey_implied),
                    mnemonic: "DEY",
                    addressing: "Implied",
                }
            }
            
            Opcode::TXA_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::txa_implied),
                    mnemonic: "TXA",
                    addressing: "Implied",
                }
            }
            
            Opcode::STY_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::sty_address),
                    mnemonic: "STY",
                    addressing: "Absolute",
                }
            }
            
            Opcode::STA_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::sta_address),
                    mnemonic: "STA",
                    addressing: "Absolute",
                }
            }
            
            Opcode::STX_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::stx_address),
                    mnemonic: "STX",
                    addressing: "Absolute",
                }
            }
            
            Opcode::BCC_rel => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::rel,
                    op_delegate: OpDelegate::Address(ops::bcc_address),
                    mnemonic: "BCC",
                    addressing: "Relative",
                }
            }
            
            Opcode::STA_ind_y => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::ind_y,
                    op_delegate: OpDelegate::Address(ops::sta_address),
                    mnemonic: "STA",
                    addressing: "(IND), Y",
                }
            }
            
            Opcode::STY_zp_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::sty_address),
                    mnemonic: "STY",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::STA_zp_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::sta_address),
                    mnemonic: "STA",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::STX_zp_y => {
                &Instruction{
                    cycles: 4,
                    bytes: 2,
                    addr_delegate: addressing::zp_y,
                    op_delegate: OpDelegate::Address(ops::stx_address),
                    mnemonic: "STX",
                    addressing: "ZP, Y",
                }
            }
            
            Opcode::TYA_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::tya_implied),
                    mnemonic: "TYA",
                    addressing: "Implied",
                }
            }
            
            Opcode::STA_abs_y => {
                &Instruction{
                    cycles: 5,
                    bytes: 3,
                    addr_delegate: addressing::abs_y,
                    op_delegate: OpDelegate::Address(ops::sta_address),
                    mnemonic: "STA",
                    addressing: "ABS, Y",
                }
            }
            
            Opcode::TXS_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::txs_implied),
                    mnemonic: "TXS",
                    addressing: "Implied",
                }
            }
            
            Opcode::STA_abs_x => {
                &Instruction{
                    cycles: 5,
                    bytes: 3,
                    addr_delegate: addressing::abs_x,
                    op_delegate: OpDelegate::Address(ops::sta_address),
                    mnemonic: "STA",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::LDY_imm => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::imm,
                    op_delegate: OpDelegate::Immediate(ops::ldy_immediate),
                    mnemonic: "LDY",
                    addressing: "IMM",
                }
            }
            
            Opcode::LDA_ind_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::ind_x,
                    op_delegate: OpDelegate::Address(ops::lda_address),
                    mnemonic: "LDA",
                    addressing: "(IND, X)",
                }
            }
            
            Opcode::LDX_imm => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::imm,
                    op_delegate: OpDelegate::Immediate(ops::ldx_immediate),
                    mnemonic: "LDX",
                    addressing: "IMM",
                }
            }
            
            Opcode::LDY_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::ldy_address),
                    mnemonic: "LDY",
                    addressing: "ZP",
                }
            }
            
            Opcode::LDA_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::lda_address),
                    mnemonic: "LDA",
                    addressing: "ZP",
                }
            }
            
            Opcode::LDX_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::ldx_address),
                    mnemonic: "LDX",
                    addressing: "ZP",
                }
            }
            
            Opcode::TAY_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::tay_implied),
                    mnemonic: "TAY",
                    addressing: "Implied",
                }
            }
            
            Opcode::LDA_imm => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::imm,
                    op_delegate: OpDelegate::Immediate(ops::lda_immediate),
                    mnemonic: "LDA",
                    addressing: "IMM",
                }
            }
            
            Opcode::TAX_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::tax_implied),
                    mnemonic: "TAX",
                    addressing: "Implied",
                }
            }
            
            Opcode::LDY_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::ldy_address),
                    mnemonic: "LDY",
                    addressing: "Absolute",
                }
            }
            
            Opcode::LDA_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::lda_address),
                    mnemonic: "LDA",
                    addressing: "Absolute",
                }
            }
            
            Opcode::LDX_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::ldx_address),
                    mnemonic: "LDX",
                    addressing: "Absolute",
                }
            }
            
            Opcode::BCS_rel => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::rel,
                    op_delegate: OpDelegate::Address(ops::bcs_address),
                    mnemonic: "BCS",
                    addressing: "Relative",
                }
            }
            
            Opcode::LDA_ind_y => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::ind_y_extra,
                    op_delegate: OpDelegate::Address(ops::lda_address),
                    mnemonic: "LDA",
                    addressing: "(IND), Y",
                }
            }
            
            Opcode::LDY_zp_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::ldy_address),
                    mnemonic: "LDY",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::LDA_zp_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::lda_address),
                    mnemonic: "LDA",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::LDX_zp_y => {
                &Instruction{
                    cycles: 4,
                    bytes: 2,
                    addr_delegate: addressing::zp_y,
                    op_delegate: OpDelegate::Address(ops::ldx_address),
                    mnemonic: "LDX",
                    addressing: "ZP, Y",
                }
            }
            
            Opcode::CLV_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::clv_implied),
                    mnemonic: "CLV",
                    addressing: "Implied",
                }
            }
            
            Opcode::LDA_abs_y => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_y_extra,
                    op_delegate: OpDelegate::Address(ops::lda_address),
                    mnemonic: "LDA",
                    addressing: "ABS, Y",
                }
            }
            
            Opcode::TSX_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::tsx_implied),
                    mnemonic: "TSX",
                    addressing: "Implied",
                }
            }
            
            Opcode::LDY_abs_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_x_extra,
                    op_delegate: OpDelegate::Address(ops::ldy_address),
                    mnemonic: "LDY",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::LDA_abs_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_x_extra,
                    op_delegate: OpDelegate::Address(ops::lda_address),
                    mnemonic: "LDA",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::LDX_abs_y => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_y_extra,
                    op_delegate: OpDelegate::Address(ops::ldx_address),
                    mnemonic: "LDX",
                    addressing: "ABS, Y",
                }
            }
            
            Opcode::CPY_imm => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::imm,
                    op_delegate: OpDelegate::Immediate(ops::cpy_immediate),
                    mnemonic: "CPY",
                    addressing: "IMM",
                }
            }
            
            Opcode::CMP_ind_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::ind_x,
                    op_delegate: OpDelegate::Address(ops::cmp_address),
                    mnemonic: "CMP",
                    addressing: "(IND, X)",
                }
            }
            
            Opcode::CPY_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::cpy_address),
                    mnemonic: "CPY",
                    addressing: "ZP",
                }
            }
            
            Opcode::CMP_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::cmp_address),
                    mnemonic: "CMP",
                    addressing: "ZP",
                }
            }
            
            Opcode::DEC_zp => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::dec_address),
                    mnemonic: "DEC",
                    addressing: "ZP",
                }
            }
            
            Opcode::INY_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::iny_implied),
                    mnemonic: "INY",
                    addressing: "Implied",
                }
            }
            
            Opcode::CMP_imm => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::imm,
                    op_delegate: OpDelegate::Immediate(ops::cmp_immediate),
                    mnemonic: "CMP",
                    addressing: "IMM",
                }
            }
            
            Opcode::DEX_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::dex_implied),
                    mnemonic: "DEX",
                    addressing: "Implied",
                }
            }
            
            Opcode::CPY_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::cpy_address),
                    mnemonic: "CPY",
                    addressing: "Absolute",
                }
            }
            
            Opcode::CMP_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::cmp_address),
                    mnemonic: "CMP",
                    addressing: "Absolute",
                }
            }
            
            Opcode::DEC_abs => {
                &Instruction{
                    cycles: 6,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::dec_address),
                    mnemonic: "DEC",
                    addressing: "Absolute",
                }
            }
            
            Opcode::BNE_rel => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::rel,
                    op_delegate: OpDelegate::Address(ops::bne_address),
                    mnemonic: "BNE",
                    addressing: "Relative",
                }
            }
            
            Opcode::CMP_ind_y => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::ind_y_extra,
                    op_delegate: OpDelegate::Address(ops::cmp_address),
                    mnemonic: "CMP",
                    addressing: "(IND), Y",
                }
            }
            
            Opcode::CMP_zp_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::cmp_address),
                    mnemonic: "CMP",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::DEC_zp_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::dec_address),
                    mnemonic: "DEC",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::CLD_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::cld_implied),
                    mnemonic: "CLD",
                    addressing: "Implied",
                }
            }
            
            Opcode::CMP_abs_y => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_y_extra,
                    op_delegate: OpDelegate::Address(ops::cmp_address),
                    mnemonic: "CMP",
                    addressing: "ABS, Y",
                }
            }
            
            Opcode::CMP_abs_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_x_extra,
                    op_delegate: OpDelegate::Address(ops::cmp_address),
                    mnemonic: "CMP",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::DEC_abs_x => {
                &Instruction{
                    cycles: 7,
                    bytes: 3,
                    addr_delegate: addressing::abs_x,
                    op_delegate: OpDelegate::Address(ops::dec_address),
                    mnemonic: "DEC",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::CPX_imm => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::imm,
                    op_delegate: OpDelegate::Immediate(ops::cpx_immediate),
                    mnemonic: "CPX",
                    addressing: "IMM",
                }
            }
            
            Opcode::SBC_ind_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::ind_x,
                    op_delegate: OpDelegate::Address(ops::sbc_address),
                    mnemonic: "SBC",
                    addressing: "(IND, X)",
                }
            }
            
            Opcode::CPX_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::cpx_address),
                    mnemonic: "CPX",
                    addressing: "ZP",
                }
            }
            
            Opcode::SBC_zp => {
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::sbc_address),
                    mnemonic: "SBC",
                    addressing: "ZP",
                }
            }
            
            Opcode::INC_zp => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::zp,
                    op_delegate: OpDelegate::Address(ops::inc_address),
                    mnemonic: "INC",
                    addressing: "ZP",
                }
            }
            
            Opcode::INX_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::inx_implied),
                    mnemonic: "INX",
                    addressing: "Implied",
                }
            }
            
            Opcode::SBC_imm => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::imm,
                    op_delegate: OpDelegate::Immediate(ops::sbc_immediate),
                    mnemonic: "SBC",
                    addressing: "IMM",
                }
            }
            
            Opcode::NOP_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::nop_implied),
                    mnemonic: "NOP",
                    addressing: "Implied",
                }
            }
            
            Opcode::CPX_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::cpx_address),
                    mnemonic: "CPX",
                    addressing: "Absolute",
                }
            }
            
            Opcode::SBC_abs => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::sbc_address),
                    mnemonic: "SBC",
                    addressing: "Absolute",
                }
            }
            
            Opcode::INC_abs => {
                &Instruction{
                    cycles: 6,
                    bytes: 3,
                    addr_delegate: addressing::abs,
                    op_delegate: OpDelegate::Address(ops::inc_address),
                    mnemonic: "INC",
                    addressing: "Absolute",
                }
            }
            
            Opcode::BEQ_rel => {
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::rel,
                    op_delegate: OpDelegate::Address(ops::beq_address),
                    mnemonic: "BEQ",
                    addressing: "Relative",
                }
            }
            
            Opcode::SBC_ind_y => {
                &Instruction{
                    cycles: 5,
                    bytes: 2,
                    addr_delegate: addressing::ind_y_extra,
                    op_delegate: OpDelegate::Address(ops::sbc_address),
                    mnemonic: "SBC",
                    addressing: "(IND), Y",
                }
            }
            
            Opcode::SBC_zp_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::sbc_address),
                    mnemonic: "SBC",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::INC_zp_x => {
                &Instruction{
                    cycles: 6,
                    bytes: 2,
                    addr_delegate: addressing::zp_x,
                    op_delegate: OpDelegate::Address(ops::inc_address),
                    mnemonic: "INC",
                    addressing: "ZP, X",
                }
            }
            
            Opcode::SED_imp => {
                &Instruction{
                    cycles: 2,
                    bytes: 1,
                    addr_delegate: addressing::imp,
                    op_delegate: OpDelegate::Implied(ops::sed_implied),
                    mnemonic: "SED",
                    addressing: "Implied",
                }
            }
            
            Opcode::SBC_abs_y => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_y_extra,
                    op_delegate: OpDelegate::Address(ops::sbc_address),
                    mnemonic: "SBC",
                    addressing: "ABS, Y",
                }
            }
            
            Opcode::SBC_abs_x => {
                &Instruction{
                    cycles: 4,
                    bytes: 3,
                    addr_delegate: addressing::abs_x_extra,
                    op_delegate: OpDelegate::Address(ops::sbc_address),
                    mnemonic: "SBC",
                    addressing: "ABS, X",
                }
            }
            
            Opcode::INC_abs_x => {
                &Instruction{
                    cycles: 7,
                    bytes: 3,
                    addr_delegate: addressing::abs_x,
                    op_delegate: OpDelegate::Address(ops::inc_address),
                    mnemonic: "INC",
                    addressing: "ABS, X",
                }
            }
        }
    }
}
