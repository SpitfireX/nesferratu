use std::convert::TryFrom;

use num_traits::FromPrimitive;

pub trait CPU {
    fn clock(&mut self, data: Option<u8>) -> BusMessage;

    fn irq(&mut self);

    fn nmi(&mut self);

    fn reset(&mut self);
}

pub enum BusMessage {
    Read {addr: u16},
    Write {addr: u16, data: u8},
    Nop,
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Opcodes {
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
    XXX,
}

#[repr(u8)]
enum CPUFlags {
    C = (1 << 0),   // Carry Bit
    Z = (1 << 1),   // Zero
    I = (1 << 2),   // Disable Interrupts
    D = (1 << 3),   // Decimal Mode (unused in this implementation)
    B = (1 << 4),   // Break
    V = (1 << 6),   // Overflow
    N = (1 << 7),   // Negative
}

#[derive(Debug)]
enum CPUState {
    FetchDecode,
    Fetch,
    Execute,
    Halt,
}

#[derive(Default, Debug)]
pub struct CPURegisters {
    a: u8,      // Accumulator
    x: u8,      // X Register
    y: u8,      // Y Register
    sp: u8,     // Stack Pointer
    pc: u16,    // Program Counter
    status: u8, // Status Register
}

impl CPURegisters {
    fn get_flag(&self, flag: CPUFlags) -> bool {
        (self.status & (flag as u8)) > 0
    }

    fn set_flag(&mut self, flag: CPUFlags, value: bool) {
        if value {
            self.status |= flag as u8;
        } else {
            self.status &= !(flag as u8);
        }
    }
}

type OpDelegate = fn(&mut CPURegisters, Option<u8>, u8) -> BusMessage;

pub struct CPUInterpreter {
    // CPU internals
    registers: CPURegisters,

    // helper variables
    total_cycles: u32,
    op_cycle: u8,
    state: CPUState,
    fetch_op: Option<OpDelegate>,
    fetch_cycles: u8,
    exec_op: Option<OpDelegate>,
    exec_cycles: u8,
}

impl CPUInterpreter {
    pub fn new() -> CPUInterpreter {
        CPUInterpreter {
            registers: CPURegisters::default(),

            total_cycles: 0,
            op_cycle: 0,
            state: CPUState::Halt,
            fetch_op: None,
            fetch_cycles: 0,
            exec_op: None,
            exec_cycles: 0,
        }
    }

    fn print_debug(&self) {
        println!("cycle: {}, op_cycle: {}, state: {:?}", self.total_cycles, self.op_cycle, self.state);
        println!("{:02X?}", self.registers);
    }

    fn decode_opcode(&mut self, opcode: u8) -> ((u8, OpDelegate), (u8, OpDelegate)) {
        let opcode: Opcodes = Opcodes::from_u8(opcode).expect("Unknown opcode");
        match opcode {
            Opcodes::LDA_imm => ((1, Addressing::immediate), (1, Ops::lda)),
            _ => panic!("Unimplemented opcode {:?}", opcode)
        }
    }
}

impl CPU for CPUInterpreter {

    fn clock(&mut self, data: Option<u8>) -> BusMessage {

        use CPUState::*;
        use BusMessage::*;

        self.total_cycles += 1;
        self.print_debug();

        if let Some(data) = data {
            println!("\tbus data 0x{:02X}", data);
        }

        if let FetchDecode = self.state {
            let decoded = self.decode_opcode(data.expect("No Opcode to decode"));
            self.fetch_cycles = decoded.0.0;
            self.fetch_op = Some(decoded.0.1);
            self.exec_cycles = decoded.1.0;
            self.exec_op = Some(decoded.1.1);

            self.registers.pc += 1;

            if self.fetch_op.is_some() {
                self.state = Fetch;
            } else if self.exec_op.is_some() {
                self.state = Execute;
            } else {
                panic!("No valid CPU state to switch to");
            }
        }

        match self.state {
            FetchDecode => {
                panic!("CPU shouldn't be in decode state anymore")
            }
            Fetch => {
                let op = self.fetch_op.expect("CPU in fetch state without op");
                let msg = op(&mut self.registers, data, self.op_cycle);

                if self.op_cycle >= self.fetch_cycles { // fetch is done
                    self.state = Execute;
                    self.op_cycle = 1;
                } else {
                    self.op_cycle += 1;
                }

                msg
            }
            Execute => {
                let op = self.exec_op.expect("CPU in execute state without op");
                let msg = op(&mut self.registers, data, self.op_cycle);
                
                if self.op_cycle >= self.exec_cycles { // execution is done
                    self.state = FetchDecode;
                    self.op_cycle = 1;
                } else {
                    self.op_cycle += 1;
                }

                msg
            }
            Halt => {
                println!("CPU is halted");
                Nop
            }
        }
    }

    fn irq(&mut self) {
        todo!()
    }

    fn nmi(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        self.total_cycles = 0;
        self.op_cycle = 1;
        self.state = CPUState::Execute;
        self.fetch_op = None;
        self.fetch_cycles = 0;
        self.exec_op = Some(Ops::reset);
        self.exec_cycles = 8;
    }
}

mod Addressing {
    use super::{BusMessage, CPUFlags, CPURegisters};
    use super::BusMessage::*;

    pub fn immediate(regs: &mut CPURegisters, data: Option<u8>, cycle: u8) -> BusMessage {
        regs.pc += 1;
        Read{addr: regs.pc}
    }
}

mod Ops {
    use super::{BusMessage, CPUFlags, CPURegisters};
    use super::BusMessage::*;

    pub fn lda(regs: &mut CPURegisters, data: Option<u8>, cycle: u8) -> BusMessage {
        regs.a = data.expect("Empty data");
        
        regs.pc += 1;
        Read{addr: regs.pc}
    }

    pub fn reset(regs: &mut CPURegisters, data: Option<u8>, cycle: u8) -> BusMessage {
        let reset_addr: u16 = 0xFFFC;
        
        match cycle {
            1 => Read{addr: reset_addr},
            2 => {
                regs.pc |= data.expect("Empty data") as u16; // set low byte of new PC addresss
                Read{addr: reset_addr+1}
            },
            3 => {
                regs.pc |= (data.expect("Empty data") as u16) << 8; // set high byte of the new PC address
                
                // reset rest of the registers
                regs.a = 0x00;
                regs.x = 0x00;
                regs.y = 0x00;
                regs.sp = 0xFD; // default address for stack pointer
                regs.status = 0x00;
                
                Nop
            },
            8 => Read{addr: regs.pc},
            _ => Nop,
        }
    }
}