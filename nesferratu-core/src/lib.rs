#![feature(destructuring_assignment)]

#[macro_use]
extern crate num_derive;

use cpu::{BusMessage, CPU, CPUInterpreter};

pub mod cpu;

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
            ram: [0x00; 64*1024] // 0xa9 = LDA_imm opcode
        };

        // reset vector
        new.write(0xFFFC, 0x37);
        new.write(0xFFFD, 0x13);

        // program at 0x1337
        new.write(0x1337, 0xA9);
        new.write(0x1338, 0xFF);

        new
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