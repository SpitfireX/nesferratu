use crate::cpu::{CpuState, CpuFlags};
use crate::BusMessage;
use crate::BusMessage::*;

pub fn txs_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.sp = s.regs.x;
    Nop
}

pub fn cld_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.set_flag(CpuFlags::D, false);
    Nop
}

pub fn asl_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value
        2 => {
            // carry flag contains old LSB
            s.regs.set_flag(CpuFlags::C, s.data & 0x80 == 0x80); // work with fetched value

            s.data <<= 1;
        
            // zero flag
            s.regs.set_flag(CpuFlags::Z, s.data == 0);
        
            // negative flag
            s.regs.set_flag(CpuFlags::Z, s.data & 0x80 == 0x80);
        
            Write{addr: address, data: s.data} // write back changed value
        }
        _ => Nop
    }
}

pub fn tay_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.y = s.regs.a;
    Nop
}

pub fn sbc_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => BusMessage::Read{addr: address}, // fetch value
        2 => {
            // invert operand
            let immediate = s.data ^ 0xFF;

            let result: usize = (s.regs.a as usize)
                                    .wrapping_add(immediate as usize)
                                    .wrapping_add(s.regs.get_flag(CpuFlags::C) as usize);

            // carry flag
            s.regs.set_flag(CpuFlags::C, result > 255);
        
            // zero flag
            s.regs.set_flag(CpuFlags::Z, (result & 0xFF) == 0);
        
            // signed overflow flag, V = (A^result) & (M^result) & 0x80
            // see http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
            s.regs.set_flag(CpuFlags::V, (s.regs.a as usize ^ result) & (immediate as usize ^ result) & 0x80 > 1);
        
            // load result into accumultoar
            s.regs.a = result as u8;
        
            Nop
        }
        _ => Nop
    }
}

pub fn jsr_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Write{addr: 0x100 | s.regs.sp as u16, data: ((s.regs.pc-1) >> 8) as u8}, // push PC high byte to stack
        2 => {
            s.regs.sp = s.regs.sp.wrapping_sub(1);
            Write{addr: 0x100 | s.regs.sp as u16, data: (s.regs.pc-1) as u8} // push PC low byte to stack
        }
        3 => {
            s.regs.sp = s.regs.sp.wrapping_sub(1);
            s.regs.pc = address; // jump to new PC
            Nop
        }
        _ => Nop,
    }
}

pub fn lda_immediate(s: &mut CpuState, immediate: u8, _cycle: usize) -> BusMessage {
    s.regs.a = immediate;
    s.regs.set_flag(CpuFlags::Z, s.regs.a == 0);
    s.regs.set_flag(CpuFlags::N, s.regs.a & 0x80 == 0x80);
    Nop
}

pub fn cpy_immediate(s: &mut CpuState, immediate: u8, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            let result = s.regs.y.wrapping_sub(s.data); // CPY performs Y - M and sets flags

            // zero flag <- A == M
            s.regs.set_flag(CpuFlags::Z, result == 0);

            // carry flag <- A >= M
            s.regs.set_flag(CpuFlags::C, s.regs.y >= s.data);

            // negative flag
            s.regs.set_flag(CpuFlags::N, result & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn pha_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    match cycle {
        1 => Write{addr: 0x100 | s.regs.sp as u16, data: s.regs.a},
        2 => {
            s.regs.sp = s.regs.sp.wrapping_sub(1); // decrement stack pointer
            Nop
        }
        _ => Nop
    }
}

pub fn sei_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.set_flag(CpuFlags::I, true);
    Nop
}

pub fn inc_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address},
        2 => {
            s.data = s.data.wrapping_add(1);
            s.regs.set_flag(CpuFlags::Z, s.data == 0);
            s.regs.set_flag(CpuFlags::N, s.data & 0x80 == 0x80);
            Nop
        }
        3 => Write{addr: address, data: s.data},
        _ => Nop,
    }
}

pub fn cpx_immediate(s: &mut CpuState, immediate: u8, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            let result = s.regs.x.wrapping_sub(immediate); // CPX performs X - M and sets flags

            // zero flag <- A == M
            s.regs.set_flag(CpuFlags::Z, result == 0);

            // carry flag <- A >= M
            s.regs.set_flag(CpuFlags::C, s.regs.x >= immediate);

            // negative flag
            s.regs.set_flag(CpuFlags::N, result & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn cpy_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value M from memory
        2 => {
            let result = s.regs.y.wrapping_sub(s.data); // CPY performs Y - M and sets flags

            // zero flag <- A == M
            s.regs.set_flag(CpuFlags::Z, result == 0);

            // carry flag <- A >= M
            s.regs.set_flag(CpuFlags::C, s.regs.y >= s.data);

            // negative flag
            s.regs.set_flag(CpuFlags::N, result & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn rol_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            // save status register
            let temp = s.regs.status;

            // carry flag contains old MSB
            s.regs.set_flag(CpuFlags::C, s.regs.a & 0x80 == 0x80);

            s.regs.a <<= 1;

            // new LSB is the old carry flag
            s.regs.a |= temp & 1;

            // zero flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a == 0);

            // negative flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn dec_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address},
        2 => {
            s.data = s.data.wrapping_sub(1);
            s.regs.set_flag(CpuFlags::Z, s.data == 0);
            s.regs.set_flag(CpuFlags::N, s.data & 0x80 == 0x80);
            Nop
        }
        3 => Write{addr: address, data: s.data},
        _ => Nop,
    }
}

pub fn ldx_immediate(s: &mut CpuState, immediate: u8, _cycle: usize) -> BusMessage {
    s.regs.x = immediate;
    s.regs.set_flag(CpuFlags::Z, s.regs.x == 0);
    s.regs.set_flag(CpuFlags::N, s.regs.x & 0x80 == 0x80);
    Nop
}

pub fn tsx_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.x = s.regs.sp;
    Nop
}

pub fn inx_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.x = s.regs.x.wrapping_add(1);
    s.regs.set_flag(CpuFlags::Z, s.regs.x == 0);
    s.regs.set_flag(CpuFlags::N, s.regs.x & 0x80 == 0x80);
    Nop
}

pub fn brk_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    match cycle {
        1 => Write{addr: 0x100 | s.regs.sp as u16, data: ((s.regs.pc+1) >> 8) as u8}, // push BRK PC+2 high byte to stack
        2 => {
            s.regs.sp = s.regs.sp.wrapping_sub(1);
            Write{addr: 0x100 | s.regs.sp as u16, data: (s.regs.pc+1) as u8} // push BRK PC+2 low byte to stack
        }
        3 => {
            s.regs.sp = s.regs.sp.wrapping_sub(1);
            Write{addr: 0x100 | s.regs.sp as u16, data: s.regs.status | 0x10} // push status register with B set to stack
        }
        4 => {
            s.regs.sp = s.regs.sp.wrapping_sub(1);
            Read{addr: 0xFFFE} // fetch PC low byte from vector
        }
        5 => {
            s.regs.pc = s.data as u16; // set PC low byte from stack
            Read{addr: 0xFFFF} // fetch PC high byte from vector
        }
        6 => {
            s.regs.pc |= (s.data as u16) << 8; // set PC high byte from stack
            Nop
        }
        _ => Nop,
    }
}

pub fn iny_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.y = s.regs.y.wrapping_add(1);
    s.regs.set_flag(CpuFlags::Z, s.regs.y == 0);
    s.regs.set_flag(CpuFlags::N, s.regs.y & 0x80 == 0x80);
    Nop
}

pub fn sed_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.set_flag(CpuFlags::D, true);
    Nop
}

pub fn sbc_immediate(s: &mut CpuState, mut immediate: u8, _cycle: usize) -> BusMessage {
    // invert operand
    immediate ^= 0xFF;
    
    let result: usize = (s.regs.a as usize)
                            .wrapping_add(immediate as usize)
                            .wrapping_add(s.regs.get_flag(CpuFlags::C) as usize);

    // carry flag
    s.regs.set_flag(CpuFlags::C, result > 255);

    // zero flag
    s.regs.set_flag(CpuFlags::Z, (result & 0xFF) == 0);

    // signed overflow flag, V = (A^result) & (M^result) & 0x80
    // see http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    s.regs.set_flag(CpuFlags::V, (s.regs.a as usize ^ result) & (immediate as usize ^ result) & 0x80 > 1);

    // load result into accumultoar
    s.regs.a = result as u8;

    Nop
}

pub fn ora_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value from address
        2 => {
            s.regs.a |= s.data; // fetched value

            // zero flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a == 0);
        
            // negative flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a & 0x80 == 0x80);
        
            Nop
        }
        _ => Nop
    }
}

pub fn sec_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.set_flag(CpuFlags::C, true);
    Nop
}

pub fn bne_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            if !s.regs.get_flag(CpuFlags::Z) { // zero flag not set
                s.extra_cycle = true;
            }
            Nop
        }
        2 => { // can only be reached if branch condition met
            // branch ops need one extra cycle if the branch jumps across a page boundary
            if s.regs.pc >> 8 != address >> 8 {
                s.extra_cycle = true;
            }

            println!("new address after branch: {:04X}", address);
            s.regs.pc = address;
            
            Nop
        }
        _ => Nop,
    }
}

pub fn sty_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            Write{addr: address, data: s.regs.y}
        },
        _ => Nop,
    }
}

pub fn ldy_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address},
        2 => {
            s.regs.y = s.data;
            s.regs.set_flag(CpuFlags::Z, s.regs.y == 0);
            s.regs.set_flag(CpuFlags::N, s.regs.y & 0x80 == 0x80);
            Nop
        }
        _ => Nop
    }
}

pub fn rol_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value
        2 => {
            // save status register
            let temp = s.regs.status;
            
            // carry flag contains old LSB
            s.regs.set_flag(CpuFlags::C, s.data & 0x80 == 0x80); // work with fetched value

            s.data <<= 1;

            // new LSB is the old carry flag
            s.data |= temp & 1;

            // zero flag
            s.regs.set_flag(CpuFlags::Z, s.data == 0);
        
            // negative flag
            s.regs.set_flag(CpuFlags::N, s.data & 0x80 == 0x80);
        
            Write{addr: address, data: s.data} // write back changed value
        }
        _ => Nop
    }
}

pub fn dex_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.x = s.regs.x.wrapping_sub(1);
    s.regs.set_flag(CpuFlags::Z, s.regs.x == 0);
    s.regs.set_flag(CpuFlags::N, s.regs.x & 0x80 == 0x80);
    Nop
}

pub fn php_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    match cycle {
        1 => Write{addr: 0x100 | s.regs.sp as u16, data: s.regs.status},
        2 => {
            s.regs.sp = s.regs.sp.wrapping_sub(1); // decrement stack pointer
            Nop
        }
        _ => Nop
    }
}

pub fn rti_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            s.regs.sp = s.regs.sp.wrapping_add(1); // increment stack pointer
            Read{addr: 0x100 | s.regs.sp as u16} // pull status register from stack
        }
        2 => {
            s.regs.status = s.data; // set status register from stack

            s.regs.sp = s.regs.sp.wrapping_add(1); // increment stack pointer
            Read{addr: 0x100 | s.regs.sp as u16} // pull PC low byte from stack
        }
        3 => {
            s.regs.pc = s.data as u16; // set PC low byte from stack

            s.regs.sp = s.regs.sp.wrapping_add(1); // increment stack pointer
            Read{addr: 0x100 | s.regs.sp as u16} // pull PC high byte from stack
        }
        4 => {
            s.regs.pc |= (s.data as u16) << 8; // set PC high byte from stack
            Nop
        }
        _ => Nop,
    }
}

pub fn asl_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            // carry flag contains old MSB
            s.regs.set_flag(CpuFlags::C, s.regs.a & 0x80 == 0x80);

            s.regs.a <<= 1;

            // zero flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a == 0);

            // negative flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn ldx_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address},
        2 => {
            s.regs.x = s.data;
            s.regs.set_flag(CpuFlags::Z, s.regs.x == 0);
            s.regs.set_flag(CpuFlags::N, s.regs.x & 0x80 == 0x80);
            Nop
        }
        _ => Nop
    }
}

pub fn clv_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.set_flag(CpuFlags::V, false);
    Nop
}

pub fn nop_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    Nop
}

pub fn adc_immediate(s: &mut CpuState, immediate: u8, _cycle: usize) -> BusMessage {
    let result: usize = (s.regs.a as usize)
                            .wrapping_add(immediate as usize)
                            .wrapping_add(s.regs.get_flag(CpuFlags::C) as usize);

    // carry flag
    s.regs.set_flag(CpuFlags::C, result > 255);

    // zero flag
    s.regs.set_flag(CpuFlags::Z, (result & 0xFF) == 0);

    // signed overflow flag, V = (A^result) & (M^result) & 0x80
    // see http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    s.regs.set_flag(CpuFlags::V, (s.regs.a as usize ^ result) & (immediate as usize ^ result) & 0x80 > 1);

    // load result into accumultoar
    s.regs.a = result as u8;

    Nop
}

pub fn cli_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.set_flag(CpuFlags::I, false);
    Nop
}

pub fn stx_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            Write{addr: address, data: s.regs.x}
        },
        _ => Nop,
    }
}

pub fn bmi_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            if s.regs.get_flag(CpuFlags::N) { // negative flag set
                s.extra_cycle = true;
            }
            Nop
        }
        2 => { // can only be reached if branch condition met
            // branch ops need one extra cycle if the branch jumps across a page boundary
            if s.regs.pc >> 8 != address >> 8 {
                s.extra_cycle = true;
            }

            println!("new address after branch: {:04X}", address);
            s.regs.pc = address;
            
            Nop
        }
        _ => Nop,
    }
}

pub fn ldy_immediate(s: &mut CpuState, immediate: u8, _cycle: usize) -> BusMessage {
    s.regs.y = immediate;
    s.regs.set_flag(CpuFlags::Z, s.regs.y == 0);
    s.regs.set_flag(CpuFlags::N, s.regs.y & 0x80 == 0x80);
    Nop
}

pub fn tax_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.x = s.regs.a;
    Nop
}

pub fn dey_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.y = s.regs.y.wrapping_sub(1);
    s.regs.set_flag(CpuFlags::Z, s.regs.y == 0);
    s.regs.set_flag(CpuFlags::N, s.regs.y & 0x80 == 0x80);
    Nop
}

pub fn eor_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value from address
        2 => {
            s.regs.a ^= s.data; // fetched value

            // zero flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a == 0);
        
            // negative flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a & 0x80 == 0x80);
        
            Nop
        }
        _ => Nop
    }
}

pub fn lsr_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            // carry flag contains old LSB
            s.regs.set_flag(CpuFlags::C, s.regs.a & 1 == 1);

            s.regs.a >>= 1;

            // zero flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a == 0);

            // negative flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn bvs_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            if s.regs.get_flag(CpuFlags::V) { // overflow flag set
                s.extra_cycle = true;
            }
            Nop
        }
        2 => { // can only be reached if branch condition met
            // branch ops need one extra cycle if the branch jumps across a page boundary
            if s.regs.pc >> 8 != address >> 8 {
                s.extra_cycle = true;
            }

            println!("new address after branch: {:04X}", address);
            s.regs.pc = address;
            
            Nop
        }
        _ => Nop,
    }
}

pub fn rts_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            s.regs.sp = s.regs.sp.wrapping_add(1); // increment stack pointer
            Read{addr: 0x100 | s.regs.sp as u16} // load new PC low
        }
        2 => {
            s.regs.pc = s.data as u16;
            s.regs.sp = s.regs.sp.wrapping_add(1); // increment stack pointer
            Read{addr: 0x100 | s.regs.sp as u16} // load new PC high
        }
        3 => {
            s.regs.pc |= (s.data as u16) << 8;
            Nop
        }
        4 => {
            s.regs.pc += 1; // increment PC by one to point to next opcode
            Nop
        }
        _ => Nop,
    }
}

pub fn tya_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.a = s.regs.y;
    Nop
}

pub fn plp_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            s.regs.sp = s.regs.sp.wrapping_add(1); // increment stack pointer
            Read{addr: 0x100 | s.regs.sp as u16}
        }
        2 => {
            s.regs.status = s.data;
            Nop
        }
        _ => Nop
    }
}

pub fn and_immediate(s: &mut CpuState, immediate: u8, _cycle: usize) -> BusMessage {
    s.regs.a &= immediate;

    // zero flag
    s.regs.set_flag(CpuFlags::Z, s.regs.a == 0);

    // negative flag
    s.regs.set_flag(CpuFlags::Z, s.regs.a & 0x80 == 0x80);

    Nop
}

pub fn jmp_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    s.regs.pc = address;
    Nop
}

pub fn ror_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value
        2 => {
            // save status register
            let temp = s.regs.status;
            
            // carry flag contains old LSB
            s.regs.set_flag(CpuFlags::C, s.data & 1 == 1); // work with fetched value

            s.data >>= 1;

            // new MSB is the old carry flag
            s.data |= (temp & 1) << 7;
        
            // zero flag
            s.regs.set_flag(CpuFlags::Z, s.data == 0);
        
            // negative flag
            s.regs.set_flag(CpuFlags::Z, s.data & 0x80 == 0x80);
        
            Write{addr: address, data: s.data} // write back changed value
        }
        _ => Nop
    }
}

pub fn sta_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            Write{addr: address, data: s.regs.a}
        },
        _ => Nop,
    }
}

pub fn cmp_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value M from memory
        2 => {
            let result = s.regs.a.wrapping_sub(s.data); // CMP performs A - M and sets flags

            // zero flag <- A == M
            s.regs.set_flag(CpuFlags::Z, result == 0);

            // carry flag <- A >= M
            s.regs.set_flag(CpuFlags::C, s.regs.a >= s.data);

            // negative flag
            s.regs.set_flag(CpuFlags::N, result & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn beq_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            if s.regs.get_flag(CpuFlags::Z) { // zero flag set
                s.extra_cycle = true;
            }
            Nop
        }
        2 => { // can only be reached if branch condition met
            // branch ops need one extra cycle if the branch jumps across a page boundary
            if s.regs.pc >> 8 != address >> 8 {
                s.extra_cycle = true;
            }

            println!("new address after branch: {:04X}", address);
            s.regs.pc = address;
            
            Nop
        }
        _ => Nop,
    }
}

pub fn lda_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address},
        2 => {
            s.regs.a = s.data;
            s.regs.set_flag(CpuFlags::Z, s.regs.a == 0);
            s.regs.set_flag(CpuFlags::N, s.regs.a & 0x80 == 0x80);
            Nop
        }
        _ => Nop
    }
}

pub fn lsr_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value
        2 => {
            // carry flag contains old LSB
            s.regs.set_flag(CpuFlags::C, s.data & 1 == 1); // work with fetched value

            s.data >>= 1;
        
            // zero flag
            s.regs.set_flag(CpuFlags::Z, s.data == 0);
        
            // negative flag
            s.regs.set_flag(CpuFlags::Z, s.data & 0x80 == 0x80);
        
            Write{addr: address, data: s.data} // write back changed value
        }
        _ => Nop
    }
}

pub fn adc_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => BusMessage::Read{addr: address}, // fetch value
        2 => {
            let result: usize = (s.regs.a as usize)
                                    .wrapping_add(s.data as usize)
                                    .wrapping_add(s.regs.get_flag(CpuFlags::C) as usize);

            // carry flag
            s.regs.set_flag(CpuFlags::C, result > 255);
        
            // zero flag
            s.regs.set_flag(CpuFlags::Z, (result & 0xFF) == 0);
        
            // signed overflow flag, V = (A^result) & (M^result) & 0x80
            // see http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
            s.regs.set_flag(CpuFlags::V, (s.regs.a as usize ^ result) & (s.data as usize ^ result) & 0x80 > 1);
        
            // load result into accumultoar
            s.regs.a = result as u8;
        
            Nop
        }
        _ => Nop
    }
}

pub fn bcs_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            if s.regs.get_flag(CpuFlags::C) { // carry flag set
                s.extra_cycle = true;
            }
            Nop
        }
        2 => { // can only be reached if branch condition met
            // branch ops need one extra cycle if the branch jumps across a page boundary
            if s.regs.pc >> 8 != address >> 8 {
                s.extra_cycle = true;
            }

            println!("new address after branch: {:04X}", address);
            s.regs.pc = address;
            
            Nop
        }
        _ => Nop,
    }
}

pub fn ror_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            // save status register
            let temp = s.regs.status;

            // carry flag contains old LSB
            s.regs.set_flag(CpuFlags::C, s.regs.a & 1 == 1);

            s.regs.a >>= 1;

            // new MSB is the old carry flag
            s.regs.a |= (temp & 1) << 7;

            // zero flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a == 0);

            // negative flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn cpx_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value M from memory
        2 => {
            let result = s.regs.x.wrapping_sub(s.data); // CPX performs A - M and sets flags

            // zero flag <- A == M
            s.regs.set_flag(CpuFlags::Z, result == 0);

            // carry flag <- A >= M
            s.regs.set_flag(CpuFlags::C, s.regs.x >= s.data);

            // negative flag
            s.regs.set_flag(CpuFlags::N, result & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn and_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch value from address
        2 => {
            s.regs.a &= s.data; // fetched value

            // zero flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a == 0);
        
            // negative flag
            s.regs.set_flag(CpuFlags::Z, s.regs.a & 0x80 == 0x80);
        
            Nop
        }
        _ => Nop
    }
}

pub fn eor_immediate(s: &mut CpuState, immediate: u8, _cycle: usize) -> BusMessage {
    s.regs.a ^= immediate;

    // zero flag
    s.regs.set_flag(CpuFlags::Z, s.regs.a == 0);

    // negative flag
    s.regs.set_flag(CpuFlags::Z, s.regs.a & 0x80 == 0x80);

    Nop
}

pub fn pla_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            s.regs.sp = s.regs.sp.wrapping_add(1); // increment stack pointer
            Read{addr: 0x100 | s.regs.sp as u16}
        }
        2 => {
            s.regs.a = s.data;
            Nop
        }
        _ => Nop
    }
}

pub fn ora_immediate(s: &mut CpuState, immediate: u8, _cycle: usize) -> BusMessage {
    s.regs.a |= immediate;

    // zero flag
    s.regs.set_flag(CpuFlags::Z, s.regs.a == 0);

    // negative flag
    s.regs.set_flag(CpuFlags::Z, s.regs.a & 0x80 == 0x80);

    Nop
}

pub fn cmp_immediate(s: &mut CpuState, immediate: u8, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            let result = s.regs.a.wrapping_sub(immediate); // CMP performs A - M and sets flags

            // zero flag <- A == M
            s.regs.set_flag(CpuFlags::Z, result == 0);

            // carry flag <- A >= M
            s.regs.set_flag(CpuFlags::C, s.regs.a >= immediate);

            // negative flag
            s.regs.set_flag(CpuFlags::N, result & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn clc_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.set_flag(CpuFlags::C, false);
    Nop
}

pub fn bpl_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            if !s.regs.get_flag(CpuFlags::N) { // negative flag not set
                s.extra_cycle = true;
            }
            Nop
        }
        2 => { // can only be reached if branch condition met
            // branch ops need one extra cycle if the branch jumps across a page boundary
            if s.regs.pc >> 8 != address >> 8 {
                s.extra_cycle = true;
            }

            println!("new address after branch: {:04X}", address);
            s.regs.pc = address;
            
            Nop
        }
        _ => Nop,
    }
}

pub fn bit_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => Read{addr: address}, // fetch operand M
        2 => {
            let result = s.regs.a & s.data; // BIT performs A & M but doesn't store the result

            // zero flag
            s.regs.set_flag(CpuFlags::Z, result == 0);

            // overflow flag <- M6
            s.regs.set_flag(CpuFlags::V, s.data & 0x70 == 0x70);

            // negative flag <- M7
            s.regs.set_flag(CpuFlags::N, s.data & 0x80 == 0x80);

            Nop
        }
        _ => Nop
    }
}

pub fn txa_implied(s: &mut CpuState, cycle: usize) -> BusMessage {
    s.regs.a = s.regs.x;
    Nop
}

pub fn bvc_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            if !s.regs.get_flag(CpuFlags::V) { // overflow flag not set
                s.extra_cycle = true;
            }
            Nop
        }
        2 => { // can only be reached if branch condition met
            // branch ops need one extra cycle if the branch jumps across a page boundary
            if s.regs.pc >> 8 != address >> 8 {
                s.extra_cycle = true;
            }

            println!("new address after branch: {:04X}", address);
            s.regs.pc = address;
            
            Nop
        }
        _ => Nop,
    }
}

pub fn bcc_address(s: &mut CpuState, address: u16, cycle: usize) -> BusMessage {
    match cycle {
        1 => {
            if !s.regs.get_flag(CpuFlags::C) { // carry flag not set
                s.extra_cycle = true;
            }
            Nop
        }
        2 => { // can only be reached if branch condition met
            // branch ops need one extra cycle if the branch jumps across a page boundary
            if s.regs.pc >> 8 != address >> 8 {
                s.extra_cycle = true;
            }

            println!("new address after branch: {:04X}", address);
            s.regs.pc = address;
            
            Nop
        }
        _ => Nop,
    }
}

pub fn reset(s: &mut CpuState, reset_vector: u16, cycle: usize) -> BusMessage {
    match cycle {
        x if x < 6 => Nop,
        6 => {
            s.regs.set_flag(CpuFlags::I, true);
            Read{addr: reset_vector}
        },
        7 => {
            s.regs.pc |= s.data as u16; // set low byte of new PC address
            Read{addr: reset_vector+1}
        },
        8 => {
            s.regs.pc |= (s.data as u16) << 8; // set high byte of ne PC address

            // reset rest of the registers
            s.regs.a = 0x00;
            s.regs.x = 0x00;
            s.regs.y = 0x00;
            s.regs.sp = 0xFD; // default address for stack pointer
            s.regs.status = 0x24; // 3rd bit unused and always high, I flag still set

            // also the relevant helpers
            s.op = 0x00;
            s.o1 = 0x00;
            s.o2 = 0x00;
            
            Read{addr: s.regs.pc}
        },
        _ => panic!("Impossible cycle count in match, reset takes 8 cycles"),
    }
}

pub fn interrupt(s: &mut CpuState, interrupt_vector: u16, cycle: usize) -> BusMessage {
    match cycle {
        x if x < 3 => Nop,
        3 => Write{addr: 0x100 | s.regs.sp as u16, data: (s.regs.pc >> 8) as u8}, // push PC high byte to stack
        4 => {
            s.regs.sp = s.regs.sp.wrapping_sub(1);
            Write{addr: 0x100 | s.regs.sp as u16, data: s.regs.pc as u8} // push PC low byte to stack
        }
        5 => {
            s.regs.sp = s.regs.sp.wrapping_sub(1);
            Write{addr: 0x100 | s.regs.sp as u16, data: s.regs.status & !0x10} // push status register with B forced to 0 to stack
        }
        6 => {
            s.regs.sp = s.regs.sp.wrapping_sub(1);
            Read{addr: interrupt_vector} // fetch PC low byte from vector
        }
        7 => {
            s.regs.pc = s.data as u16; // set PC low byte from stack
            s.regs.set_flag(CpuFlags::I, true); // set interrupt disable flag
            Read{addr: interrupt_vector+1} // fetch PC high byte from vector
        }
        8 => {
            s.regs.pc |= (s.data as u16) << 8; // set PC high byte from stack
            Read{addr: s.regs.pc} // issue fetch for next opcode
        }
        _ => panic!("Impossible cycle count in match, interrupt handling takes 8 cycles")
    }
}
