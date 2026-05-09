use crate::r65816::{R65816, R65816Trait};

impl<T: R65816Trait> R65816<T> {
    pub fn op_nop(&mut self) {
        self.child.last_cycle();
        self.op_io_irq();
    }

    pub fn op_wdm(&mut self) {
        self.child.last_cycle();
        self.op_readpc();
    }

    pub fn op_xba(&mut self) {
        self.child.op_io();
        self.child.last_cycle();
        self.child.op_io();
        *self.regs.a.l_mut() ^= self.regs.a.h();
        *self.regs.a.h_mut() ^= self.regs.a.l();
        *self.regs.a.l_mut() ^= self.regs.a.h();
        self.regs.p.n = (self.regs.a.l() & 0x80) != 0;
        self.regs.p.z = self.regs.a.l() == 0;
    }

    pub fn op_move_b<const ADJUST: i8>(&mut self) {
        self.dp = self.op_readpc();
        self.sp = self.op_readpc();
        self.regs.db = self.dp;
        *self.rd.l_mut() = self.op_readlong(((self.sp as u32) << 16) | self.regs.x.w() as u32);
        self.op_writelong(
            ((self.dp as u32) << 16) | self.regs.y.w() as u32,
            self.rd.l(),
        );
        self.child.op_io();
        *self.regs.x.l_mut() = self.regs.x.l().wrapping_add(ADJUST as u8);
        *self.regs.y.l_mut() = self.regs.y.l().wrapping_add(ADJUST as u8);
        self.child.last_cycle();
        self.child.op_io();
        if self.regs.a.w() != 0 {
            *self.regs.pc.w_mut() = self.regs.pc.w().wrapping_sub(3);
        }
        *self.regs.a.w_mut() = self.regs.a.w().wrapping_sub(1);
    }

    pub fn op_move_w<const ADJUST: i16>(&mut self) {
        self.dp = self.op_readpc();
        self.sp = self.op_readpc();
        self.regs.db = self.dp;
        *self.rd.l_mut() = self.op_readlong(((self.sp as u32) << 16) | self.regs.x.w() as u32);
        self.op_writelong(
            ((self.dp as u32) << 16) | self.regs.y.w() as u32,
            self.rd.l(),
        );
        self.child.op_io();
        *self.regs.x.w_mut() = self.regs.x.w().wrapping_add(ADJUST as u16);
        *self.regs.y.w_mut() = self.regs.y.w().wrapping_add(ADJUST as u16);
        self.child.last_cycle();
        self.child.op_io();
        if self.regs.a.w() != 0 {
            *self.regs.pc.w_mut() = self.regs.pc.w().wrapping_sub(3);
        }
        *self.regs.a.w_mut() = self.regs.a.w().wrapping_sub(1);
    }

    pub fn op_interrupt_e<const VECTOR_E: u32, const VECTOR_N: usize>(&mut self) {
        self.op_readpc();
        self.op_writestack(self.regs.pc.h());
        self.op_writestack(self.regs.pc.l());
        self.op_writestack(self.regs.p.into());
        *self.rd.l_mut() = self.op_readlong(VECTOR_E.wrapping_add(0));
        *self.regs.pc.b_mut() = 0;
        self.regs.p.i = true;
        self.regs.p.d = false;
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readlong(VECTOR_E.wrapping_add(1));
        *self.regs.pc.w_mut() = self.rd.w();
    }

    pub fn op_interrupt_n<const VECTOR_E: u32, const VECTOR_N: u32>(&mut self) {
        self.op_readpc();
        self.op_writestack(self.regs.pc.b());
        self.op_writestack(self.regs.pc.h());
        self.op_writestack(self.regs.pc.l());
        self.op_writestack(self.regs.p.into());
        *self.rd.l_mut() = self.op_readlong(VECTOR_N.wrapping_add(0));
        *self.regs.pc.b_mut() = 0x00;
        self.regs.p.i = true;
        self.regs.p.d = false;
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readlong(VECTOR_N.wrapping_add(1));
        *self.regs.pc.w_mut() = self.rd.w();
    }

    pub fn op_stp(&mut self) {
        while {
            self.regs.wai = true;
            self.regs.wai
        } {
            self.child.last_cycle();
            self.child.op_io();
        }
    }
}
