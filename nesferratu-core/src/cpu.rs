use num_traits::FromPrimitive;

pub trait CPU {
    fn clock(&mut self, data: Option<u8>) -> BusMessage;

    fn irq(&mut self);

    fn nmi(&mut self);

    fn reset(&mut self);
}

#[derive(Debug)]
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
    Fetch,
    Addressing,
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
    
    // emulation helpers
    op: u8,     // 1st byte of instruction
    o1: u8,     // 2nd byte of instruction
    o2: u8,     // 3rd byte of instruction
    data: u8,   // data from bus
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

struct Instruction {
    cyles: usize,
    bytes: usize,
    addr_delegate: AddrDelegate,
    op_delegate: OpDelegate,
    mnemonic: &'static str,
    addressing: &'static str,
}

pub enum Operand {
    Implied,
    Immediate(u8),
    Address(u16),
}

pub enum AddrDelegateReturn {
    Yield(BusMessage),
    Return(Operand),
}

enum OpDelegate {
    Implied(OpDelegateImplied),
    Immediate(OpDelegateImmediate),
    Address(OpDelegateAddress),
}

type AddrDelegate = fn(&mut CPURegisters, usize) -> AddrDelegateReturn;
type OpDelegateImplied = fn(&mut CPURegisters, usize) -> BusMessage;
type OpDelegateImmediate = fn(&mut CPURegisters, u8, usize) -> BusMessage;
type OpDelegateAddress = fn(&mut CPURegisters, u16, usize) -> BusMessage;

pub struct CPUInterpreter {
    // CPU state
    registers: CPURegisters,

    // helper variables
    total_cycles: usize,
    op_cycle: usize,
    addr_cycle: usize,
    exec_cycle: usize,
    state: CPUState,
    instruction: Option<Instruction>,
    operand: Option<Operand>,
}

impl CPUInterpreter {
    pub fn new() -> CPUInterpreter {
        CPUInterpreter {
            registers: CPURegisters::default(),

            total_cycles: 0,
            op_cycle: 0,
            addr_cycle: 0,
            exec_cycle: 0,
            state: CPUState::Halt,
            instruction: None,
            operand: None,
        }
    }

    fn print_debug(&self) {
        println!("cycle: {}, op_cycle: {}, state: {:?}", self.total_cycles, self.op_cycle, self.state);
        if let Some(i) = self.instruction.as_ref() {
            println!("op: {}, addr: {}, bytes: {}, cycles: {}", i.mnemonic, i.addressing, i.bytes, i.cyles);
        }
        println!("{:02X?}", self.registers);
    }

    fn decode_opcode(&self, opcode: u8) -> Instruction {
        let mnemonic: Opcodes = Opcodes::from_u8(opcode).expect("Invalid opcode");
        match mnemonic {
            Opcodes::LDA_imm => (
                Instruction{
                    cyles: 2,
                    bytes: 2,
                    addr_delegate: addressing::immediate,
                    op_delegate: OpDelegate::Immediate(ops::lda_imm),
                    mnemonic: "LDA",
                    addressing: "Immediate",
                }
            ),
            Opcodes::STA_zp => (
                Instruction{
                    cyles: 3,
                    bytes: 2,
                    addr_delegate: addressing::zero_page,
                    op_delegate: OpDelegate::Address(ops::sta_addr),
                    mnemonic: "STA",
                    addressing: "ZP",
                }
            ),
            _ => panic!("Unimplemented opcode 0x{:02X} = {:?}", opcode, mnemonic)
        }
    }
}

impl CPU for CPUInterpreter {

    fn clock(&mut self, data: Option<u8>) -> BusMessage {

        use CPUState::*;
        use BusMessage::*;

        self.total_cycles += 1;
        self.op_cycle += 1;
        self.print_debug();

        if let Some(data) = data {
            println!("\tbus data 0x{:02X}", data);
            self.registers.data = data;
        }

        // CPU state machine

        loop {
            match self.state {
                CPUState::Fetch => {
                    match self.op_cycle {
                        1 => {
                            self.registers.op = data.expect("Bus data can't be empty in fetch cycle 1");
                            self.instruction = Some(self.decode_opcode(self.registers.op));
                            self.registers.pc += 1;
                        },
                        2 => {
                            self.registers.o1 = data.expect("Bus data can't be empty in fetch cycle 2");
                            self.registers.pc += 1;
                        },
                        3 => {
                            self.registers.o2 = data.expect("Bus data can't be empty in fetch cycle 3");
                            self.registers.pc += 1;
                        },
                        _ => panic!("Fetch state can't take longer than 3 cycles"),
                    }

                    if self.op_cycle < self.instruction.as_ref().expect("CPU::instruction cant be None after decoding").bytes {
                        return Read{addr: self.registers.pc};
                    } else {
                        self.state = Addressing;
                    }
                }
                CPUState::Addressing => {
                    self.addr_cycle += 1;

                    if let Some(instruction) = self.instruction.as_ref() {
                        match (instruction.addr_delegate)(&mut self.registers, self.addr_cycle) {
                            AddrDelegateReturn::Yield(msg) => {
                                return msg;
                            }
                            AddrDelegateReturn::Return(operand) => {
                                self.operand = Some(operand);
                                self.state = Execute;
                                continue;
                            }
                        }
                    } else {
                        panic!("CPU::instruction is None, this should be impossible at this point");
                    }
                }
                CPUState::Execute => {
                    self.exec_cycle += 1;

                    if let Some(instruction) = self.instruction.as_ref() {
                        let operand = self.operand.as_ref().expect("CPU::operand can't be None after addressing");
                        let msg: Option<BusMessage>;

                        match instruction.op_delegate {
                            OpDelegate::Implied(delegate) => {
                                if let Operand::Implied = operand {
                                    msg = Some(delegate(&mut self.registers, self.exec_cycle));
                                } else {
                                    panic!("Incompatible operand type");
                                }
                            }
                            OpDelegate::Immediate(delegate) => {
                                if let Operand::Immediate(imm) = operand {
                                    msg = Some(delegate(&mut self.registers, *imm, self.exec_cycle));
                                } else {
                                    panic!("Incompatible operand type");
                                }
                            }
                            OpDelegate::Address(delegate) => {
                                if let Operand::Address(addr) = operand {
                                    msg = Some(delegate(&mut self.registers, *addr, self.exec_cycle));
                                } else {
                                    panic!("Incompatible operand type");
                                }
                            }
                        }

                        if self.op_cycle < instruction.cyles {
                            return msg.expect("BusMessage can't be None after OpDelegate execution");
                        } else {
                            // We're done with this instruction, prepare the next one!
                            self.op_cycle = 0;
                            self.addr_cycle = 0;
                            self.exec_cycle = 0;
                            self.instruction = None;
                            self.operand = None;
                            self.state = Fetch;
                            return Read{addr: self.registers.pc};
                        }
                    } else {
                        panic!("CPU::instruction is None, this should be impossilbe at this point");
                    }
                }
                CPUState::Halt => {
                    println!("CPU is halted");
                    return Nop;
                }
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
        println!("CPU reset");
        self.total_cycles = 0;
        self.op_cycle = 0;
        self.addr_cycle = 0;
        self.exec_cycle = 0;
        self.state = CPUState::Addressing;
        self.instruction = Some(Instruction {
            cyles: 8,
            bytes: 0,
            addr_delegate: addressing::reset_vector,
            op_delegate: OpDelegate::Implied(ops::reset),
            mnemonic: "RESET",
            addressing: "Reset Vector",
        });
    }
}

mod addressing {
    use super::{AddrDelegateReturn, CPURegisters, Operand};
    use super::BusMessage::*;

    pub fn reset_vector(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
        let reset_addr: u16 = 0xFFFC;
        
        match cycle {
            1 => AddrDelegateReturn::Yield(Read{addr: reset_addr}),
            2 => {
                regs.pc |= regs.data as u16; // set low byte of new PC address
                AddrDelegateReturn::Yield(Read{addr: reset_addr+1})
            },
            3 => {
                regs.pc |= (regs.data as u16) << 8; // set high byte of ne PC address
                AddrDelegateReturn::Return(Operand::Implied)
            },
            _ => panic!("Impossible cycle count in match"),
        }
    }

    pub fn immediate(regs: &mut CPURegisters, _cycle: usize) -> AddrDelegateReturn {
        AddrDelegateReturn::Return(Operand::Immediate(regs.o1))
    }

    pub fn zero_page(regs: &mut CPURegisters, _cycle: usize) -> AddrDelegateReturn {
        AddrDelegateReturn::Return(Operand::Address(regs.o1 as u16))
    }
}

mod ops {
    use super::{BusMessage, CPUFlags, CPURegisters};
    use super::BusMessage::*;

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

    pub fn reset(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
        match cycle {
            1 => {                
                // reset rest of the registers
                regs.a = 0x00;
                regs.x = 0x00;
                regs.y = 0x00;
                regs.sp = 0xFD; // default address for stack pointer
                regs.status = 0x00;

                // also the relevant helpers
                regs.op = 0x00;
                regs.o1 = 0x00;
                regs.o2 = 0x00;
                
                Nop
            },
            _ => Nop,
        }
    }
}