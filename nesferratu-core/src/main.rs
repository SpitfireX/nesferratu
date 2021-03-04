extern crate clap;
use clap::{Arg, App};

use nesferratu_core::{Bus};

use std::io;

fn main() {

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

    let mut bus = Bus::new();
    loop {
        bus.clock();
        
        if matches.is_present("step") {
            let mut input = String::new();
            io::stdin().read_line(&mut input);
        }
        println!();
    }
}