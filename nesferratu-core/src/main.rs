extern crate clap;
use clap::{Arg, App};

use nesferratu_core::Emulator;
use nesferratu_core::cartridge::Cartridge;
use nesferratu_core::debugger;
use nesferratu_core::debugger::Command;

fn main() {

    let cli_args = App::new("NESferratu CLI")
        .version("0.1-turboalpha")
        .author("SpitfireX")
        .about("Tries (and and by try i mean it really tries its best) to emulate a NES. Written in Rust btw 🦀")
        .arg(Arg::with_name("debugger")
            .short("d")
            .takes_value(false)
            .help("If set the emulator runs with the debugger enabled"))
        .arg(Arg::with_name("cmd")
            .short("c")
            .long("debugger-cmd")
            .takes_value(true)
            .help("An initial command for the debugger. Multiple commands can be specified and must be separated by ';'"))
        .arg(Arg::with_name("ROM")
            .required(true)
            .index(1)
            .help("The ROM file to load"))
        .get_matches();

    let cartridge;

    match Cartridge::read_from_file(cli_args.value_of("ROM").unwrap()) {
        Ok(c) => {
            cartridge = c;
        }
        Err(e) => {
            panic!("Could not read ROM file: {}", e);
        }
    }

    let mut emu = Emulator::new(cartridge);
    
    if cli_args.is_present("debugger") {

        let mut debugger = debugger::Debugger::new(emu);
        if let Some(cmd) = cli_args.value_of("cmd") {
            match Command::parse(cmd) {
                Ok(cmds) => debugger.add_cmds(cmds),
                Err(_) => eprintln!("Could not parse initial debbugger command"),
            }
        }

        debugger.run();
        
    } else {
        loop {
            emu.clock();
        }
    }
}