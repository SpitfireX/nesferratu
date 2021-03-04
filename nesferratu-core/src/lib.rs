extern crate num_derive;

use cpu::{CPU, CPUInterpreter};
use cartridge::Cartridge;

pub mod cpu;
pub mod cartridge;

#[derive(Debug)]
pub enum BusMessage {
    Read {addr: u16},
    Write {addr: u16, data: u8},
    Nop,
}

pub struct Bus {
    fetch: Option<u8>,
    cpu: CpuInterpreter,
    memory: Ram,
    cartridge: Cartridge,
}

impl Bus {

    pub fn new(cartridge: Cartridge) -> Bus {
        let mut temp = Bus {
            fetch: None,
            cpu: CpuInterpreter::new(),
            memory: Ram::new(),
            cartridge,
        };
        temp.cpu.reset();
        temp
    }

    pub fn clock(&mut self) {
        let msg = self.cpu.clock(self.fetch);

        match msg {
            BusMessage::Read { addr } => {
                self.fetch = self.read_cpu(addr);
            }
            BusMessage::Write { addr, data } => {
                self.fetch = None;
                self.write_cpu(addr, data);
            }
            BusMessage::Nop => {
                self.fetch = None;
            }
        }

        // println!("Program:");
        // self.memory.prettyprint(0x1337, 0x50);

        println!("Zero Page:");
        self.memory.prettyprint(0x0000, 0x100);

        println!("Stack:");
        self.memory.prettyprint(0x01E0, 0x20);
    }

    fn read_cpu(&self, addr: u16) -> Option<u8> {
        match addr {
            // $0000-$1FFF RAM, 2KB mirrored 4 times
            addr if addr < 0x2000 => {
                Some(self.memory.cpu_read(addr % 0x800))
            }
            // $2000-$3FFF PPU registers, 8 byte mirrored several times
            addr if addr < 0x4000 => {
                // panic!("PPU not implemented yet");
                Some(0)
            }
            // $4000-$4017 APU / IO
            addr if addr < 0x4018 => {
                panic!("APU and IO not implemented yet");
            }
            // $4018-$401F CPU Test Mode stuff
            addr if addr < 0x4020 => {
                panic!("CPU Test Mode stuff not implemented yet");
            }
            // $4020-$FFFF Cartridge
            _ => {
                Some(self.cartridge.cpu_read(addr))
            }
        }
    }

    fn write_cpu(&mut self, addr: u16, data: u8) {
        match addr {
            // $0000-$1FFF RAM, 2KB mirrored 4 times
            addr if addr < 0x2000 => {
                self.memory.cpu_write(addr % 0x800, data);
            }
            // $2000-$3FFF PPU registers, 8 byte mirrored several times
            addr if addr < 0x4000 => {
                panic!("PPU not implemented yet");
            }
            // $4000-$4017 APU / IO
            addr if addr < 0x4018 => {
                panic!("APU and IO not implemented yet");
            }
            // $4018-$401F CPU Test Mode stuff
            addr if addr < 0x4020 => {
                panic!("CPU Test Mode stuff not implemented yet");
            }
            // $4020-$FFFF Cartridge
            _ => {
                self.cartridge.cpu_write(addr, data);
            }
        }
    }
}

trait CpuBusDevice {
    fn cpu_read(&self, addr: u16) -> u8;
    fn cpu_write(&mut self, addr: u16, data: u8);
}

trait PpuBusDevice {
    fn ppu_read(&self, addr: u16) -> u8;
    fn ppu_write(&mut self, addr: u16, data: u8);
}

struct Ram {
    ram: [u8; 2048]
}

impl Ram {
    fn new() -> Ram {
        Ram{
            ram: [0x00; 2048]
        }
    }
}

impl Ram {
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

impl CpuBusDevice for Ram {

    fn cpu_read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn cpu_write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}