pub mod mappers;

use mappers::{Mapper, MappedAddress};

use std::{io::{Result as IoResult, Read, Error, ErrorKind}, path::Path};
use std::fs::File;

use crate::BusDevice;

// iNES / NES2.0 header
pub struct Header {
    is_nes20: bool,
    prg_rom_size: usize,
    chr_rom_size: usize,
    mapper_id: u16,
    trainer_present: bool,
    persistent_memory_present: bool,
    prg_ram_size: usize,
    chr_ram_size: usize,
}

pub struct Cartridge {
    header: Header,
    mapper: Box<dyn Mapper>,
    trainer: Option<Box<[u8]>>,
    prg_rom: Box<[u8]>,
    chr_rom: Box<[u8]>,
    misc_rom: Box<[u8]>, 
}

impl Cartridge {

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> IoResult<Cartridge> {
        let file = File::open(path)?;
        Self::read(file)
    }

    pub fn read<R: Read>(mut reader: R) -> IoResult<Cartridge> {
        let mut raw_header = [0u8; 16];
        reader.read(&mut raw_header)?;
        

        // check if file is valid iNES
        if !(raw_header[0..4] == *"NES\u{1A}".as_bytes()) {
            return Err(Error::new(ErrorKind::InvalidData, "Input Data is not in iNES format"));
        }

        // parse header
        let header = {
            // check if file is maybe actually in NES 2.0 format
            // header[7] must be xxxx 10xx
            let is_nes20 = raw_header[7] & 0xC == 0x8;

            let mut prg_rom_size = raw_header[4] as usize;
            let mut chr_rom_size = raw_header[5] as usize;
            let mapper = (raw_header[6] >> 4 | raw_header[7] & 0xF0) as u16;
            let trainer = (raw_header[6] >> 2) == 1;
            let persistent_memory = (raw_header[6] >> 1) == 1;
            let mut prg_ram_size = 0;
            let chr_ram_size;

            if is_nes20 {
                // prg rom size is more complicated
                let b9 = raw_header[9] & 0xF0;

                if b9 != 0xF0 {
                    // literal notation
                    prg_rom_size |= (b9 as usize) << 4;
                } else {
                    // exponent-multiplier notation
                    let mul = prg_rom_size & 0x3;
                    let exp = prg_rom_size >> 2;

                    prg_rom_size = 2usize.pow(exp as u32) * (mul * 2 + 1);
                }

                // chr rom size aswell
                let b9 = raw_header[9] & 0x0F;

                if b9 != 0x0F {
                    // literal notation
                    chr_rom_size |= (b9 as usize) << 8
                } else {
                    // exponent-multiplier notation
                    let mul = chr_rom_size & 0x3;
                    let exp = chr_rom_size >> 2;

                    chr_rom_size = 2usize.pow(exp as u32) * (mul * 2 + 1);
                }

                // prg ram size
                prg_ram_size = 64 << (raw_header[10] & 0x0F);

                // chr ram size
                chr_ram_size = 64 << (raw_header[11] & 0x0F);


            } else {
                prg_rom_size *= 16384;
                chr_rom_size *= 8192 ;

                if persistent_memory {
                    prg_ram_size = 8192;
                }

                chr_ram_size = 8192;
            }

            Header {
                is_nes20,
                prg_rom_size,
                chr_rom_size,
                mapper_id: mapper,
                trainer_present: trainer,
                persistent_memory_present: persistent_memory,
                prg_ram_size,
                chr_ram_size,
            }
        };

        // get mapper instance
        let mapper = mappers::map_mapper(header.mapper_id).expect("Unimplemented Mapper");

        // read trainer, if present
        let trainer: Option<Box<[u8]>> = if header.trainer_present {
            let mut data = [0u8; 512];
            reader.read_exact(&mut data)?;
            Some(Box::new(data))
        } else {
            None
        };

        // read prg rom
        let mut prg_rom = vec![0u8; header.prg_rom_size];
        reader.read_exact(&mut prg_rom)?;

        //read chr rom
        let mut chr_rom = vec![0u8; header.chr_rom_size];
        reader.read_exact(&mut chr_rom)?;

        let mut misc_rom = Vec::new();
        reader.read_to_end(&mut misc_rom)?;

        Ok(
            Cartridge {
                header,
                mapper,
                trainer,
                prg_rom: prg_rom.into_boxed_slice(),
                chr_rom: chr_rom.into_boxed_slice(),
                misc_rom: misc_rom.into_boxed_slice(),
            }
        )
    } 
}

impl BusDevice for Cartridge {
    fn read(&self, addr: u16) -> u8 {
        match self.mapper.map_cpu(&self.header, addr) {
            MappedAddress::ChrRam(_) => {
                panic!("The fuck is a cartridge RAM?");
            }
            MappedAddress::ChrRom(_) => {
                panic!("This should not be happening aaAAaaaaAA spaghet");
            }
            MappedAddress::PrgRam(_) => {
                panic!("The fuck is a cartridge RAM?");
            }
            MappedAddress::PrgRom(addr) => {
                self.prg_rom[addr as usize]
            }
            MappedAddress::None => 0x00,
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match self.mapper.map_cpu(&self.header, addr) {
            MappedAddress::ChrRam(_) => {
                panic!("The fuck is a cartridge RAM?");
            }
            MappedAddress::ChrRom(_) => {
                panic!("This should not be happening aaAAaaaaAA spaghet");
            }
            MappedAddress::PrgRam(_) => {
                panic!("The fuck is a cartridge RAM?");
            }
            MappedAddress::PrgRom(addr) => {
                self.prg_rom[addr as usize] = data;
            }
            MappedAddress::None => {},
        }
    }
}