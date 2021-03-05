use std::io;

use crate::{Bus, cpu::{CpuRegisters, EmulationState}};
use crate::cpu::instructions::{Instruction, Operand};

pub trait CpuDebugger {
    fn get_cpu_regs(&self) -> &CpuRegisters;
    fn get_cup_regs_mut(&mut self) -> Option<&mut CpuRegisters>;
    fn get_emulation_state(&self) -> &EmulationState;
    fn get_instruction(&self) -> (Option<&Instruction>, Option<&Operand>);
}

pub trait MemDebugger {
    fn get_mem(&self) -> &[u8];
    fn get_mem_mut(&mut self) -> &mut [u8];
}

pub struct Debugger {
    emu: Bus,
}

impl Debugger {

    pub fn new(emu: Bus) -> Debugger {
        Debugger {
            emu
        }
    }

    pub fn add_cmd(&mut self, cmd: &str) {
        
    }

    pub fn run(&mut self) {
        loop {
            if self.step() {
                self.emu.clock();
            } else {
                break;
            }
        }
    }

    fn step(&mut self) -> bool {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        println!();
        true
    }
}



pub fn hex_print(bytes: &[u8], start: usize, len: usize, title_text: Option<&str>) {
    let len = if len > bytes.len() - start as usize {
        bytes.len() - start as usize
    } else {
        len
    };

    let title = title_text.unwrap_or_default();

    println!("┌──────┬─────────────────────────────────────────────────┬──────────────────┐");
    println!("│ Hex  │ 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F │ {:^16.16} │", title);
    println!("├──────┼─────────────────────────────────────────────────┼──────────────────┤");
    let mut col = 0;
    let mut s = String::new();

    for (i, b) in bytes[start as usize .. start as usize + len].iter().enumerate() {
        if i % 0x10 == 0 {
            print!("│ {:04X} │ ", start as usize + i);
        }
        
        print!("{:02X} ", b);
        
        let c = *b as char;
        if c.is_ascii_graphic() || c == ' ' {
            s.push(c);
        } else {
            s.push('.');
        }

        col = i & 0xF;
        if col == 0xF {
            println!("│ {} │", s);
            s.clear();
        }
    }

    if col < 0xF {
        while col < 0xF {
            print!("   ");
            col += 1;
        }
        while s.len() <= 0xF {
            s.push(' ');
        }
        println!("│ {} │", s);
    }

    println!("└──────┴─────────────────────────────────────────────────┴──────────────────┘");
}