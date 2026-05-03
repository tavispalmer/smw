// code adapted from bsnes-mercury
// https://github.com/libretro/bsnes-mercury

use crate::r65816::{R65816, R65816Trait};

impl<T: R65816Trait> R65816<T> {
    #[inline(always)]
    pub fn op_readpc(&mut self) -> u8 {
        let r = self
            .child
            .op_read(((self.regs.pc.b() as u32) << 16).wrapping_add(self.regs.pc.w() as u32));
        self.regs.pc.set_w(self.regs.pc.w().wrapping_add(1));
        r
    }

    #[inline(always)]
    pub fn op_readstack(&mut self) -> u8 {
        if self.regs.e {
            self.regs.s.set_l(self.regs.s.l().wrapping_add(1));
        } else {
            self.regs.s.set_w(self.regs.s.w().wrapping_add(1));
        }
        self.child.op_read(self.regs.s.w() as u32)
    }

    #[inline(always)]
    pub fn op_readstackn(&mut self) -> u8 {
        self.regs.s.set_w(self.regs.s.w().wrapping_add(1));
        self.child.op_read(self.regs.s.w() as u32)
    }

    #[inline(always)]
    pub fn op_readaddr(&mut self, addr: u32) -> u8 {
        self.child.op_read(addr & 0xffff)
    }

    #[inline(always)]
    pub fn op_readlong(&mut self, addr: u32) -> u8 {
        self.child.op_read(addr & 0xffffff)
    }

    #[inline(always)]
    pub fn op_readdbr(&mut self, addr: u32) -> u8 {
        self.child
            .op_read(((self.regs.db as u32) << 16).wrapping_add(addr) & 0xffffff)
    }

    #[inline(always)]
    pub fn op_readpbr(&mut self, addr: u32) -> u8 {
        self.child
            .op_read(((self.regs.pc.b() as u32) << 16).wrapping_add(addr & 0xffff))
    }

    #[inline(always)]
    pub fn op_readdp(&mut self, addr: u32) -> u8 {
        if self.regs.e && self.regs.d.l() == 0x00 {
            self.child.op_read(
                (self.regs.d.w() as u32 & 0xff00)
                    .wrapping_add(((self.regs.d.w() as u32).wrapping_add(addr & 0xffff)) & 0xff),
            )
        } else {
            self.child
                .op_read(((self.regs.d.w() as u32).wrapping_add(addr & 0xffff)) & 0xffff)
        }
    }

    #[inline(always)]
    pub fn op_readsp(&mut self, addr: u32) -> u8 {
        self.child
            .op_read(((self.regs.s.w() as u32).wrapping_add(addr & 0xffff)) & 0xffff)
    }

    #[inline(always)]
    pub fn op_writestack(&mut self, data: u8) {
        self.child.op_write(self.regs.s.w() as u32, data);
        if self.regs.e {
            self.regs.s.set_l(self.regs.s.l().wrapping_sub(1))
        } else {
            self.regs.s.set_w(self.regs.s.w().wrapping_sub(1))
        }
    }

    #[inline(always)]
    pub fn op_writestackn(&mut self, data: u8) {
        self.child.op_write(self.regs.s.w() as u32, data);
        self.regs.s.set_w(self.regs.s.w().wrapping_sub(1))
    }

    #[inline(always)]
    pub fn op_writeaddr(&mut self, addr: u32, data: u8) {
        self.child.op_write(addr & 0xffff, data)
    }

    #[inline(always)]
    pub fn op_writelong(&mut self, addr: u32, data: u8) {
        self.child.op_write(addr & 0xffffff, data)
    }

    #[inline(always)]
    pub fn op_writedbr(&mut self, addr: u32, data: u8) {
        self.child.op_write(
            (((self.regs.db as u32) << 16).wrapping_add(addr)) & 0xffffff,
            data,
        )
    }

    #[inline(always)]
    pub fn op_writepbr(&mut self, addr: u32, data: u8) {
        self.child.op_write(
            ((self.regs.pc.b() as u32) << 16).wrapping_add(addr & 0xffff),
            data,
        )
    }

    #[inline(always)]
    pub fn op_writedp(&mut self, addr: u32, data: u8) {
        if self.regs.e && self.regs.d.l() == 0x00 {
            self.child.op_write(
                (self.regs.d.w() as u32 & 0xff00)
                    .wrapping_add(((self.regs.d.w() as u32).wrapping_add(addr & 0xffff)) & 0xff),
                data,
            )
        } else {
            self.child.op_write(
                ((self.regs.d.w() as u32).wrapping_add(addr & 0xffff)) & 0xffff,
                data,
            )
        }
    }

    #[inline(always)]
    pub fn op_writesp(&mut self, addr: u32, data: u8) {
        self.child.op_write(
            ((self.regs.s.w() as u32).wrapping_add(addr & 0xffff)) & 0xffff,
            data,
        )
    }
}
