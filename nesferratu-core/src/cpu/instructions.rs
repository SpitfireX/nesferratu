use crate::cpu::{CPURegisters};
use crate::BusMessage;

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
            Opcode::LDA_imm => (
                &Instruction{
                    cycles: 2,
                    bytes: 2,
                    addr_delegate: addressing::immediate,
                    op_delegate: OpDelegate::Immediate(ops::lda_imm),
                    mnemonic: "LDA",
                    addressing: "Immediate",
                }
            ),
            Opcode::STA_zp => (
                &Instruction{
                    cycles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zero_page,
                    op_delegate: OpDelegate::Address(ops::sta_addr),
                    mnemonic: "STA",
                    addressing: "ZP",
                }
            ),
            _ => panic!("Unimplemented opcode 0x{:02X} = {:?}", *self as u8, self)
        }
    }
}

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

pub type AddrDelegate = fn(&mut CPURegisters, usize) -> AddrDelegateReturn;
pub type OpDelegateImplied = fn(&mut CPURegisters, usize) -> BusMessage;
pub type OpDelegateImmediate = fn(&mut CPURegisters, u8, usize) -> BusMessage;
pub type OpDelegateAddress = fn(&mut CPURegisters, u16, usize) -> BusMessage;

pub static RESET_INSTRUCTION: Instruction = Instruction {
    cycles: 8,
    bytes: 0,
    addr_delegate: addressing::implied, // addressing should be skipped altogether
    op_delegate: OpDelegate::Address(ops::reset),
    mnemonic: "RESET",
    addressing: "Reset Vector",
};

mod addressing {
    use crate::cpu::{AddrDelegateReturn, CPURegisters, Operand};

    pub fn implied(_regs: &mut CPURegisters, _cycle: usize) -> AddrDelegateReturn {
        AddrDelegateReturn::Return(Operand::Implied)
    }

    pub fn immediate(regs: &mut CPURegisters, _cycle: usize) -> AddrDelegateReturn {
        AddrDelegateReturn::Return(Operand::Immediate(regs.o1))
    }

    pub fn zero_page(regs: &mut CPURegisters, _cycle: usize) -> AddrDelegateReturn {
        AddrDelegateReturn::Return(Operand::Address(regs.o1 as u16))
    }
}

mod ops {
    use crate::cpu::{CPURegisters, CPUFlags};
    use crate::BusMessage;
    use crate::BusMessage::*;

    pub fn lda_imm(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
        regs.a = immediate;
        regs.set_flag(CPUFlags::Z, regs.a == 0);
        regs.set_flag(CPUFlags::N, regs.a & 0x80 == 0x80);
        Nop
    }

    pub fn sta_addr(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
        match cycle {
            1 => {
                Write{addr: address, data: regs.a}
            },
            _ => Nop,
        }
    }

    pub fn reset(regs: &mut CPURegisters, reset_vector: u16, cycle: usize) -> BusMessage {
        match cycle {
            x if x < 6 => Nop,
            6 => {
                regs.set_flag(CPUFlags::I, true);
                Read{addr: reset_vector}
            },
            7 => {
                regs.pc |= regs.data as u16; // set low byte of new PC address
                Read{addr: reset_vector+1}
            },
            8 => {
                regs.pc |= (regs.data as u16) << 8; // set high byte of ne PC address

                // reset rest of the registers
                regs.a = 0x00;
                regs.x = 0x00;
                regs.y = 0x00;
                regs.sp = 0xFD; // default address for stack pointer
                regs.status = 0x24; // 3rd bit unused and always high, I flag still set

                // also the relevant helpers
                regs.op = 0x00;
                regs.o1 = 0x00;
                regs.o2 = 0x00;
                
                Read{addr: regs.pc}
            },
            _ => panic!("Impossible cycle count in match, reset takes 8 cycles"),
        }
    }
}