use crate::{BusMessage, cpu::{AddrDelegateReturn, CPURegisters, Operand}};

pub fn acc(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    AddrDelegateReturn::Return(Operand::Implied)
}

pub fn imm(regs: &mut CPURegisters, _cycle: usize) -> AddrDelegateReturn {
    AddrDelegateReturn::Return(Operand::Immediate(regs.o1))
}

pub fn abs(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    regs.addr = regs.o1 as u16;
    regs.addr |= (regs.o2 as u16) << 8;

    AddrDelegateReturn::Return(Operand::Address(regs.addr))
}

pub fn zp(regs: &mut CPURegisters, _cycle: usize) -> AddrDelegateReturn {
    regs.addr = regs.o1 as u16;
    AddrDelegateReturn::Return(Operand::Address(regs.addr))
}

pub fn zp_x(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    match cycle {
        1 => AddrDelegateReturn::Yield(BusMessage::Nop),
        2 => {
            regs.addr = (regs.o1 + regs.x) as u16;
            AddrDelegateReturn::Return(Operand::Address(regs.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn zp_y(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    match cycle {
        1 => AddrDelegateReturn::Yield(BusMessage::Nop),
        2 => {
            regs.addr = (regs.o1 + regs.y) as u16;
            AddrDelegateReturn::Return(Operand::Address(regs.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn abs_x(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    regs.addr = regs.o1 as u16;
    regs.addr |= (regs.o2 as u16) << 8;
    regs.addr += regs.x as u16;

    AddrDelegateReturn::Return(Operand::Address(regs.addr))
}

pub fn abs_y(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    regs.addr = regs.o1 as u16;
    regs.addr |= (regs.o2 as u16) << 8;
    regs.addr += regs.y as u16;

    AddrDelegateReturn::Return(Operand::Address(regs.addr))
}

pub fn imp(_regs: &mut CPURegisters, _cycle: usize) -> AddrDelegateReturn {
    AddrDelegateReturn::Return(Operand::Implied)
}

pub fn rel(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    let offset = regs.o1 as i8;
    let mut new_pc = regs.pc as i32;
    new_pc -= offset as i32;

    AddrDelegateReturn::Return(Operand::Address(new_pc as u16))
}

pub fn ind_x(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    match cycle {
        1 => {
            // fetch zero page address
            let jmpaddr = (regs.x + regs.o1) as u16;
            AddrDelegateReturn::Yield(BusMessage::Read{addr: jmpaddr})
        }
        2 => {
            // set new addr LSB
            regs.addr = regs.data as u16;

            // fetch new PC MSB at operand+1
            let jmpaddr = (regs.x + regs.o1 + 1) as u16;
            AddrDelegateReturn::Yield(BusMessage::Read{addr: jmpaddr})
        }
        3 => {
            // set new addr MSB
            regs.addr |= (regs.data as u16) << 8;

            // done
            AddrDelegateReturn::Return(Operand::Address(regs.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn ind_y(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    match cycle {
        1 => {
            // fetch LSB of address from zero page
            AddrDelegateReturn::Yield(BusMessage::Read{addr: regs.o1 as u16})
        }
        2 => {
            // set new PC LSB
            regs.addr = regs.data as u16;

            // fetch MSB of address from zero page
            AddrDelegateReturn::Yield(BusMessage::Read{addr: (regs.o1 + 1) as u16})
        }
        3 => {
            // set new PC MSB
            regs.addr |= (regs.data as u16) << 8;
            
            // add Y
            regs.addr += regs.y as u16;

            // done
            AddrDelegateReturn::Return(Operand::Address(regs.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn ind(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    match cycle {
        1 => {
            // fetch new PC LSB at operand
            let mut jmpaddr = regs.o1 as u16;
            jmpaddr |= (regs.o2 as u16) << 8;
            AddrDelegateReturn::Yield(BusMessage::Read{addr: jmpaddr})
        }
        2 => {
            // set new PC LSB
            regs.addr = regs.data as u16;

            // fetch new PC MSB at operand+1
            let mut jmpaddr = regs.o1 as u16;
            jmpaddr |= (regs.o2 as u16) << 8;
            AddrDelegateReturn::Yield(BusMessage::Read{addr: jmpaddr+1})
        }
        3 => {
            // set new PC MSB
            regs.addr |= (regs.data as u16) << 8;

            // done
            AddrDelegateReturn::Return(Operand::Address(regs.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn abs_x_extra(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    match cycle {
        1 => {
            regs.addr = regs.o1 as u16;
            regs.addr |= (regs.o2 as u16) << 8;
            regs.addr += regs.x as u16;
            
            // this instruction takes one additional cycle when the address crosses a page boundary
            if regs.addr >> 8 != regs.o2 as u16{
                regs.extra_cycle = true;
                AddrDelegateReturn::Yield(BusMessage::Nop)
            } else {
                AddrDelegateReturn::Return(Operand::Address(regs.addr))
            }
        }
        2 => {
            AddrDelegateReturn::Return(Operand::Address(regs.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn abs_y_extra(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    match cycle {
        1 => {
            regs.addr = regs.o1 as u16;
            regs.addr |= (regs.o2 as u16) << 8;
            regs.addr += regs.y as u16;
            
            // this instruction takes one additional cycle when the address crosses a page boundary
            if regs.addr >> 8 != regs.o2 as u16{
                regs.extra_cycle = true;
                AddrDelegateReturn::Yield(BusMessage::Nop)
            } else {
                AddrDelegateReturn::Return(Operand::Address(regs.addr))
            }
        }
        2 => {
            AddrDelegateReturn::Return(Operand::Address(regs.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}

pub fn ind_y_extra(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    match cycle {
        1 => {
            // fetch LSB of address from zero page
            AddrDelegateReturn::Yield(BusMessage::Read{addr: regs.o1 as u16})
        }
        2 => {
            // set new PC LSB
            regs.addr = regs.data as u16;

            // fetch MSB of address from zero page
            AddrDelegateReturn::Yield(BusMessage::Read{addr: (regs.o1 + 1) as u16})
        }
        3 => {
            // instructions needs an extra cycle if page boundary is crossed
            if regs.addr >> 8 != regs.data as u16 {
                regs.extra_cycle = true;
            }

            // set new PC MSB
            regs.addr |= (regs.data as u16) << 8;
            
            // add Y
            regs.addr += regs.y as u16;

            // done
            AddrDelegateReturn::Return(Operand::Address(regs.addr))
        }
        _ => panic!("Addressing cannot continue after Return")
    }
}
