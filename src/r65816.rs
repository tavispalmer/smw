// code adapted from bsnes-mercury
// https://github.com/libretro/bsnes-mercury

use crate::r65816::registers::{Reg24, Regs};

mod algorithms;
mod memory;
mod opcode_read;
mod opcode_write;
mod registers;
mod table;

pub struct R65816<T: R65816Trait> {
    regs: Regs,
    aa: Reg24,
    rd: Reg24,
    sp: u8,
    dp: u8,
    op_table: [fn(&mut Self); 5 * 256],
    child: T,
}

pub trait R65816Trait {
    fn op_io(&mut self);
    fn op_read(&mut self, addr: u32) -> u8;
    fn op_write(&mut self, addr: u32, data: u8);
    fn last_cycle(&mut self);
    fn interrupt_pending(&mut self) -> bool;
    fn op_irq(&mut self);

    fn disassembler_read(&mut self, _addr: u32) -> u8 {
        0
    }
}

impl<T: R65816Trait> R65816<T> {
    //immediate, 2-cycle opcodes with I/O cycle will become bus read
    //when an IRQ is to be triggered immediately after opcode completion.
    //this affects the following opcodes:
    //  clc, cld, cli, clv, sec, sed, sei,
    //  tax, tay, txa, txy, tya, tyx,
    //  tcd, tcs, tdc, tsc, tsx, txs,
    //  inc, inx, iny, dec, dex, dey,
    //  asl, lsr, rol, ror, nop, xce.
    #[inline(always)]
    pub fn op_io_irq(&mut self) {
        if self.child.interrupt_pending() {
            //modify I/O cycle to bus read cycle, do not increment PC
            self.child.op_read(self.regs.pc.d());
        } else {
            self.child.op_io();
        }
    }
    #[inline(always)]
    pub fn op_io_cond2(&mut self) {
        if self.regs.d.l() != 0x00 {
            self.child.op_io();
        }
    }
    #[inline(always)]
    pub fn op_io_cond4(&mut self, x: u16, y: u16) {
        if !self.regs.p.x || (x & 0xff00) != (y & 0xff00) {
            self.child.op_io();
        }
    }
    #[inline(always)]
    pub fn op_io_cond6(&mut self, addr: u16) {
        if self.regs.e && (self.regs.pc.w() & 0xff00) != (addr & 0xff00) {
            self.child.op_io();
        }
    }

    pub fn op_irq(&mut self) {
        self.child.op_read(self.regs.pc.d());
        self.child.op_io();
        if !self.regs.e {
            self.op_writestack(self.regs.pc.b())
        }
        self.op_writestack(self.regs.pc.h());
        self.op_writestack(self.regs.pc.l());
        self.op_writestack(if self.regs.e {
            self.regs.p & !0x10
        } else {
            self.regs.p.into()
        });
        *self.rd.l_mut() = self
            .child
            .op_read((self.regs.vector as u32).wrapping_add(0));
        *self.regs.pc.b_mut() = 0x00;
        self.regs.p.i = true;
        self.regs.p.d = false;
        *self.rd.h_mut() = self
            .child
            .op_read((self.regs.vector as u32).wrapping_add(1));
        *self.regs.pc.w_mut() = self.rd.w();
    }

    pub fn new(child: T) -> Self {
        Self {
            regs: Regs::new(),
            aa: Reg24::new(),
            rd: Reg24::new(),
            sp: 0,
            dp: 0,
            op_table: Self::initialize_opcode_table(),
            child,
        }
    }
}
