// code adapted from bsnes-mercury
// https://github.com/libretro/bsnes-mercury

use crate::r65816::{R65816, R65816Trait};

impl<T: R65816Trait> R65816<T> {
    pub fn op_write_addr_b<const N: usize>(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        self.child.last_cycle();
        self.op_writedbr(
            self.aa.d(),
            match N {
                0 => self.regs.a,
                1 => self.regs.x,
                2 => self.regs.y,
                3 => self.regs.z,
                _ => unreachable!(),
            }
            .l(),
        )
    }

    pub fn op_write_addr_w<const N: usize>(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        self.op_writedbr(
            self.aa.d().wrapping_add(0),
            match N {
                0 => self.regs.a,
                1 => self.regs.x,
                2 => self.regs.y,
                3 => self.regs.z,
                _ => unreachable!(),
            }
            .l(),
        );
        self.child.last_cycle();
        self.op_writedbr(
            self.aa.d().wrapping_add(1),
            match N {
                0 => self.regs.a,
                1 => self.regs.x,
                2 => self.regs.y,
                3 => self.regs.z,
                _ => unreachable!(),
            }
            .h(),
        );
    }

    pub fn op_write_addrr_b<const N: usize, const I: usize>(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        self.child.op_io();
        self.child.last_cycle();
        self.op_writedbr(
            self.aa.d().wrapping_add(
                match I {
                    0 => self.regs.a,
                    1 => self.regs.x,
                    2 => self.regs.y,
                    3 => self.regs.z,
                    _ => unreachable!(),
                }
                .w() as u32,
            ),
            match N {
                0 => self.regs.a,
                1 => self.regs.x,
                2 => self.regs.y,
                3 => self.regs.z,
                _ => unreachable!(),
            }
            .l(),
        );
    }

    pub fn op_write_addrr_w<const N: usize, const I: usize>(&mut self) {
        let l = self.op_readpc();
        self.aa.set_l(l);
        let h = self.op_readpc();
        self.aa.set_h(h);
        self.child.op_io();
        self.op_writedbr(
            self.aa
                .d()
                .wrapping_add(
                    match I {
                        0 => self.regs.a,
                        1 => self.regs.x,
                        2 => self.regs.y,
                        3 => self.regs.z,
                        _ => unreachable!(),
                    }
                    .w() as u32,
                )
                .wrapping_add(0),
            match N {
                0 => self.regs.a,
                1 => self.regs.x,
                2 => self.regs.y,
                3 => self.regs.z,
                _ => unreachable!(),
            }
            .l(),
        );
        self.child.last_cycle();
        self.op_writedbr(
            self.aa
                .d()
                .wrapping_add(
                    match I {
                        0 => self.regs.a,
                        1 => self.regs.x,
                        2 => self.regs.y,
                        3 => self.regs.z,
                        _ => unreachable!(),
                    }
                    .w() as u32,
                )
                .wrapping_add(1),
            match N {
                0 => self.regs.a,
                1 => self.regs.x,
                2 => self.regs.y,
                3 => self.regs.z,
                _ => unreachable!(),
            }
            .h(),
        );
    }
}
