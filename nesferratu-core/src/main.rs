#![feature(generators, generator_trait)]
use nesferratu_core::{Bus};

use std::io;

fn main() {
    let mut bus = Bus::new();
    loop {
        bus.clock();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input);
    }
}