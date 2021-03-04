use crate::{BusMessage, cpu::{AddrDelegateReturn, CpuState, Operand}};

pub fn acc(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    AddrDelegateReturn::Return(Operand::Implied)
}

pub fn imm(s: &mut CpuState, _cycle: u8) -> AddrDelegateReturn {
    AddrDelegateReturn::Return(Operand::Immediate(s.o1))
}

pub fn abs(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    s.addr = s.o1 as u16;
    s.addr |= (s.o2 as u16) << 8;

    AddrDelegateReturn::Return(Operand::Address(s.addr))
}

pub fn zp(s: &mut CpuState, _cycle: u8) -> AddrDelegateReturn {
    s.addr = s.o1 as u16;
    AddrDelegateReturn::Return(Operand::Address(s.addr))
}

pub fn zp_x(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    match cycle {
        1 => AddrDelegateReturn::Yield(BusMessage::Nop),
        2 => {
            s.addr = (s.o1 + s.regs.x) as u16;
            AddrDelegateReturn::Return(Operand::Address(s.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn zp_y(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    match cycle {
        1 => AddrDelegateReturn::Yield(BusMessage::Nop),
        2 => {
            s.addr = (s.o1 + s.regs.y) as u16;
            AddrDelegateReturn::Return(Operand::Address(s.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn abs_x(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    s.addr = s.o1 as u16;
    s.addr |= (s.o2 as u16) << 8;
    s.addr += s.regs.x as u16;

    AddrDelegateReturn::Return(Operand::Address(s.addr))
}

pub fn abs_y(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    s.addr = s.o1 as u16;
    s.addr |= (s.o2 as u16) << 8;
    s.addr += s.regs.y as u16;

    AddrDelegateReturn::Return(Operand::Address(s.addr))
}

pub fn imp(_s: &mut CpuState, _cycle: u8) -> AddrDelegateReturn {
    AddrDelegateReturn::Return(Operand::Implied)
}

pub fn rel(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    let offset = s.o1 as i8;
    let mut new_pc = s.regs.pc as i32;
    new_pc += offset as i32;

    AddrDelegateReturn::Return(Operand::Address(new_pc as u16))
}

pub fn ind_x(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    match cycle {
        1 => {
            // fetch zero page address
            let jmpaddr = (s.regs.x + s.o1) as u16;
            AddrDelegateReturn::Yield(BusMessage::Read{addr: jmpaddr})
        }
        2 => {
            // set new addr LSB
            s.addr = s.data as u16;

            // fetch new PC MSB at operand+1
            let jmpaddr = (s.regs.x + s.o1 + 1) as u16;
            AddrDelegateReturn::Yield(BusMessage::Read{addr: jmpaddr})
        }
        3 => {
            // set new addr MSB
            s.addr |= (s.data as u16) << 8;

            // done
            AddrDelegateReturn::Return(Operand::Address(s.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn ind_y(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    match cycle {
        1 => {
            // fetch LSB of address from zero page
            AddrDelegateReturn::Yield(BusMessage::Read{addr: s.o1 as u16})
        }
        2 => {
            // set new PC LSB
            s.addr = s.data as u16;

            // fetch MSB of address from zero page
            AddrDelegateReturn::Yield(BusMessage::Read{addr: (s.o1 + 1) as u16})
        }
        3 => {
            // set new PC MSB
            s.addr |= (s.data as u16) << 8;
            
            // add Y
            s.addr += s.regs.y as u16;

            // done
            AddrDelegateReturn::Return(Operand::Address(s.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn ind(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    match cycle {
        1 => {
            // fetch new PC LSB at operand
            let mut jmpaddr = s.o1 as u16;
            jmpaddr |= (s.o2 as u16) << 8;
            AddrDelegateReturn::Yield(BusMessage::Read{addr: jmpaddr})
        }
        2 => {
            // set new PC LSB
            s.addr = s.data as u16;

            // fetch new PC MSB at operand+1
            let mut jmpaddr = s.o1 as u16;
            jmpaddr |= (s.o2 as u16) << 8;
            AddrDelegateReturn::Yield(BusMessage::Read{addr: jmpaddr+1})
        }
        3 => {
            // set new PC MSB
            s.addr |= (s.data as u16) << 8;

            // done
            AddrDelegateReturn::Return(Operand::Address(s.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn abs_x_extra(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    match cycle {
        1 => {
            s.addr = s.o1 as u16;
            s.addr |= (s.o2 as u16) << 8;
            s.addr += s.regs.x as u16;
            
            // this instruction takes one additional cycle when the address crosses a page boundary
            if s.addr >> 8 != s.o2 as u16{
                s.extra_cycle = true;
                AddrDelegateReturn::Yield(BusMessage::Nop)
            } else {
                AddrDelegateReturn::Return(Operand::Address(s.addr))
            }
        }
        2 => {
            AddrDelegateReturn::Return(Operand::Address(s.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn abs_y_extra(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    match cycle {
        1 => {
            s.addr = s.o1 as u16;
            s.addr |= (s.o2 as u16) << 8;
            s.addr += s.regs.y as u16;
            
            // this instruction takes one additional cycle when the address crosses a page boundary
            if s.addr >> 8 != s.o2 as u16{
                s.extra_cycle = true;
                AddrDelegateReturn::Yield(BusMessage::Nop)
            } else {
                AddrDelegateReturn::Return(Operand::Address(s.addr))
            }
        }
        2 => {
            AddrDelegateReturn::Return(Operand::Address(s.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn ind_y_extra(s: &mut CpuState, cycle: u8) -> AddrDelegateReturn {
    match cycle {
        1 => {
            // fetch LSB of address from zero page
            AddrDelegateReturn::Yield(BusMessage::Read{addr: s.o1 as u16})
        }
        2 => {
            // set new PC LSB
            s.addr = s.data as u16;

            // fetch MSB of address from zero page
            AddrDelegateReturn::Yield(BusMessage::Read{addr: (s.o1 + 1) as u16})
        }
        3 => {
            // instructions needs an extra cycle if page boundary is crossed
            if s.addr >> 8 != s.data as u16 {
                s.extra_cycle = true;
            }

            // set new PC MSB
            s.addr |= (s.data as u16) << 8;
            
            // add Y
            s.addr += s.regs.y as u16;

            // done
            AddrDelegateReturn::Return(Operand::Address(s.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}
