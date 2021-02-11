use crate::{BusMessage, cpu::{AddrDelegateReturn, CPURegisters, Operand}};

pub fn acc(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    AddrDelegateReturn::Return(Operand::Implied)
}

pub fn imm(regs: &mut CPURegisters, _cycle: usize) -> AddrDelegateReturn {
    AddrDelegateReturn::Return(Operand::Immediate(regs.o1))
}

pub fn abs(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    let mut addr = regs.o1 as u16;
    addr |= (regs.o2 as u16) << 8;
    AddrDelegateReturn::Return(Operand::Address(addr))
}

pub fn zp(regs: &mut CPURegisters, _cycle: usize) -> AddrDelegateReturn {
    AddrDelegateReturn::Return(Operand::Address(regs.o1 as u16))
}

pub fn zp_x(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    todo!("functionality for zp_x() addressing");
}

pub fn zp_y(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    todo!("functionality for zp_y() addressing");
}

pub fn abs_x(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    todo!("functionality for abs_x() addressing");
}

pub fn abs_y(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    todo!("functionality for abs_y() addressing");
}

pub fn imp(_regs: &mut CPURegisters, _cycle: usize) -> AddrDelegateReturn {
    AddrDelegateReturn::Return(Operand::Implied)
}

pub fn rel(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    todo!("functionality for rel() addressing");
}

pub fn ind_x(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    todo!("functionality for ind_x() addressing");
}

pub fn ind_y(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    todo!("functionality for ind_y() addressing");
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
    todo!("functionality for abs_x_extra() addressing");
}

pub fn abs_y_extra(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    todo!("functionality for abs_y_extra() addressing");
}

pub fn ind_y_extra(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    todo!("functionality for ind_y_extra() addressing");
}
