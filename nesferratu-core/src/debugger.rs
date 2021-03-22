use rustyline::error::ReadlineError;
use rustyline::Editor;
use regex::Regex;

use std::fmt::Display;

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

lazy_static! {
    static ref CMD_REGEXES: Vec<(CommandDelegate, Regex, usize)> = vec![
        (commands::cycle, Regex::new(r"c(?:ycle)?").unwrap(), 1),
        (commands::step, Regex::new(r"s(?:tep)?").unwrap(), 1),
    ];

    static ref ARG_UINT: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug, Clone)]
pub enum Arg {
    UInt(u32),
    String(String),
}

impl Arg {
    fn parse(s: &str) -> Option<Self> {
        match s {
            s if ARG_UINT.is_match(s) => {
                match s.parse() {
                    Ok(i) => Some(Self::UInt(i)),
                    Err(_) => None,
                }
            },

            _ => Some(Self::String(s.to_owned())),
        }
    }
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arg::UInt(i) => write!(f, "{}", i),
            Arg::String(s) => write!(f, "{}", s)
        }
    }
}

type CommandDelegate = fn(d: &mut Debugger, args: &Vec<Arg>) -> Result<(), CommandRunError>;

pub enum CommandParseError {
    EmptyInput,
    UnknownCommand,
    InvalidArgument(usize), // index of invalid argument
    InvalidArgumentNum(usize, usize), // expected number, actual number
}

pub enum CommandRunError {
    InvalidArgumentType(usize, Arg, Arg), // index of invalid argument, expected type, actual type
}

pub struct Command {
    cmd: String,
    delegate: CommandDelegate,
    args: Vec<Arg>,
}

impl Command {
    pub fn parse(mut input: &str) -> Result<Vec<Self>, CommandParseError> {

        if input.len() == 0 {
            return Err(CommandParseError::EmptyInput);
        }

        let mut cmd: Option<Command> = None;
        let mut argnum = 0;
        
        for (delegate, regex, num) in CMD_REGEXES.iter() {

            if let Some(m) = regex.find(input) {
                input = &input[m.end()..];
                
                argnum = *num;
                cmd = Some(Command {
                    cmd: m.as_str().to_owned(),
                    delegate: *delegate,
                    args: Vec::new(),
                });
                break;
            }
        }

        match cmd {
            None => Err(CommandParseError::UnknownCommand),
            Some(mut cmd) => {

                for (i, token) in input.split_ascii_whitespace().enumerate() {

                    if i < argnum {
                        if let Some(arg) = Arg::parse(token) {
                            cmd.args.push(arg);
                        } else {
                            return Err(CommandParseError::InvalidArgument(i));
                        }
                    } else {
                        return Err(CommandParseError::InvalidArgumentNum(argnum, i+1));
                    }
                }

                Ok(vec!(cmd))
            }
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cmd);
        for arg in &self.args {
            write!(f, " {}", arg);
        }
        Ok(())
    }
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
                let cmd = self.commands.pop().unwrap();
                
                // run actual debugger command
                match (cmd.delegate)(self, &cmd.args) {
                    Ok(_) => {
                        self.last_command = Some(cmd);
                    }
                    Err(e) => {
                        eprintln!("Could not run command \"{}\"", cmd);
                        match e {
                            CommandRunError::InvalidArgumentType(i, exp, got) => eprintln!("Invalid argument type at position {}, expected {:?}, got {:?}", i+1, exp, got),
                        }
                    }
                }

            } else {
                match rl.readline(&self.format_prompt()) {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                            match Command::parse(&line) {
                            Ok(cmds) => {
                                self.add_cmds(cmds);
                            }
                            Err(e) => {
                                match e {
                                    // put last command back into the queue when no new command is given
                                    CommandParseError::EmptyInput => {
                                        if let Some(cmd) = self.last_command.take() {
                                            self.commands.push(cmd);
                                        }
                                    },
                                    CommandParseError::UnknownCommand => eprintln!("Unknown command"),
                                    CommandParseError::InvalidArgument(i) => eprintln!("Invalid argument at position {}", i+1),
                                    CommandParseError::InvalidArgumentNum(exp, got) => eprintln!("Invalid number of arguments: expected {}, got {}", exp, got),
                                }
                            }
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

mod commands {
    use crate::debugger::{Debugger, Arg, CommandRunError, CpuDebugger};

    pub fn cycle(d: &mut Debugger, args: &Vec<Arg>) -> Result<(), CommandRunError> {
        let cycles;

        if args.len() == 0 {
            cycles = 1;
        } else {
            if let Arg::UInt(i) = args[0] {
                cycles = i;
            } else {
                return Err(CommandRunError::InvalidArgumentType(0, Arg::UInt(0), args[0].clone()));
            }
        }

        for _ in 0..cycles {
            d.emu.clock();
        }

        Ok(())
    }

    pub fn step(d: &mut Debugger, args: &Vec<Arg>) -> Result<(), CommandRunError> {
        let steps;

        if args.len() == 0 {
            steps = 1;
        } else {
            if let Arg::UInt(i) = args[0] {
                steps = i;
            } else {
                return Err(CommandRunError::InvalidArgumentType(0, Arg::UInt(0), args[0].clone()));
            }
        }

        for _ in 0..steps {
            d.emu.clock();

            while !d.emu.cpu.get_emulation_state().instruction_done {
                d.emu.clock();
            }
        }

        Ok(())
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