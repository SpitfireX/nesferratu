#[allow(unused_variables)]
pub mod instructions;

use num_traits::FromPrimitive;
use instructions::{Instruction, Operand, Opcode, AddrDelegateReturn};
use crate::BusMessage;

pub trait CPU {
    fn clock(&mut self, data: Option<u8>) -> BusMessage;

    fn irq(&mut self);

    fn nmi(&mut self);

    fn reset(&mut self);
}

#[repr(u8)]
enum CPUFlags {
    C = (1 << 0),   // Carry Bit
    Z = (1 << 1),   // Zero
    I = (1 << 2),   // Disable Interrupts
    D = (1 << 3),   // Decimal Mode (unused in this implementation)
    B = (1 << 4),   // Break
                    // 3rd bit unused and always high
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
    addr: u16,  // scratch pad for addressing modes
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

pub struct CPUInterpreter {
    // CPU state
    registers: CPURegisters,

    // helper variables
    total_cycles: usize,
    op_cycle: usize,
    addr_cycle: usize,
    exec_cycle: usize,
    state: CPUState,
    instruction: Option<&'static Instruction>,
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
            println!("op: {}, addr: {}, bytes: {}, cycles: {}", i.mnemonic, i.addressing, i.bytes, i.cycles);
        }
        println!("{:02X?}", self.registers);
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
                            self.instruction = Some(
                                Opcode::from_u8(self.registers.op)
                                    .expect("Illegal Opcode")
                                    .to_instruction()
                            );
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

                    if self.op_cycle < self.instruction.expect("CPU::instruction cant be None after decoding").bytes {
                        return Read{addr: self.registers.pc};
                    } else {
                        self.state = Addressing;
                    }
                }
                CPUState::Addressing => {
                    self.addr_cycle += 1;

                    let instruction = self.instruction
                                        .expect("CPU::instruction is None, this should be impossible at this point");

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
                }
                CPUState::Execute => {
                    self.exec_cycle += 1;
                    
                    let instruction = self.instruction
                                        .as_ref()
                                        .expect("CPU::instruction is None, this should be impossible at this point");

                    let operand = self.operand
                                        .as_ref()
                                        .expect("CPU::operand can't be None after addressing");
                    
                    let msg: BusMessage = match operand {
                        Operand::Implied => {
                            instruction.op_delegate.implied().unwrap()(&mut self.registers, self.exec_cycle)
                        }
                        Operand::Immediate(imm) => {
                            instruction.op_delegate.immediate().unwrap()(&mut self.registers, *imm, self.exec_cycle)
                        }
                        Operand::Address(addr) => {
                            instruction.op_delegate.address().unwrap()(&mut self.registers, *addr, self.exec_cycle)
                        }
                    };

                    if self.op_cycle < instruction.cycles {
                        return msg;
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

        // zero internal interpreter state
        self.total_cycles = 0;
        self.op_cycle = 0;
        self.addr_cycle = 0;
        self.exec_cycle = 0;

        // set IRQ disable flag
        self.registers.set_flag(CPUFlags::I, true);

        // dispatch reset pseudo-instruction with the reset vector address as operand
        self.state = CPUState::Execute;
        self.operand = Some(Operand::Address(0xFFFC));
        self.instruction = Some(&instructions::RESET_INSTRUCTION);
    }
}
