extern crate num_derive;
#[macro_use]
extern crate lazy_static;

pub mod cpu;
pub mod cartridge;
pub mod debugger;

use cpu::{CPU, CpuInterpreter};
use cartridge::Cartridge;
use debugger::MemDebugger;

#[derive(Debug)]
pub enum BusMessage {
    Read {addr: u16},
    Write {addr: u16, data: u8},
    Nop,
}

pub struct Emulator {
    fetch: Option<u8>,
    cpu: CpuInterpreter,
    memory: Ram,
    cartridge: Cartridge,
}

impl Emulator {

    pub fn new(cartridge: Cartridge) -> Emulator {
        let mut temp = Emulator {
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
    }

    fn read_cpu(&self, addr: u16) -> Option<u8> {
        match addr {
            // $0000-$1FFF RAM, 2KB mirrored 4 times
            addr if addr < 0x2000 => {
                Some(self.memory.cpu_read(addr % 0x800))
            }
            // $2000-$3FFF PPU registers, 8 byte mirrored several times
            addr if addr < 0x4000 => {
                println!("PPU not implemented yet (read {:#04X})", addr);
                Some(0)
            }
            // $4000-$4017 APU / IO
            addr if addr < 0x4018 => {
                println!("APU and IO not implemented yet (read {:#04X})", addr);
                Some(0)
            }
            // $4018-$401F CPU Test Mode stuff
            addr if addr < 0x4020 => {
                println!("CPU Test Mode stuff not implemented yet (read {:#04X})", addr);
                Some(0)
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
                println!("PPU not implemented yet (write {:#04X}, {:#02X})", addr, data);
            }
            // $4000-$4017 APU / IO
            addr if addr < 0x4018 => {
                println!("APU and IO not implemented yet (write {:#04X}, {:#02X})", addr, data);
            }
            // $4018-$401F CPU Test Mode stuff
            addr if addr < 0x4020 => {
                println!("CPU Test Mode stuff not implemented (write {:#04X}, {:#02X})", addr, data);
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

impl MemDebugger for Ram {
    fn get_mem(&self) -> &[u8] {
        &self.ram
    }

    fn get_mem_mut(&mut self) -> &mut [u8] {
        &mut self.ram
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