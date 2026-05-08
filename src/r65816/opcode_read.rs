// code adapted from bsnes-mercury
// https://github.com/libretro/bsnes-mercury

use crate::r65816::{R65816, R65816Trait};

impl<T: R65816Trait> R65816<T> {
    #[inline(always)]
    pub fn op_read_const_b(&mut self) {
        self.child.last_cycle();
        *self.rd.l_mut() = self.op_readpc();
    }

    #[inline(always)]
    pub fn op_read_const_w(&mut self) {
        *self.rd.l_mut() = self.op_readpc();
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readpc();
    }

    pub fn op_read_bit_const_b(&mut self) {
        self.child.last_cycle();
        *self.rd.l_mut() = self.op_readpc();
        self.regs.p.z = (self.rd.l() & self.regs.a.l()) == 0;
    }

    pub fn op_read_bit_const_w(&mut self) {
        *self.rd.l_mut() = self.op_readpc();
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readpc();
        self.regs.p.z = (self.rd.w() & self.regs.a.w()) == 0;
    }

    #[inline(always)]
    pub fn op_read_addr_b(&mut self) {
        *self.aa.l_mut() = self.op_readpc();
        *self.aa.h_mut() = self.op_readpc();
        self.child.last_cycle();
        *self.rd.l_mut() = self.op_readdbr(self.aa.w() as u32);
    }

    #[inline(always)]
    pub fn op_read_addr_w(&mut self) {
        *self.aa.l_mut() = self.op_readpc();
        *self.aa.h_mut() = self.op_readpc();
        *self.rd.l_mut() = self.op_readdbr((self.aa.w() as u32).wrapping_add(0));
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readdbr((self.aa.w() as u32).wrapping_add(1));
    }

    #[inline(always)]
    pub fn op_read_addrx_b(&mut self) {
        *self.aa.l_mut() = self.op_readpc();
        *self.aa.h_mut() = self.op_readpc();
        self.op_io_cond4(self.aa.w(), self.aa.w() + self.regs.x.w());
        self.child.last_cycle();
        *self.rd.l_mut() =
            self.op_readdbr((self.aa.w() as u32).wrapping_add(self.regs.x.w() as u32));
    }

    #[inline(always)]
    pub fn op_read_addrx_w(&mut self) {
        *self.aa.l_mut() = self.op_readpc();
        *self.aa.h_mut() = self.op_readpc();
        self.op_io_cond4(self.aa.w(), self.aa.w().wrapping_add(self.regs.x.w()));
        *self.rd.l_mut() = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(0),
        );
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(1),
        );
    }

    #[inline(always)]
    pub fn op_read_addry_b(&mut self) {
        *self.aa.l_mut() = self.op_readpc();
        *self.aa.h_mut() = self.op_readpc();
        self.op_io_cond4(self.aa.w(), self.aa.w().wrapping_add(self.regs.y.w()));
        self.child.last_cycle();
        *self.rd.l_mut() =
            self.op_readdbr((self.aa.w() as u32).wrapping_add(self.regs.y.w() as u32));
    }

    #[inline(always)]
    pub fn op_read_addry_w(&mut self) {
        *self.aa.l_mut() = self.op_readpc();
        *self.aa.h_mut() = self.op_readpc();
        self.op_io_cond4(self.aa.w(), self.aa.w().wrapping_add(self.regs.y.w()));
        *self.rd.l_mut() = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(0),
        );
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(1),
        );
    }

    #[inline(always)]
    pub fn op_read_long_b(&mut self) {
        *self.aa.l_mut() = self.op_readpc();
        *self.aa.h_mut() = self.op_readpc();
        *self.aa.b_mut() = self.op_readpc();
        self.child.last_cycle();
        *self.rd.l_mut() = self.op_readlong(self.aa.d());
    }

    #[inline(always)]
    pub fn op_read_long_w(&mut self) {
        *self.aa.l_mut() = self.op_readpc();
        *self.aa.h_mut() = self.op_readpc();
        *self.aa.b_mut() = self.op_readpc();
        *self.rd.l_mut() = self.op_readlong(self.aa.d().wrapping_add(0));
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readlong(self.aa.d().wrapping_add(1));
    }

    #[inline(always)]
    pub fn op_read_longx_b(&mut self) {
        *self.aa.l_mut() = self.op_readpc();
        *self.aa.h_mut() = self.op_readpc();
        *self.aa.b_mut() = self.op_readpc();
        self.child.last_cycle();
        *self.rd.l_mut() = self.op_readlong(self.aa.d().wrapping_add(self.regs.x.w() as u32));
    }

    #[inline(always)]
    pub fn op_read_longx_w(&mut self) {
        *self.aa.l_mut() = self.op_readpc();
        *self.aa.h_mut() = self.op_readpc();
        *self.aa.b_mut() = self.op_readpc();
        *self.rd.l_mut() = self.op_readlong(
            self.aa
                .d()
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(0),
        );
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readlong(
            self.aa
                .d()
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(1),
        );
    }

    #[inline(always)]
    pub fn op_read_dp_b(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        self.child.last_cycle();
        *self.rd.l_mut() = self.op_readdp(self.dp as u32);
    }

    #[inline(always)]
    pub fn op_read_dp_w(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        *self.rd.l_mut() = self.op_readdp((self.dp as u32).wrapping_add(0));
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readdp((self.dp as u32).wrapping_add(1));
    }

    #[inline(always)]
    pub fn op_read_dpr_b<const N: usize>(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        self.child.op_io();
        self.child.last_cycle();
        *self.rd.l_mut() = self.op_readdp(
            (self.dp as u32).wrapping_add(
                match N {
                    1 => self.regs.x,
                    2 => self.regs.y,
                    _ => unreachable!(),
                }
                .w() as u32,
            ),
        );
    }

    #[inline(always)]
    pub fn op_read_dpr_w<const N: usize>(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        self.child.op_io();
        *self.rd.l_mut() = self.op_readdp(
            (self.dp as u32)
                .wrapping_add(
                    match N {
                        1 => self.regs.x,
                        2 => self.regs.y,
                        _ => unreachable!(),
                    }
                    .w() as u32,
                )
                .wrapping_add(0),
        );
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readdp(
            (self.dp as u32)
                .wrapping_add(
                    match N {
                        1 => self.regs.x,
                        2 => self.regs.y,
                        _ => unreachable!(),
                    }
                    .w() as u32,
                )
                .wrapping_add(1),
        );
    }

    #[inline(always)]
    pub fn op_read_idp_b(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        *self.aa.l_mut() = self.op_readdp((self.dp as u32).wrapping_add(0));
        *self.aa.h_mut() = self.op_readdp((self.dp as u32).wrapping_add(1));
        self.child.last_cycle();
        *self.rd.l_mut() = self.op_readdbr(self.aa.w() as u32);
    }

    pub fn op_read_idp_w(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        *self.aa.l_mut() = self.op_readdp((self.dp as u32).wrapping_add(0));
        *self.aa.h_mut() = self.op_readdp((self.dp as u32).wrapping_add(1));
        *self.rd.l_mut() = self.op_readdbr((self.aa.w() as u32).wrapping_add(0));
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readdbr((self.aa.w() as u32).wrapping_add(1));
    }

    pub fn op_read_idpx_b(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        self.child.op_io();
        *self.aa.l_mut() = self.op_readdp(
            (self.dp as u32)
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(0),
        );
        *self.aa.h_mut() = self.op_readdp(
            (self.dp as u32)
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(1),
        );
        self.child.last_cycle();
        *self.rd.l_mut() = self.op_readdbr(self.aa.w() as u32);
    }

    pub fn op_read_idpx_w(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        self.child.op_io();
        *self.aa.l_mut() = self.op_readdp(
            (self.dp as u32)
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(0),
        );
        *self.aa.h_mut() = self.op_readdp(
            (self.dp as u32)
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(1),
        );
        *self.rd.l_mut() = self.op_readdbr((self.aa.w() as u32).wrapping_add(0));
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readdbr((self.aa.w() as u32).wrapping_add(1));
    }

    pub fn op_read_idpy_b(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        *self.aa.l_mut() = self.op_readdp((self.dp as u32).wrapping_add(0));
        *self.aa.h_mut() = self.op_readdp((self.dp as u32).wrapping_add(1));
        self.op_io_cond4(self.aa.w(), self.aa.w().wrapping_add(self.regs.y.w()));
        self.child.last_cycle();
        *self.rd.l_mut() =
            self.op_readdbr((self.aa.w() as u32).wrapping_add(self.regs.y.w() as u32));
    }

    pub fn op_read_idpy_w(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        *self.aa.l_mut() = self.op_readdp((self.dp as u32).wrapping_add(0));

        *self.aa.h_mut() = self.op_readdp((self.dp as u32).wrapping_add(1));
        self.op_io_cond4(self.aa.w(), self.aa.w().wrapping_add(self.regs.y.w()));
        *self.rd.l_mut() = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(0),
        );
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(1),
        );
    }

    pub fn op_read_ildp_b(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        *self.aa.l_mut() = self.op_readdp((self.dp as u32).wrapping_add(0));
        *self.aa.h_mut() = self.op_readdp((self.dp as u32).wrapping_add(1));
        *self.aa.b_mut() = self.op_readdp((self.dp as u32).wrapping_add(2));
        self.child.last_cycle();
        *self.rd.l_mut() = self.op_readlong(self.aa.d());
    }

    pub fn op_read_ildp_w(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        *self.aa.l_mut() = self.op_readdp((self.dp as u32).wrapping_add(0));
        *self.aa.h_mut() = self.op_readdp((self.dp as u32).wrapping_add(1));
        *self.aa.b_mut() = self.op_readdp((self.dp as u32).wrapping_add(2));
        *self.rd.l_mut() = self.op_readlong(self.aa.d().wrapping_add(0));
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readlong(self.aa.d().wrapping_add(1));
    }

    pub fn op_read_ildpy_b(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        *self.aa.l_mut() = self.op_readdp((self.dp as u32).wrapping_add(0));
        *self.aa.h_mut() = self.op_readdp((self.dp as u32).wrapping_add(1));
        *self.aa.b_mut() = self.op_readdp((self.dp as u32).wrapping_add(2));
        self.child.last_cycle();
        *self.rd.l_mut() = self.op_readlong(self.aa.d().wrapping_add(self.regs.y.w() as u32));
    }

    pub fn op_read_ildpy_w(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        *self.aa.l_mut() = self.op_readdp((self.dp as u32).wrapping_add(0));
        *self.aa.h_mut() = self.op_readdp((self.dp as u32).wrapping_add(1));
        *self.aa.b_mut() = self.op_readdp((self.dp as u32).wrapping_add(2));
        *self.rd.l_mut() = self.op_readlong(
            self.aa
                .d()
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(0),
        );
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readlong(
            self.aa
                .d()
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(1),
        );
    }

    pub fn op_read_sr_b(&mut self) {
        self.sp = self.op_readpc();
        self.child.op_io();
        self.child.last_cycle();
        *self.rd.l_mut() = self.op_readsp(self.sp as u32);
    }

    pub fn op_read_sr_w(&mut self) {
        self.sp = self.op_readpc();
        self.child.op_io();
        *self.rd.l_mut() = self.op_readsp((self.sp as u32).wrapping_add(0));
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readsp((self.sp as u32).wrapping_add(1));
    }

    pub fn op_read_isry_b(&mut self) {
        self.sp = self.op_readpc();
        self.child.op_io();
        *self.aa.l_mut() = self.op_readsp((self.sp as u32).wrapping_add(0));
        *self.aa.h_mut() = self.op_readsp((self.sp as u32).wrapping_add(1));
        self.child.op_io();
        self.child.last_cycle();
        *self.rd.l_mut() =
            self.op_readdbr((self.aa.w() as u32).wrapping_add(self.regs.y.w() as u32));
    }

    pub fn op_read_isry_w(&mut self) {
        self.sp = self.op_readpc();
        self.child.op_io();
        *self.aa.l_mut() = self.op_readsp((self.sp as u32).wrapping_add(0));
        *self.aa.h_mut() = self.op_readsp((self.sp as u32).wrapping_add(1));
        self.child.op_io();
        *self.rd.l_mut() = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(0),
        );
        self.child.last_cycle();
        *self.rd.h_mut() = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(1),
        );
    }
}
