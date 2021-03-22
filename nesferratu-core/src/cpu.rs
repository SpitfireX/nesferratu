#[allow(unused_variables)]
pub mod instructions;

use std::fmt::Display;

use num_traits::FromPrimitive;
use instructions::{AddrDelegateReturn, Instruction, Opcode, Operand};
use crate::BusMessage;
use crate::debugger::CpuDebugger;

pub trait CPU {
    fn clock(&mut self, data: Option<u8>) -> BusMessage;

    fn irq(&mut self);

    fn nmi(&mut self);

    fn reset(&mut self);
}

#[repr(u8)]
enum CpuFlags {
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
enum CpuInterpreterState {
    Fetch,
    Addressing,
    Execute,
    Halt,
}

#[derive(Debug, PartialEq)]
pub enum Interrupt {
    None,
    Irq(u16),
    Nmi(u16),
}

#[derive(Default, Debug)]
pub struct CpuState {
    // CPU registers
    regs: CpuRegisters,

    // emulation helpers
    op: u8,             // 1st byte of instruction
    o1: u8,             // 2nd byte of instruction
    o2: u8,             // 3rd byte of instruction
    addr: u16,          // scratch pad for addressing modes
    data: u8,           // data from bus
    extra_cycle: bool   // flag to add one cycle to the instructions cycle length during the next cycle
}

#[derive(Default, Debug)]
pub struct CpuRegisters {
    pub a: u8,      // Accumulator
    pub x: u8,      // X Register
    pub y: u8,      // Y Register
    pub sp: u8,     // Stack Pointer
    pub pc: u16,    // Program Counter
    pub status: u8, // Status Register
}

impl CpuRegisters {
    fn get_flag(&self, flag: CpuFlags) -> bool {
        (self.status & (flag as u8)) > 0
    }

    fn set_flag(&mut self, flag: CpuFlags, value: bool) {
        if value {
            self.status |= flag as u8;
        } else {
            self.status &= !(flag as u8);
        }
    }
}

impl Display for CpuRegisters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pc = format!("({})", self.pc);
        let sp = format!("({})", self.sp);
        let a = format!("({})", self.a);
        let x = format!("({})", self.x);
        let y = format!("({})", self.y);
        
        let mut bullets = String::new();
        
        for i in 0..8 {
            if i == 2 {
                bullets.push('-'); 
            } else if self.status >> i & 1 == 1 {
                bullets.push('●');
            } else {
                bullets.push('○');
            }
        }

        writeln!(f, "┌──────┬────────────────────────┬────────────────┬─────────────────┐")?;
        writeln!(f, "│ CPU  │ PC: 0x{:04X}     {:>7} │ SP: 0x{:02X} {:>5} │ Flags: NV-BDIZC │", self.pc, pc, self.sp, sp)?;
        writeln!(f, "├──────┴────────┬───────────────┼────────────────┤        {} │", bullets)?;
        writeln!(f, "│ A: 0x{:02X} {:>5} │ X: 0x{:02X} {:>5} │  Y: 0x{:02X} {:>5} │        {:08b} │", self.a, a, self.x, x, self.y, y, self.status)?;
        write!(f, "└───────────────┴───────────────┴────────────────┴─────────────────┘")?;
        Ok(())
    }
}

pub struct EmulationState {
    pub total_cycles: u64,
    pub op_cycle: u8,
    pub additional_cycles: u8,
    pub instruction_done: bool,
    pub interrupt_request: Interrupt,
}

impl Default for EmulationState {
    fn default() -> Self {
        Self {
            total_cycles: 0,
            op_cycle: 0,
            additional_cycles: 0,
            instruction_done: true,
            interrupt_request: Interrupt::None,
        }
    }
}

pub struct CpuInterpreter {
    // CPU state
    cpu_state: CpuState,

    // (public) Emulation State
    emu_state: EmulationState,

    //  Interpreter State
    addr_cycle: u8,
    exec_cycle: u8,
    exec_state: CpuInterpreterState,
    instruction: Option<&'static Instruction>,
    operand: Option<Operand>,
}

impl CpuInterpreter {
    pub fn new() -> CpuInterpreter {
        CpuInterpreter {
            cpu_state: CpuState::default(),

            emu_state: EmulationState::default(),

            addr_cycle: 0,
            exec_cycle: 0,
            exec_state: CpuInterpreterState::Halt,
            instruction: None,
            operand: None,
        }
    }

    fn print_debug(&self) {
        println!("cycle: {}, op_cycle: {}, state: {:?}", self.emu_state.total_cycles, self.emu_state.op_cycle, self.exec_state);
        if let Some(i) = self.instruction.as_ref() {
            println!("op: {}, addr: {}, bytes: {}, cycles: {}", i.mnemonic, i.addressing, i.bytes, i.cycles);
        }
        println!("{:02X?}", self.cpu_state);
        println!("Flags: NV-BDIZC");
        println!("       {:08b}", self.cpu_state.regs.status);
    }
}

impl CPU for CpuInterpreter {

    fn clock(&mut self, data: Option<u8>) -> BusMessage {

        use CpuInterpreterState::*;
        use BusMessage::*;

        self.emu_state.instruction_done = false;
        self.emu_state.total_cycles += 1;
        self.emu_state.op_cycle += 1;

        if let Some(data) = data {
            self.cpu_state.data = data;
        }

        // increase the cycle lenght of the current instruction if the current instruction requires it
        if self.cpu_state.extra_cycle {
            self.emu_state.additional_cycles += 1;
            self.cpu_state.extra_cycle = false;
        }

        // CPU state machine

        loop {
            match self.exec_state {
                CpuInterpreterState::Fetch => {
                    match self.emu_state.op_cycle {
                        1 => {
                            self.cpu_state.op = data.expect("Bus data can't be empty in fetch cycle 1");
                            self.instruction = Some(
                                Opcode::from_u8(self.cpu_state.op)
                                    .expect("Illegal Opcode")
                                    .to_instruction()
                            );
                            self.cpu_state.regs.pc += 1;
                        },
                        2 => {
                            self.cpu_state.o1 = data.expect("Bus data can't be empty in fetch cycle 2");
                            self.cpu_state.regs.pc += 1;
                        },
                        3 => {
                            self.cpu_state.o2 = data.expect("Bus data can't be empty in fetch cycle 3");
                            self.cpu_state.regs.pc += 1;
                        },
                        _ => panic!("Fetch state can't take longer than 3 cycles"),
                    }

                    if self.emu_state.op_cycle < self.instruction.expect("CPU::instruction cant be None after decoding").bytes {
                        return Read{addr: self.cpu_state.regs.pc};
                    } else {
                        self.exec_state = Addressing;
                    }
                }
                CpuInterpreterState::Addressing => {
                    self.addr_cycle += 1;

                    let instruction = self.instruction
                                        .expect("CPU::instruction is None, this should be impossible at this point");

                    match (instruction.addr_delegate)(&mut self.cpu_state, self.addr_cycle) {
                        AddrDelegateReturn::Yield(msg) => {
                            return msg;
                        }
                        AddrDelegateReturn::Return(operand) => {
                            self.operand = Some(operand);
                            self.exec_state = Execute;
                            continue;
                        }
                    }
                }
                CpuInterpreterState::Execute => {
                    self.exec_cycle += 1;
                    
                    let instruction = self.instruction
                                        .expect("CPU::instruction is None, this should be impossible at this point");

                    let operand = self.operand
                                        .as_ref()
                                        .expect("CPU::operand can't be None after addressing");
                    
                    let msg: BusMessage = match operand {
                        Operand::Implied => {
                            instruction.op_delegate.implied().unwrap()(&mut self.cpu_state, self.exec_cycle)
                        }
                        Operand::Immediate(imm) => {
                            instruction.op_delegate.immediate().unwrap()(&mut self.cpu_state, *imm, self.exec_cycle)
                        }
                        Operand::Address(addr) => {
                            instruction.op_delegate.address().unwrap()(&mut self.cpu_state, *addr, self.exec_cycle)
                        }
                    };

                    if self.emu_state.op_cycle < instruction.cycles + self.emu_state.additional_cycles || self.cpu_state.extra_cycle {
                        return msg;
                    } else {
                        // We're done with this instruction, prepare the next one!
                        self.emu_state.op_cycle = 0;
                        self.addr_cycle = 0;
                        self.exec_cycle = 0;
                        self.emu_state.additional_cycles = 0;
                        self.emu_state.instruction_done = true;

                        // read next instruction or handle interrupt request
                        match self.emu_state.interrupt_request {
                            Interrupt::None => {
                                self.instruction = None;
                                self.operand = None;
                                self.exec_state = Fetch;
                            }
                            Interrupt::Irq(interrupt_vector) => {
                                self.instruction = Some(&instructions::IRQ_INSTRUCTION);
                                self.operand = Some(Operand::Address(interrupt_vector));
                                self.exec_state = Execute;
                                self.emu_state.interrupt_request = Interrupt::None;
                            }
                            Interrupt::Nmi(interrupt_vector) => {
                                self.instruction = Some(&instructions::NMI_INSTRUCTION);
                                self.operand = Some(Operand::Address(interrupt_vector));
                                self.exec_state = Execute;
                                self.emu_state.interrupt_request = Interrupt::None;
                            }
                        }

                        return Read{addr: self.cpu_state.regs.pc};
                    }
                }
                CpuInterpreterState::Halt => {
                    println!("CPU is halted");
                    return Nop;
                }
            }
        }
    }

    fn irq(&mut self) {
        if !self.cpu_state.regs.get_flag(CpuFlags::I) && self.emu_state.interrupt_request == Interrupt::None {
            self.emu_state.interrupt_request = Interrupt::Irq(0xFFFE);
        }
    }

    fn nmi(&mut self) {
        self.emu_state.interrupt_request = Interrupt::Nmi(0xFFFA);
    }

    fn reset(&mut self) {
        // zero internal interpreter state
        self.emu_state = EmulationState::default();
        self.addr_cycle = 0;
        self.exec_cycle = 0;

        // bit 3 of status always high
        self.cpu_state.regs.status = 0x20;
        // set IRQ disable flag
        self.cpu_state.regs.set_flag(CpuFlags::I, true);

        // dispatch reset pseudo-instruction with the reset vector address as operand
        self.exec_state = CpuInterpreterState::Execute;
        self.operand = Some(Operand::Address(0xFFFC));
        self.instruction = Some(&instructions::RESET_INSTRUCTION);
    }
}

impl CpuDebugger for CpuInterpreter {
    fn get_cpu_regs(&self) -> &CpuRegisters {
        &self.cpu_state.regs
    }

    fn get_cup_regs_mut(&mut self) -> Option<&mut CpuRegisters> {
        if self.emu_state.instruction_done {
            Some(&mut self.cpu_state.regs)
        } else {
            None
        }
    }

    fn get_emulation_state(&self) -> &EmulationState {
        &self.emu_state
    }

    fn get_instruction(&self) -> (Option<&Instruction>, Option<&Operand>) {
        (self.instruction, self.operand.as_ref())
    }
}
