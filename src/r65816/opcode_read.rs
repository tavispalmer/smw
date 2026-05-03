// code adapted from bsnes-mercury
// https://github.com/libretro/bsnes-mercury

use crate::r65816::{R65816, R65816Trait};

impl<T: R65816Trait> R65816<T> {
    #[inline(always)]
    pub fn op_read_const_b(&mut self) {
        self.child.last_cycle();
        let l = self.op_readpc();
        self.rd.set_l(l);
    }

    #[inline(always)]
    pub fn op_read_const_w(&mut self) {
        let l = self.op_readpc();
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readpc();
        self.rd.set_h(h);
    }

    pub fn op_read_bit_const_b(&mut self) {
        self.child.last_cycle();
        let l = self.op_readpc();
        self.rd.set_l(l);
        self.regs.p.z = (self.rd.l() & self.regs.a.l()) == 0;
    }

    pub fn op_read_bit_const_w(&mut self) {
        let l = self.op_readpc();
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readpc();
        self.rd.set_h(h);
        self.regs.p.z = (self.rd.w() & self.regs.a.w()) == 0;
    }

    #[inline(always)]
    pub fn op_read_addr_b(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        self.child.last_cycle();
        let l = self.op_readdbr(self.aa.w() as u32);
        self.rd.set_l(l);
    }

    #[inline(always)]
    pub fn op_read_addr_w(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        let l = self.op_readdbr((self.aa.w() as u32).wrapping_add(0));
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readdbr((self.aa.w() as u32).wrapping_add(1));
        self.rd.set_h(h);
    }

    #[inline(always)]
    pub fn op_read_addrx_b(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        self.op_io_cond4(self.aa.w(), self.aa.w() + self.regs.x.w());
        self.child.last_cycle();
        let l = self.op_readdbr((self.aa.w() as u32).wrapping_add(self.regs.x.w() as u32));
        self.rd.set_l(l);
    }

    #[inline(always)]
    pub fn op_read_addrx_w(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        self.op_io_cond4(self.aa.w(), self.aa.w().wrapping_add(self.regs.x.w()));
        let l = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(0),
        );
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(1),
        );
        self.rd.set_h(h);
    }

    #[inline(always)]
    pub fn op_read_addry_b(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        self.op_io_cond4(self.aa.w(), self.aa.w().wrapping_add(self.regs.y.w()));
        self.child.last_cycle();
        let l = self.op_readdbr((self.aa.w() as u32).wrapping_add(self.regs.y.w() as u32));
        self.rd.set_l(l);
    }

    #[inline(always)]
    pub fn op_read_addry_w(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        self.op_io_cond4(self.aa.w(), self.aa.w().wrapping_add(self.regs.y.w()));
        let l = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(0),
        );
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(1),
        );
        self.rd.set_h(h);
    }

    #[inline(always)]
    pub fn op_read_long_b(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        let b = self.op_readpc();
        self.aa.set_b(b);
        self.child.last_cycle();
        let l = self.op_readlong(self.aa.d());
        self.rd.set_l(l);
    }

    #[inline(always)]
    pub fn op_read_long_w(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        let b = self.op_readpc();
        self.aa.set_b(b);
        let l = self.op_readlong(self.aa.d().wrapping_add(0));
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readlong(self.aa.d().wrapping_add(1));
        self.rd.set_h(h);
    }

    #[inline(always)]
    pub fn op_read_longx_b(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        let b = self.op_readpc();
        self.aa.set_b(b);
        self.child.last_cycle();
        let l = self.op_readlong(self.aa.d().wrapping_add(self.regs.x.w() as u32));
        self.rd.set_l(l);
    }

    #[inline(always)]
    pub fn op_read_longx_w(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        let b = self.op_readpc();
        self.aa.set_b(b);
        let l = self.op_readlong(
            self.aa
                .d()
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(0),
        );
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readlong(
            self.aa
                .d()
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(1),
        );
        self.rd.set_h(h);
    }

    #[inline(always)]
    pub fn op_read_dp_b(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        self.child.last_cycle();
        let l = self.op_readdp(self.dp as u32);
        self.rd.set_l(l);
    }

    #[inline(always)]
    pub fn op_read_dp_w(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        let l = self.op_readdp((self.dp as u32).wrapping_add(0));
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readdp((self.dp as u32).wrapping_add(1));
        self.rd.set_h(h);
    }

    #[inline(always)]
    pub fn op_read_dpr_b<const N: usize>(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        self.child.op_io();
        self.child.last_cycle();
        let l = self.op_readdp(
            (self.dp as u32).wrapping_add(
                match N {
                    1 => self.regs.x,
                    2 => self.regs.y,
                    _ => unreachable!(),
                }
                .w() as u32,
            ),
        );
        self.rd.set_l(l);
    }

    #[inline(always)]
    pub fn op_read_dpr_w<const N: usize>(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        self.child.op_io();
        let l = self.op_readdp(
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
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readdp(
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
        self.rd.set_h(h);
    }

    #[inline(always)]
    pub fn op_read_idp_b(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        let l = self.op_readdp((self.dp as u32).wrapping_add(0));
        self.aa.set_l(l);
        let h = self.op_readdp((self.dp as u32).wrapping_add(1));
        self.aa.set_h(h);
        self.child.last_cycle();
        let l = self.op_readdbr(self.aa.w() as u32);
        self.rd.set_l(l);
    }

    pub fn op_read_idp_w(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        let l = self.op_readdp((self.dp as u32).wrapping_add(0));
        self.aa.set_l(l);
        let h = self.op_readdp((self.dp as u32).wrapping_add(1));
        self.aa.set_h(h);
        let l = self.op_readdbr((self.aa.w() as u32).wrapping_add(0));
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readdbr((self.aa.w() as u32).wrapping_add(1));
        self.rd.set_h(h);
    }

    pub fn op_read_idpx_b(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        self.child.op_io();
        let l = self.op_readdp(
            (self.dp as u32)
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(0),
        );
        self.aa.set_l(l);
        let h = self.op_readdp(
            (self.dp as u32)
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(1),
        );
        self.aa.set_h(h);
        self.child.last_cycle();
        let l = self.op_readdbr(self.aa.w() as u32);
        self.rd.set_l(l);
    }

    pub fn op_read_idpx_w(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        self.child.op_io();
        let l = self.op_readdp(
            (self.dp as u32)
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(0),
        );
        self.aa.set_l(l);
        let h = self.op_readdp(
            (self.dp as u32)
                .wrapping_add(self.regs.x.w() as u32)
                .wrapping_add(1),
        );
        self.aa.set_h(h);
        let l = self.op_readdbr((self.aa.w() as u32).wrapping_add(0));
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readdbr((self.aa.w() as u32).wrapping_add(1));
        self.rd.set_h(h);
    }

    pub fn op_read_idpy_b(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        let l = self.op_readdp((self.dp as u32).wrapping_add(0));
        self.aa.set_l(l);
        let h = self.op_readdp((self.dp as u32).wrapping_add(1));
        self.aa.set_h(h);
        self.op_io_cond4(self.aa.w(), self.aa.w().wrapping_add(self.regs.y.w()));
        self.child.last_cycle();
        let l = self.op_readdbr((self.aa.w() as u32).wrapping_add(self.regs.y.w() as u32));
        self.rd.set_l(l);
    }

    pub fn op_read_idpy_w(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        let l = self.op_readdp((self.dp as u32).wrapping_add(0));
        self.aa.set_l(l);
        let h = self.op_readdp((self.dp as u32).wrapping_add(1));
        self.aa.set_h(h);
        self.op_io_cond4(self.aa.w(), self.aa.w().wrapping_add(self.regs.y.w()));
        let l = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(0),
        );
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(1),
        );
        self.rd.set_h(h);
    }

    pub fn op_read_ildp_b(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        let l = self.op_readdp((self.dp as u32).wrapping_add(0));
        self.aa.set_l(l);
        let h = self.op_readdp((self.dp as u32).wrapping_add(1));
        self.aa.set_h(h);
        let b = self.op_readdp((self.dp as u32).wrapping_add(2));
        self.aa.set_b(b);
        self.child.last_cycle();
        let l = self.op_readlong(self.aa.d());
        self.rd.set_l(l);
    }

    pub fn op_read_ildp_w(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        let l = self.op_readdp((self.dp as u32).wrapping_add(0));
        self.aa.set_l(l);
        let h = self.op_readdp((self.dp as u32).wrapping_add(1));
        self.aa.set_h(h);
        let b = self.op_readdp((self.dp as u32).wrapping_add(2));
        self.aa.set_b(b);
        let l = self.op_readlong(self.aa.d().wrapping_add(0));
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readlong(self.aa.d().wrapping_add(1));
        self.rd.set_h(h);
    }

    pub fn op_read_ildpy_b(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        let l = self.op_readdp((self.dp as u32).wrapping_add(0));
        self.aa.set_l(l);
        let h = self.op_readdp((self.dp as u32).wrapping_add(1));
        self.aa.set_h(h);
        let b = self.op_readdp((self.dp as u32).wrapping_add(2));
        self.aa.set_b(b);
        self.child.last_cycle();
        let l = self.op_readlong(self.aa.d().wrapping_add(self.regs.y.w() as u32));
        self.rd.set_l(l);
    }

    pub fn op_read_ildpy_w(&mut self) {
        self.dp = self.op_readpc();
        self.op_io_cond2();
        let l = self.op_readdp((self.dp as u32).wrapping_add(0));
        self.aa.set_l(l);
        let h = self.op_readdp((self.dp as u32).wrapping_add(1));
        self.aa.set_h(h);
        let b = self.op_readdp((self.dp as u32).wrapping_add(2));
        self.aa.set_b(b);
        let l = self.op_readlong(
            self.aa
                .d()
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(0),
        );
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readlong(
            self.aa
                .d()
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(1),
        );
        self.rd.set_h(h);
    }

    pub fn op_read_sr_b(&mut self) {
        self.sp = self.op_readpc();
        self.child.op_io();
        self.child.last_cycle();
        let l = self.op_readsp(self.sp as u32);
        self.rd.set_l(l);
    }

    pub fn op_read_sr_w(&mut self) {
        self.sp = self.op_readpc();
        self.child.op_io();
        let l = self.op_readsp((self.sp as u32).wrapping_add(0));
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readsp((self.sp as u32).wrapping_add(1));
        self.rd.set_h(h);
    }

    pub fn op_read_isry_b(&mut self) {
        self.sp = self.op_readpc();
        self.child.op_io();
        let l = self.op_readsp((self.sp as u32).wrapping_add(0));
        self.aa.set_l(l);
        let h = self.op_readsp((self.sp as u32).wrapping_add(1));
        self.aa.set_h(h);
        self.child.op_io();
        self.child.last_cycle();
        let l = self.op_readdbr((self.aa.w() as u32).wrapping_add(self.regs.y.w() as u32));
        self.rd.set_l(l);
    }

    pub fn op_read_isry_w(&mut self) {
        self.sp = self.op_readpc();
        self.child.op_io();
        let l = self.op_readsp((self.sp as u32).wrapping_add(0));
        self.aa.set_l(l);
        let h = self.op_readsp((self.sp as u32).wrapping_add(1));
        self.aa.set_h(h);
        self.child.op_io();
        let l = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(0),
        );
        self.rd.set_l(l);
        self.child.last_cycle();
        let h = self.op_readdbr(
            (self.aa.w() as u32)
                .wrapping_add(self.regs.y.w() as u32)
                .wrapping_add(1),
        );
        self.rd.set_h(h);
    }
}
