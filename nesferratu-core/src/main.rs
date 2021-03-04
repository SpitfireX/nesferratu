extern crate clap;
use clap::{Arg, App};

use nesferratu_core::Bus;
use nesferratu_core::cartridge::Cartridge;

use std::io;
use std::io::Result as IoResult;

fn main() -> IoResult<()> {

    let matches = App::new("NESferratu CLI")
        .version("0.1-turboalpha")
        .author("SpitfireX")
        .about("Tries (and and by try i mean it really tries its best) to emulate a NES.")
        .arg(Arg::with_name("debug")
            .short("d")
            .help("If set the emulator starts in debugging mode"))
        .arg(Arg::with_name("ROM")
            .required(true)
            .index(1)
            .help("The ROM file to load"))
        .get_matches();
    
        println!("{:?}", matches);

    let cartridge = Cartridge::read_from_file(matches.value_of("ROM").unwrap())?;
    let mut bus = Bus::new(cartridge);
    
    if matches.is_present("debug") {
        loop {
            bus.clock();
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            println!();
        }
    } else {
        loop {
            bus.clock();
            println!();
        }
    }
}