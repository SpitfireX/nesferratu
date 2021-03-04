use crate::cartridge::Header;

pub trait Mapper {
    fn map_cpu(&self, meta: &Header, addr: u16) -> MappedAddress;
    fn map_ppu(&self, meta: &Header, addr: u16) -> MappedAddress;
}

pub enum MappedAddress {
    ChrRam(u16),
    ChrRom(u16),
    PrgRam(u16),
    PrgRom(u16),
    None,
}

pub fn map_mapper(mapper_id: u16) -> Option<Box<dyn Mapper>> {
    match mapper_id {
        000 => Some(Box::new(Mapper000{})),
        _ => None,
    }
}

pub struct Mapper000 {}

impl Mapper for Mapper000 {
    fn map_cpu(&self, meta: &Header, addr: u16) -> MappedAddress {
        match addr {
            x if x >= 0xC000 => {
                if meta.prg_rom_size > 0x4000 {
                    MappedAddress::PrgRom(addr - 0x8000)
                } else {
                    MappedAddress::PrgRom(addr - 0xC000) // mirror first half of ROM
                }
            }
            x if x >= 0x8000 => {
                MappedAddress::PrgRom(addr - 0x8000)
            }
            x if x >= 0x6000 => {
                MappedAddress::PrgRam(addr - 0x6000)
            }
            _ => MappedAddress::None,
        }
    }

    fn map_ppu(&self, meta: &Header, addr: u16) -> MappedAddress {
        todo!()
    }
}