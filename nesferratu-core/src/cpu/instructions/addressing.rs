use crate::cpu::{AddrDelegateReturn, CPURegisters, Operand};
pub fn acc(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    todo!("functionality for acc() addressing");
}

pub fn imm(regs: &mut CPURegisters, _cycle: usize) -> AddrDelegateReturn {
    AddrDelegateReturn::Return(Operand::Immediate(regs.o1))
}

pub fn abs(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {
    todo!("functionality for abs() addressing");
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
    todo!("functionality for ind() addressing");
}
