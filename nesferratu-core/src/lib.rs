#[macro_use]
extern crate num_derive;

use cpu::{CPU, CPUInterpreter};

pub mod cpu;

#[derive(Debug)]
pub enum BusMessage {
    Read {addr: u16},
    Write {addr: u16, data: u8},
    Nop,
}

pub struct Bus {
    fetch: Option<u8>,
    cpu: CPUInterpreter,
    memory: Memory,
}

impl Bus {

    pub fn new() -> Bus {
        let mut temp = Bus {
            fetch: None,
            cpu: CPUInterpreter::new(),
            memory: Memory::new(),
        };
        temp.cpu.reset();
        temp
    }

    pub fn clock(&mut self) {
        let msg = self.cpu.clock(self.fetch);

        match msg {
            BusMessage::Read { addr } => {
                self.fetch = self.read(addr);
            }
            BusMessage::Write { addr, data } => {
                self.fetch = None;
                self.write(addr, data);
            }
            BusMessage::Nop => {
                self.fetch = None;
            }
        }

        println!("Program:");
        self.memory.prettyprint(0x1337, 0x50);

        println!("Zero Page:");
        self.memory.prettyprint(0x0000, 0x100);
    }

    fn read(&self, addr: u16) -> Option<u8> {
        Some(self.memory.read(addr))
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.memory.write(addr, data);
    }
}

trait BusDevice {
    fn read(&self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, data: u8);
}

struct Memory {
    ram: [u8; 64*1024]
}

impl Memory {
    fn new() -> Memory {
        let mut new = Memory{
            ram: [0x00; 64*1024]
        };
        let entrypoint: u16 = 0x1337;
        let program = [
            0xA9u8, // LDA imm
            b'h',
            0x85, // STA zp
            0x00,
            0xA9, // LDA imm
            b'e',
            0x85, // STA zp
            0x01,
            0xA9, // LDA imm
            b'l',
            0x85, // STA zp
            0x02,
            0xA9, // LDA imm
            b'l',
            0x85, // STA zp
            0x03,
            0xA9, // LDA imm
            b'o',
            0x85, // STA zp
            0x04,
            0xA9, // LDA imm
            b' ',
            0x85, // STA zp
            0x05,
            0xA9, // LDA imm
            b'w',
            0x85, // STA zp
            0x06,
            0xA9, // LDA imm
            b'o',
            0x85, // STA zp
            0x07,
            0xA9, // LDA imm
            b'r',
            0x85, // STA zp
            0x08,
            0xA9, // LDA imm
            b'l',
            0x85, // STA zp
            0x09,
            0xA9, // LDA imm
            b'd',
            0x85, // STA zp
            0x0A,
        ];
        
        // set reset vector
        new.write(0xFFFC, entrypoint as u8);
        new.write(0xFFFD, (entrypoint >> 8) as u8);

        // tranfer program to the entry point
        let i = entrypoint as usize;
        new.ram[i .. i + program.len()].clone_from_slice(&program);

        new
    }
}

impl Memory {
    fn prettyprint(&self, addr: u16, len: usize) {
        let len = if len > self.ram.len() - addr as usize {
            self.ram.len() - addr as usize
        } else {
            len
        };

        println!("┌──────┬─────────────────────────────────────────────────┐");
        println!("│ RAM  │ 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F │");
        println!("├──────┼─────────────────────────────────────────────────┼──────────────────┐");
        let mut col = 0;
        let mut s = String::new();

        for (i, b) in self.ram[addr as usize .. addr as usize + len].iter().enumerate() {
            if i % 0x10 == 0 {
                print!("│ {:04X} │ ", addr as usize + i);
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
}

impl BusDevice for Memory {

    fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}