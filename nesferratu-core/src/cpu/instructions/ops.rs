use crate::cpu::{CPURegisters, CPUFlags};
use crate::BusMessage;
use crate::BusMessage::*;

pub fn txs_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for txs_implied()");
}

pub fn cld_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for cld_implied()");
}

pub fn asl_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for asl_address()");
}

pub fn tay_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for tay_implied()");
}

pub fn sbc_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for sbc_address()");
}

pub fn jsr_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for jsr_address()");
}

pub fn lda_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    regs.a = immediate;
    regs.set_flag(CPUFlags::Z, regs.a == 0);
    regs.set_flag(CPUFlags::N, regs.a & 0x80 == 0x80);
    Nop
}

pub fn cpy_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    todo!("functionality for cpy_immediate()");
}

pub fn pha_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for pha_implied()");
}

pub fn sei_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for sei_implied()");
}

pub fn inc_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for inc_address()");
}

pub fn cpx_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    todo!("functionality for cpx_immediate()");
}

pub fn cpy_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for cpy_address()");
}

pub fn rol_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for rol_implied()");
}

pub fn dec_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for dec_address()");
}

pub fn ldx_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    regs.x = immediate;
    regs.set_flag(CPUFlags::Z, regs.x == 0);
    regs.set_flag(CPUFlags::N, regs.x & 0x80 == 0x80);
    Nop
}

pub fn tsx_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for tsx_implied()");
}

pub fn inx_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for inx_implied()");
}

pub fn brk_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for brk_implied()");
}

pub fn iny_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for iny_implied()");
}

pub fn sed_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for sed_implied()");
}

pub fn sbc_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    todo!("functionality for sbc_immediate()");
}

pub fn ora_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for ora_address()");
}

pub fn sec_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for sec_implied()");
}

pub fn bne_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for bne_address()");
}

pub fn sty_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            Write{addr: address, data: regs.y}
        },
        _ => Nop,
    }
}

pub fn ldy_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for ldy_address()");
}

pub fn rol_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for rol_address()");
}

pub fn dex_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for dex_implied()");
}

pub fn php_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for php_implied()");
}

pub fn rti_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for rti_implied()");
}

pub fn asl_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for asl_implied()");
}

pub fn ldx_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for ldx_address()");
}

pub fn clv_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for clv_implied()");
}

pub fn nop_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for nop_implied()");
}

pub fn adc_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    todo!("functionality for adc_immediate()");
}

pub fn cli_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for cli_implied()");
}

pub fn stx_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            Write{addr: address, data: regs.x}
        },
        _ => Nop,
    }
}

pub fn bmi_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for bmi_address()");
}

pub fn ldy_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    regs.y = immediate;
    regs.set_flag(CPUFlags::Z, regs.y == 0);
    regs.set_flag(CPUFlags::N, regs.y & 0x80 == 0x80);
    Nop
}

pub fn tax_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for tax_implied()");
}

pub fn dey_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for dey_implied()");
}

pub fn eor_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for eor_address()");
}

pub fn lsr_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for lsr_implied()");
}

pub fn bvs_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for bvs_address()");
}

pub fn rts_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for rts_implied()");
}

pub fn tya_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for tya_implied()");
}

pub fn plp_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for plp_implied()");
}

pub fn and_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    todo!("functionality for and_immediate()");
}

pub fn jmp_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    regs.pc = address;
    Nop
}

pub fn ror_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for ror_address()");
}

pub fn sta_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            Write{addr: address, data: regs.a}
        },
        _ => Nop,
    }
}

pub fn cmp_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for cmp_address()");
}

pub fn beq_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for beq_address()");
}

pub fn lda_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for lda_address()");
}

pub fn lsr_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for lsr_address()");
}

pub fn adc_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for adc_address()");
}

pub fn bcs_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for bcs_address()");
}

pub fn ror_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for ror_implied()");
}

pub fn cpx_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for cpx_address()");
}

pub fn and_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for and_address()");
}

pub fn eor_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    todo!("functionality for eor_immediate()");
}

pub fn pla_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for pla_implied()");
}

pub fn ora_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    todo!("functionality for ora_immediate()");
}

pub fn cmp_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    todo!("functionality for cmp_immediate()");
}

pub fn clc_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for clc_implied()");
}

pub fn bpl_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for bpl_address()");
}

pub fn bit_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for bit_address()");
}

pub fn txa_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for txa_implied()");
}

pub fn bvc_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for bvc_address()");
}

pub fn bcc_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for bcc_address()");
}

pub fn reset(regs: &mut CPURegisters, reset_vector: u16, cycle: usize) -> BusMessage {
    match cycle {
        x if x < 6 => Nop,
        6 => {
            regs.set_flag(CPUFlags::I, true);
            Read{addr: reset_vector}
        },
        7 => {
            regs.pc |= regs.data as u16; // set low byte of new PC address
            Read{addr: reset_vector+1}
        },
        8 => {
            regs.pc |= (regs.data as u16) << 8; // set high byte of ne PC address

            // reset rest of the registers
            regs.a = 0x00;
            regs.x = 0x00;
            regs.y = 0x00;
            regs.sp = 0xFD; // default address for stack pointer
            regs.status = 0x24; // 3rd bit unused and always high, I flag still set

            // also the relevant helpers
            regs.op = 0x00;
            regs.o1 = 0x00;
            regs.o2 = 0x00;
            
            Read{addr: regs.pc}
        },
        _ => panic!("Impossible cycle count in match, reset takes 8 cycles"),
    }
}
