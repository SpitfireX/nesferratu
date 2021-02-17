use crate::cpu::{CPURegisters, CPUFlags};
use crate::BusMessage;
use crate::BusMessage::*;

pub fn txs_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.sp = regs.x;
    Nop
}

pub fn cld_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.set_flag(CPUFlags::D, false);
    Nop
}

pub fn asl_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value
        2 => {
            // carry flag contains old LSB
            regs.set_flag(CPUFlags::C, regs.data & 0x80 == 0x80); // work with fetched value

            regs.data <<= 1;
        
            // zero flag
            regs.set_flag(CPUFlags::Z, regs.data == 0);
        
            // negative flag
            regs.set_flag(CPUFlags::Z, regs.data & 0x80 == 0x80);
        
            Write{addr: address, data: regs.data} // write back changed value
        }
        _ => Nop
    }
}

pub fn tay_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.y = regs.a;
    Nop
}

pub fn sbc_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => BusMessage::Read{addr: address}, // fetch value
        2 => {
            // invert operand
            let immediate = regs.data ^ 0xFF;

            let result: usize = regs.a as usize + immediate as usize + regs.get_flag(CPUFlags::C) as usize;

            // carry flag
            regs.set_flag(CPUFlags::C, result > 255);
        
            // zero flag
            regs.set_flag(CPUFlags::Z, (result & 0xFF) == 0);
        
            // signed overflow flag, V = (A^result) & (M^result) & 0x80
            // see http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
            regs.set_flag(CPUFlags::V, (regs.a as usize ^ result) & (immediate as usize ^ result) & 0x80 > 1);
        
            // load result into accumultoar
            regs.a = result as u8;
        
            Nop
        }
        _ => Nop
    }
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

pub fn cpy_immediate(regs: &mut CPURegisters, immediate: u8, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            let result = regs.y.wrapping_sub(regs.data); // CPY performs Y - M and sets flags

            // zero flag <- A == M
            regs.set_flag(CPUFlags::Z, result == 0);

            // carry flag <- A >= M
            regs.set_flag(CPUFlags::C, regs.y >= regs.data);

            // negative flag
            regs.set_flag(CPUFlags::N, result & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn pha_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    match cycle {
        1 => Write{addr: 0x100 | regs.sp as u16, data: regs.a},
        2 => {
            regs.sp = regs.sp.wrapping_sub(1); // decrement stack pointer
            Nop
        }
        _ => Nop
    }
}

pub fn sei_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.set_flag(CPUFlags::I, true);
    Nop
}

pub fn inc_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address},
        2 => {
            regs.data += 1;
            regs.set_flag(CPUFlags::Z, regs.data == 0);
            regs.set_flag(CPUFlags::N, regs.data & 0x80 == 0x80);
            Nop
        }
        3 => Write{addr: address, data: regs.data},
        _ => Nop,
    }
}

pub fn cpx_immediate(regs: &mut CPURegisters, immediate: u8, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            let result = regs.x.wrapping_sub(immediate); // CPX performs X - M and sets flags

            // zero flag <- A == M
            regs.set_flag(CPUFlags::Z, result == 0);

            // carry flag <- A >= M
            regs.set_flag(CPUFlags::C, regs.x >= immediate);

            // negative flag
            regs.set_flag(CPUFlags::N, result & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn cpy_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value M from memory
        2 => {
            let result = regs.y.wrapping_sub(regs.data); // CPY performs Y - M and sets flags

            // zero flag <- A == M
            regs.set_flag(CPUFlags::Z, result == 0);

            // carry flag <- A >= M
            regs.set_flag(CPUFlags::C, regs.y >= regs.data);

            // negative flag
            regs.set_flag(CPUFlags::N, result & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn rol_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            // save status register
            let temp = regs.status;

            // carry flag contains old MSB
            regs.set_flag(CPUFlags::C, regs.a & 0x80 == 0x80);

            regs.a <<= 1;

            // new LSB is the old carry flag
            regs.a |= temp & 1;

            // zero flag
            regs.set_flag(CPUFlags::Z, regs.a == 0);

            // negative flag
            regs.set_flag(CPUFlags::Z, regs.a & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn dec_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address},
        2 => {
            regs.data -= 1;
            regs.set_flag(CPUFlags::Z, regs.data == 0);
            regs.set_flag(CPUFlags::N, regs.data & 0x80 == 0x80);
            Nop
        }
        3 => Write{addr: address, data: regs.data},
        _ => Nop,
    }
}

pub fn ldx_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    regs.x = immediate;
    regs.set_flag(CPUFlags::Z, regs.x == 0);
    regs.set_flag(CPUFlags::N, regs.x & 0x80 == 0x80);
    Nop
}

pub fn tsx_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.x = regs.sp;
    Nop
}

pub fn inx_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.x += 1;
    regs.set_flag(CPUFlags::Z, regs.x == 0);
    regs.set_flag(CPUFlags::N, regs.x & 0x80 == 0x80);
    Nop
}

pub fn brk_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for brk_implied()");
}

pub fn iny_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.y += 1;
    regs.set_flag(CPUFlags::Z, regs.y == 0);
    regs.set_flag(CPUFlags::N, regs.y & 0x80 == 0x80);
    Nop
}

pub fn sed_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.set_flag(CPUFlags::D, true);
    Nop
}

pub fn sbc_immediate(regs: &mut CPURegisters, mut immediate: u8, _cycle: usize) -> BusMessage {
    // invert operand
    immediate ^= 0xFF;
    
    let result: usize = regs.a as usize + immediate as usize + regs.get_flag(CPUFlags::C) as usize;

    // carry flag
    regs.set_flag(CPUFlags::C, result > 255);

    // zero flag
    regs.set_flag(CPUFlags::Z, (result & 0xFF) == 0);

    // signed overflow flag, V = (A^result) & (M^result) & 0x80
    // see http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    regs.set_flag(CPUFlags::V, (regs.a as usize ^ result) & (immediate as usize ^ result) & 0x80 > 1);

    // load result into accumultoar
    regs.a = result as u8;

    Nop
}

pub fn ora_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value from address
        2 => {
            regs.a |= regs.data; // fetched value

            // zero flag
            regs.set_flag(CPUFlags::Z, regs.a == 0);
        
            // negative flag
            regs.set_flag(CPUFlags::Z, regs.a & 0x80 == 0x80);
        
            Nop
        }
        _ => Nop
    }
}

pub fn sec_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.set_flag(CPUFlags::C, true);
    Nop
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
    match cycle {
        1 => Read{addr: address},
        2 => {
            regs.y = regs.data;
            regs.set_flag(CPUFlags::Z, regs.y == 0);
            regs.set_flag(CPUFlags::N, regs.y & 0x80 == 0x80);
            Nop
        }
        _ => Nop
    }
}

pub fn rol_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value
        2 => {
            // save status register
            let temp = regs.status;
            
            // carry flag contains old LSB
            regs.set_flag(CPUFlags::C, regs.data & 0x80 == 0x80); // work with fetched value

            regs.data <<= 1;

            // new LSB is the old carry flag
            regs.data |= temp & 1;

            // zero flag
            regs.set_flag(CPUFlags::Z, regs.data == 0);
        
            // negative flag
            regs.set_flag(CPUFlags::N, regs.data & 0x80 == 0x80);
        
            Write{addr: address, data: regs.data} // write back changed value
        }
        _ => Nop
    }
}

pub fn dex_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.x -= 1;
    regs.set_flag(CPUFlags::Z, regs.x == 0);
    regs.set_flag(CPUFlags::N, regs.x & 0x80 == 0x80);
    Nop
}

pub fn php_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    match cycle {
        1 => Write{addr: 0x100 | regs.sp as u16, data: regs.status},
        2 => {
            regs.sp = regs.sp.wrapping_sub(1); // decrement stack pointer
            Nop
        }
        _ => Nop
    }
}

pub fn rti_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for rti_implied()");
}

pub fn asl_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            // carry flag contains old MSB
            regs.set_flag(CPUFlags::C, regs.a & 0x80 == 0x80);

            regs.a <<= 1;

            // zero flag
            regs.set_flag(CPUFlags::Z, regs.a == 0);

            // negative flag
            regs.set_flag(CPUFlags::Z, regs.a & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn ldx_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address},
        2 => {
            regs.x = regs.data;
            regs.set_flag(CPUFlags::Z, regs.x == 0);
            regs.set_flag(CPUFlags::N, regs.x & 0x80 == 0x80);
            Nop
        }
        _ => Nop
    }
}

pub fn clv_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.set_flag(CPUFlags::V, false);
    Nop
}

pub fn nop_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    Nop
}

pub fn adc_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    let result: usize = regs.a as usize + immediate as usize + regs.get_flag(CPUFlags::C) as usize;

    // carry flag
    regs.set_flag(CPUFlags::C, result > 255);

    // zero flag
    regs.set_flag(CPUFlags::Z, (result & 0xFF) == 0);

    // signed overflow flag, V = (A^result) & (M^result) & 0x80
    // see http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    regs.set_flag(CPUFlags::V, (regs.a as usize ^ result) & (immediate as usize ^ result) & 0x80 > 1);

    // load result into accumultoar
    regs.a = result as u8;

    Nop
}

pub fn cli_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.set_flag(CPUFlags::I, false);
    Nop
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
    regs.x = regs.a;
    Nop
}

pub fn dey_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.y -= 1;
    regs.set_flag(CPUFlags::Z, regs.y == 0);
    regs.set_flag(CPUFlags::N, regs.y & 0x80 == 0x80);
    Nop
}

pub fn eor_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for eor_address()");
}

pub fn lsr_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            // carry flag contains old LSB
            regs.set_flag(CPUFlags::C, regs.a & 1 == 1);

            regs.a >>= 1;

            // zero flag
            regs.set_flag(CPUFlags::Z, regs.a == 0);

            // negative flag
            regs.set_flag(CPUFlags::Z, regs.a & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn bvs_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for bvs_address()");
}

pub fn rts_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    todo!("functionality for rts_implied()");
}

pub fn tya_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.a = regs.y;
    Nop
}

pub fn plp_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            regs.sp = regs.sp.wrapping_add(1); // increment stack pointer
            Read{addr: 0x100 | regs.sp as u16}
        }
        2 => {
            regs.status = regs.data;
            Nop
        }
        _ => Nop
    }
}

pub fn and_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    regs.a &= immediate;

    // zero flag
    regs.set_flag(CPUFlags::Z, regs.a == 0);

    // negative flag
    regs.set_flag(CPUFlags::Z, regs.a & 0x80 == 0x80);

    Nop
}

pub fn jmp_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    regs.pc = address;
    Nop
}

pub fn ror_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value
        2 => {
            // save status register
            let temp = regs.status;
            
            // carry flag contains old LSB
            regs.set_flag(CPUFlags::C, regs.data & 1 == 1); // work with fetched value

            regs.data >>= 1;

            // new MSB is the old carry flag
            regs.data |= (temp & 1) << 7;
        
            // zero flag
            regs.set_flag(CPUFlags::Z, regs.data == 0);
        
            // negative flag
            regs.set_flag(CPUFlags::Z, regs.data & 0x80 == 0x80);
        
            Write{addr: address, data: regs.data} // write back changed value
        }
        _ => Nop
    }
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
    match cycle {
        1 => Read{addr: address}, // fetch value M from memory
        2 => {
            let result = regs.a.wrapping_sub(regs.data); // CMP performs A - M and sets flags

            // zero flag <- A == M
            regs.set_flag(CPUFlags::Z, result == 0);

            // carry flag <- A >= M
            regs.set_flag(CPUFlags::C, regs.a >= regs.data);

            // negative flag
            regs.set_flag(CPUFlags::N, result & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn beq_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for beq_address()");
}

pub fn lda_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address},
        2 => {
            regs.a = regs.data;
            regs.set_flag(CPUFlags::Z, regs.a == 0);
            regs.set_flag(CPUFlags::N, regs.a & 0x80 == 0x80);
            Nop
        }
        _ => Nop
    }
}

pub fn lsr_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value
        2 => {
            // carry flag contains old LSB
            regs.set_flag(CPUFlags::C, regs.data & 1 == 1); // work with fetched value

            regs.data >>= 1;
        
            // zero flag
            regs.set_flag(CPUFlags::Z, regs.data == 0);
        
            // negative flag
            regs.set_flag(CPUFlags::Z, regs.data & 0x80 == 0x80);
        
            Write{addr: address, data: regs.data} // write back changed value
        }
        _ => Nop
    }
}

pub fn adc_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => BusMessage::Read{addr: address}, // fetch value
        2 => {
            let result: usize = regs.a as usize + regs.data as usize + regs.get_flag(CPUFlags::C) as usize;

            // carry flag
            regs.set_flag(CPUFlags::C, result > 255);
        
            // zero flag
            regs.set_flag(CPUFlags::Z, (result & 0xFF) == 0);
        
            // signed overflow flag, V = (A^result) & (M^result) & 0x80
            // see http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
            regs.set_flag(CPUFlags::V, (regs.a as usize ^ result) & (regs.data as usize ^ result) & 0x80 > 1);
        
            // load result into accumultoar
            regs.a = result as u8;
        
            Nop
        }
        _ => Nop
    }
}

pub fn bcs_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for bcs_address()");
}

pub fn ror_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            // save status register
            let temp = regs.status;

            // carry flag contains old LSB
            regs.set_flag(CPUFlags::C, regs.a & 1 == 1);

            regs.a >>= 1;

            // new MSB is the old carry flag
            regs.a |= (temp & 1) << 7;

            // zero flag
            regs.set_flag(CPUFlags::Z, regs.a == 0);

            // negative flag
            regs.set_flag(CPUFlags::Z, regs.a & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn cpx_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value M from memory
        2 => {
            let result = regs.x.wrapping_sub(regs.data); // CPX performs A - M and sets flags

            // zero flag <- A == M
            regs.set_flag(CPUFlags::Z, result == 0);

            // carry flag <- A >= M
            regs.set_flag(CPUFlags::C, regs.x >= regs.data);

            // negative flag
            regs.set_flag(CPUFlags::N, result & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn and_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value from address
        2 => {
            regs.a &= regs.data; // fetched value

            // zero flag
            regs.set_flag(CPUFlags::Z, regs.a == 0);
        
            // negative flag
            regs.set_flag(CPUFlags::Z, regs.a & 0x80 == 0x80);
        
            Nop
        }
        _ => Nop
    }
}

pub fn eor_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    todo!("functionality for eor_immediate()");
}

pub fn pla_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            regs.sp = regs.sp.wrapping_add(1); // increment stack pointer
            Read{addr: 0x100 | regs.sp as u16}
        }
        2 => {
            regs.a = regs.data;
            Nop
        }
        _ => Nop
    }
}

pub fn ora_immediate(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {
    regs.a |= immediate;

    // zero flag
    regs.set_flag(CPUFlags::Z, regs.a == 0);

    // negative flag
    regs.set_flag(CPUFlags::Z, regs.a & 0x80 == 0x80);

    Nop
}

pub fn cmp_immediate(regs: &mut CPURegisters, immediate: u8, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            let result = regs.a.wrapping_sub(immediate); // CMP performs A - M and sets flags

            // zero flag <- A == M
            regs.set_flag(CPUFlags::Z, result == 0);

            // carry flag <- A >= M
            regs.set_flag(CPUFlags::C, regs.a >= immediate);

            // negative flag
            regs.set_flag(CPUFlags::N, result & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn clc_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.set_flag(CPUFlags::C, false);
    Nop
}

pub fn bpl_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    todo!("functionality for bpl_address()");
}

pub fn bit_address(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch operand M
        2 => {
            let result = regs.a & regs.data; // BIT performs A & M but doesn't store the result

            // zero flag
            regs.set_flag(CPUFlags::Z, result == 0);

            // overflow flag <- M6
            regs.set_flag(CPUFlags::V, regs.data & 0x70 == 0x70);

            // negative flag <- M7
            regs.set_flag(CPUFlags::N, regs.data & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn txa_implied(regs: &mut CPURegisters, cycle: usize) -> BusMessage {
    regs.a = regs.x;
    Nop
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
