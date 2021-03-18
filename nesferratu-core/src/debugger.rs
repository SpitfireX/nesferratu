use rustyline::error::ReadlineError;
use rustyline::{Editor, Result as RustylineResult};

use std::{fmt::Display, io};

use crate::{Emulator, cpu::{CpuRegisters, EmulationState}};
use crate::cpu::instructions::{Instruction, Operand};

pub trait CpuDebugger {
    fn get_cpu_regs(&self) -> &CpuRegisters;
    fn get_cup_regs_mut(&mut self) -> Option<&mut CpuRegisters>;
    fn get_emulation_state(&self) -> &EmulationState;
    fn get_instruction(&self) -> (Option<&Instruction>, Option<&Operand>);
}

pub trait MemDebugger {
    fn get_mem(&self) -> &[u8];
    fn get_mem_mut(&mut self) -> &mut [u8];
}

enum Arg {

}

type CommandDelegate = fn(d: &mut Debugger, args: &Vec<Arg>) -> bool;

pub struct Command {
    name: String,
    delegate: CommandDelegate,
    args: Vec<Arg>,
}

impl Command {
    pub fn parse(input: &str) -> Result<Vec<Self>, CommandParseError> {
        Err(CommandParseError::Loljk)
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub enum CommandParseError {
    Loljk
}

pub struct Debugger {
    emu: Emulator,
    commands: Vec<Command>,
    last_command: Option<Command>,
}

impl Debugger {

    pub fn new(emu: Emulator) -> Debugger {
        Debugger {
            emu,
            commands: Vec::new(),
            last_command: None,
        }
    }

    pub fn add_cmds(&mut self, cmds: Vec<Command>) {
        self.commands.extend(cmds);
    }

    pub fn run(&mut self) {
        let mut rl = Editor::<()>::new();
        
        if rl.load_history("debugger_history.txt").is_err() {
            println!("No previous history.");
        }
        
        loop {
            if self.commands.len() > 0 {

            } else {
                match rl.readline(&self.format_prompt()) {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                            match Command::parse(&line) {
                            Ok(cmds) => {
                                self.add_cmds(cmds);
                            }
                            Err(_) => eprintln!("Could not parse command \"{}\"", line),
                        }
                    },
                    Err(ReadlineError::Interrupted) => {
                        break;
                    },
                    Err(ReadlineError::Eof) => {
                        break;
                    },
                    Err(err) => {
                        println!("Readline Error: {:?}, exiting", err);
                        break;
                    }
                }
            }
        }
        
        rl.save_history("debugger_history.txt").unwrap();
    }

    fn format_prompt(&mut self) -> String {
        match self.last_command.as_ref() {
            Some(cmd) => format!("[{}] >> ", cmd),
            None => String::from(">> "),
        }
    }
}



pub fn hex_print(bytes: &[u8], start: usize, len: usize, title_text: Option<&str>) {
    let len = if len > bytes.len() - start as usize {
        bytes.len() - start as usize
    } else {
        len
    };

    let title = title_text.unwrap_or_default();

    println!("┌──────┬─────────────────────────────────────────────────┬──────────────────┐");
    println!("│ Hex  │ 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F │ {:^16.16} │", title);
    println!("├──────┼─────────────────────────────────────────────────┼──────────────────┤");
    let mut col = 0;
    let mut s = String::new();

    for (i, b) in bytes[start as usize .. start as usize + len].iter().enumerate() {
        if i % 0x10 == 0 {
            print!("│ {:04X} │ ", start as usize + i);
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