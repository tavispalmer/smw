// code adapted from bsnes-mercury
// https://github.com/libretro/bsnes-mercury

use crate::r65816::{R65816, R65816Trait};

impl<T: R65816Trait> R65816<T> {
    #[inline(always)]
    pub fn op_adc_b(&mut self) {
        let mut result;

        if !self.regs.p.d {
            result = self.regs.a.l() as u16 + self.rd.l() as u16 + self.regs.p.c as u16;
        } else {
            result = (self.regs.a.l() as u16 & 0x0f)
                + (self.rd.l() as u16 & 0x0f)
                + ((self.regs.p.c as u16) << 0);
            if result > 0x09 {
                result += 0x06
            }
            self.regs.p.c = result > 0x0f;
            result = (self.regs.a.l() as u16 & 0xf0)
                + (self.rd.l() as u16 & 0xf0)
                + ((self.regs.p.c as u16) << 4)
                + (result & 0x0f);
        }

        self.regs.p.v = (!(self.regs.a.l() as u16 ^ self.rd.l() as u16)
            & (self.regs.a.l() as u16 ^ result)
            & 0x80)
            != 0;
        if self.regs.p.d && result > 0x9f {
            result += 0x60
        }
        self.regs.p.c = result > 0xff;
        self.regs.p.n = (result & 0x80) != 0;
        self.regs.p.z = result as u8 == 0;

        self.regs.a.set_l(result as u8);
    }

    #[inline(always)]
    pub fn op_adc_w(&mut self) {
        let mut result;

        if !self.regs.p.d {
            result = self.regs.a.w() as u32 + self.rd.w() as u32 + self.regs.p.c as u32;
        } else {
            result = (self.regs.a.w() as u32 & 0x000f)
                + (self.rd.w() as u32 & 0x000f)
                + ((self.regs.p.c as u32) << 0);
            if result > 0x0009 {
                result += 0x0006
            }
            self.regs.p.c = result > 0x000f;
            result = (self.regs.a.w() as u32 & 0x00f0)
                + (self.rd.w() as u32 & 0x00f0)
                + ((self.regs.p.c as u32) << 4)
                + (result & 0x000f);
            if result > 0x009f {
                result += 0x0060
            }
            self.regs.p.c = result > 0x00ff;
            result = (self.regs.a.w() as u32 & 0x0f00)
                + (self.rd.w() as u32 & 0x0f00)
                + ((self.regs.p.c as u32) << 8)
                + (result & 0x00ff);
            if result > 0x09ff {
                result += 0x0600
            }
            self.regs.p.c = result > 0x0fff;
            result = (self.regs.a.w() as u32 & 0xf000)
                + (self.rd.w() as u32 & 0xf000)
                + ((self.regs.p.c as u32) << 12)
                + (result & 0x0fff);
        }

        self.regs.p.v = (!(self.regs.a.w() as u32 ^ self.rd.w() as u32)
            & (self.regs.a.w() as u32 ^ result)
            & 0x8000)
            != 0;
        if self.regs.p.d && result > 0x9fff {
            result += 0x6000
        }
        self.regs.p.c = result > 0xffff;
        self.regs.p.n = (result & 0x8000) != 0;
        self.regs.p.z = result as u16 == 0;

        self.regs.a.set_w(result as u16);
    }

    #[inline(always)]
    pub fn op_and_b(&mut self) {
        self.regs.a.set_l(self.regs.a.l() & self.rd.l());
        self.regs.p.n = (self.regs.a.l() & 0x80) != 0;
        self.regs.p.z = self.regs.a.l() == 0;
    }

    #[inline(always)]
    pub fn op_and_w(&mut self) {
        self.regs.a.set_w(self.regs.a.w() & self.rd.w());
        self.regs.p.n = (self.regs.a.w() & 0x8000) != 0;
        self.regs.p.z = self.regs.a.w() == 0;
    }

    #[inline(always)]
    pub fn op_bit_b(&mut self) {
        self.regs.p.n = (self.rd.l() & 0x80) != 0;
        self.regs.p.v = (self.rd.l() & 0x40) != 0;
        self.regs.p.z = (self.rd.l() & self.regs.a.l()) == 0;
    }

    #[inline(always)]
    pub fn op_bit_w(&mut self) {
        self.regs.p.n = (self.rd.w() & 0x8000) != 0;
        self.regs.p.v = (self.rd.w() & 0x4000) != 0;
        self.regs.p.z = (self.rd.w() & self.regs.a.w()) == 0;
    }

    #[inline(always)]
    pub fn op_cmp_b(&mut self) {
        let r = self.regs.a.l() as i16 - self.rd.l() as i16;
        self.regs.p.n = (r & 0x80) != 0;
        self.regs.p.z = r as u8 == 0;
        self.regs.p.c = r >= 0;
    }

    #[inline(always)]
    pub fn op_cmp_w(&mut self) {
        let r = self.regs.a.w() as i32 - self.rd.w() as i32;
        self.regs.p.n = (r & 0x8000) != 0;
        self.regs.p.z = r as u16 == 0;
        self.regs.p.c = r >= 0;
    }

    #[inline(always)]
    pub fn op_cpx_b(&mut self) {
        let r = self.regs.x.l() as i16 - self.rd.l() as i16;
        self.regs.p.n = (r & 0x80) != 0;
        self.regs.p.z = r as u8 == 0;
        self.regs.p.c = r >= 0;
    }

    #[inline(always)]
    pub fn op_cpx_w(&mut self) {
        let r = self.regs.x.w() as i32 - self.rd.w() as i32;
        self.regs.p.n = (r & 0x8000) != 0;
        self.regs.p.z = r as u16 == 0;
        self.regs.p.c = r >= 0;
    }

    #[inline(always)]
    pub fn op_cpy_b(&mut self) {
        let r = self.regs.y.l() as i16 - self.rd.l() as i16;
        self.regs.p.n = (r & 0x80) != 0;
        self.regs.p.z = r as u8 == 0;
        self.regs.p.c = r >= 0;
    }

    #[inline(always)]
    pub fn op_cpy_w(&mut self) {
        let r = self.regs.y.w() as i32 - self.rd.w() as i32;
        self.regs.p.n = (r & 0x8000) != 0;
        self.regs.p.z = r as u16 == 0;
        self.regs.p.c = r >= 0;
    }

    #[inline(always)]
    pub fn op_eor_b(&mut self) {
        self.regs.a.set_l(self.regs.a.l() ^ self.rd.l());
        self.regs.p.n = (self.regs.a.l() & 0x80) != 0;
        self.regs.p.z = self.regs.a.l() == 0;
    }

    #[inline(always)]
    pub fn op_eor_w(&mut self) {
        self.regs.a.set_w(self.regs.a.w() ^ self.rd.w());
        self.regs.p.n = (self.regs.a.w() & 0x8000) != 0;
        self.regs.p.z = self.regs.a.w() == 0;
    }

    #[inline(always)]
    pub fn op_lda_b(&mut self) {
        self.regs.a.set_l(self.rd.l());
        self.regs.p.n = (self.regs.a.l() & 0x80) != 0;
        self.regs.p.z = self.regs.a.l() == 0;
    }

    #[inline(always)]
    pub fn op_lda_w(&mut self) {
        self.regs.a.set_w(self.rd.w());
        self.regs.p.n = (self.regs.a.w() & 0x8000) != 0;
        self.regs.p.z = self.regs.a.w() == 0;
    }

    #[inline(always)]
    pub fn op_ldx_b(&mut self) {
        self.regs.x.set_l(self.rd.l());
        self.regs.p.n = (self.regs.x.l() & 0x80) != 0;
        self.regs.p.z = self.regs.x.l() == 0;
    }

    #[inline(always)]
    pub fn op_ldx_w(&mut self) {
        self.regs.x.set_w(self.rd.w());
        self.regs.p.n = (self.regs.x.w() & 0x8000) != 0;
        self.regs.p.z = self.regs.x.w() == 0;
    }

    #[inline(always)]
    pub fn op_ldy_b(&mut self) {
        self.regs.y.set_l(self.rd.l());
        self.regs.p.n = (self.regs.y.l() & 0x80) != 0;
        self.regs.p.z = self.regs.y.l() == 0;
    }

    #[inline(always)]
    pub fn op_ldy_w(&mut self) {
        self.regs.y.set_w(self.rd.w());
        self.regs.p.n = (self.regs.y.w() & 0x8000) != 0;
        self.regs.p.z = self.regs.y.w() == 0;
    }

    #[inline(always)]
    pub fn op_ora_b(&mut self) {
        self.regs.a.set_l(self.regs.a.l() | self.rd.l());
        self.regs.p.n = (self.regs.a.l() & 0x80) != 0;
        self.regs.p.z = self.regs.a.l() == 0;
    }

    #[inline(always)]
    pub fn op_ora_w(&mut self) {
        self.regs.a.set_w(self.regs.a.w() | self.rd.w());
        self.regs.p.n = (self.regs.a.w() & 0x8000) != 0;
        self.regs.p.z = self.regs.a.w() == 0;
    }

    #[inline(always)]
    pub fn op_sbc_b(&mut self) {
        let mut result;
        self.rd.set_l(self.rd.l() ^ 0xff);

        if !self.regs.p.d {
            result = self.regs.a.l() as u16 + self.rd.l() as u16 + self.regs.p.c as u16;
        } else {
            result = (self.regs.a.l() as u16 & 0x0f)
                + (self.rd.l() as u16 & 0x0f)
                + ((self.regs.p.c as u16) << 0);
            if result <= 0x0f {
                result -= 0x06
            }
            self.regs.p.c = result > 0x0f;
            result = (self.regs.a.l() as u16 & 0xf0)
                + (self.rd.l() as u16 & 0xf0)
                + ((self.regs.p.c as u16) << 4)
                + (result & 0x0f);
        }

        self.regs.p.v = (!(self.regs.a.l() as u16 ^ self.rd.l() as u16)
            & (self.regs.a.l() as u16 ^ result)
            & 0x80)
            != 0;
        if self.regs.p.d && result <= 0xff {
            result -= 0x60
        }
        self.regs.p.c = result > 0xff;
        self.regs.p.n = (result & 0x80) != 0;
        self.regs.p.z = result as u8 == 0;

        self.regs.a.set_l(result as u8);
    }

    #[inline(always)]
    pub fn op_sbc_w(&mut self) {
        let mut result;
        self.rd.set_w(self.rd.w() ^ 0xffff);

        if !self.regs.p.d {
            result = self.regs.a.w() as u32 + self.rd.w() as u32 + self.regs.p.c as u32;
        } else {
            result = (self.regs.a.w() as u32 & 0x000f)
                + (self.rd.w() as u32 & 0x000f)
                + ((self.regs.p.c as u32) << 0);
            if result <= 0x000f {
                result -= 0x0006
            }
            self.regs.p.c = result > 0x000f;
            result = (self.regs.a.w() as u32 & 0x00f0)
                + (self.rd.w() as u32 & 0x00f0)
                + ((self.regs.p.c as u32) << 4)
                + (result & 0x000f);
            if result <= 0x00ff {
                result -= 0x0060
            }
            self.regs.p.c = result > 0x00ff;
            result = (self.regs.a.w() as u32 & 0x0f00)
                + (self.rd.w() as u32 & 0x0f00)
                + ((self.regs.p.c as u32) << 8)
                + (result & 0x00ff);
            if result <= 0x0fff {
                result -= 0x0600
            }
            self.regs.p.c = result > 0x0fff;
            result = (self.regs.a.w() as u32 & 0xf000)
                + (self.rd.w() as u32 & 0xf000)
                + ((self.regs.p.c as u32) << 12)
                + (result & 0x0fff);
        }

        self.regs.p.v = (!(self.regs.a.w() as u32 ^ self.rd.w() as u32)
            & (self.regs.a.w() as u32 ^ result)
            & 0x8000)
            != 0;
        if self.regs.p.d && result <= 0xffff {
            result -= 0x6000
        }
        self.regs.p.c = result > 0xffff;
        self.regs.p.n = (result & 0x8000) != 0;
        self.regs.p.z = result as u16 == 0;

        self.regs.a.set_w(result as u16);
    }

    #[inline(always)]
    pub fn op_inc_b(&mut self) {
        self.rd.set_l(self.rd.l().wrapping_add(1));
        self.regs.p.n = (self.rd.l() & 0x80) != 0;
        self.regs.p.z = self.rd.l() == 0;
    }

    #[inline(always)]
    pub fn op_inc_w(&mut self) {
        self.rd.set_w(self.rd.w().wrapping_add(1));
        self.regs.p.n = (self.rd.w() & 0x8000) != 0;
        self.regs.p.z = self.rd.w() == 0;
    }

    #[inline(always)]
    pub fn op_dec_b(&mut self) {
        self.rd.set_l(self.rd.l().wrapping_sub(1));
        self.regs.p.n = (self.rd.l() & 0x80) != 0;
        self.regs.p.z = self.rd.l() == 0;
    }

    #[inline(always)]
    pub fn op_dec_w(&mut self) {
        self.rd.set_w(self.rd.w().wrapping_sub(1));
        self.regs.p.n = (self.rd.w() & 0x8000) != 0;
        self.regs.p.z = self.rd.w() == 0;
    }

    #[inline(always)]
    pub fn op_asl_b(&mut self) {
        self.regs.p.c = (self.rd.l() & 0x80) != 0;
        self.rd.set_l(self.rd.l() << 1);
        self.regs.p.n = (self.rd.l() & 0x80) != 0;
        self.regs.p.z = self.rd.l() == 0;
    }

    #[inline(always)]
    pub fn op_asl_w(&mut self) {
        self.regs.p.c = (self.rd.w() & 0x8000) != 0;
        self.rd.set_w(self.rd.w() << 1);
        self.regs.p.n = (self.rd.w() & 0x8000) != 0;
        self.regs.p.z = self.rd.w() == 0;
    }

    #[inline(always)]
    pub fn op_lsr_b(&mut self) {
        self.regs.p.c = (self.rd.l() & 1) != 0;
        self.rd.set_l(self.rd.l() >> 1);
        self.regs.p.n = (self.rd.l() & 0x80) != 0;
        self.regs.p.z = self.rd.l() == 0;
    }

    #[inline(always)]
    pub fn op_lsr_w(&mut self) {
        self.regs.p.c = (self.rd.w() & 1) != 0;
        self.rd.set_w(self.rd.w() >> 1);
        self.regs.p.n = (self.rd.w() & 0x8000) != 0;
        self.regs.p.z = self.rd.w() == 0;
    }

    #[inline(always)]
    pub fn op_rol_b(&mut self) {
        let carry = self.regs.p.c as u8;
        self.regs.p.c = (self.rd.l() & 0x80) != 0;
        self.rd.set_l((self.rd.l() << 1) | carry);
        self.regs.p.n = (self.rd.l() & 0x80) != 0;
        self.regs.p.z = self.rd.l() == 0;
    }

    #[inline(always)]
    pub fn op_rol_w(&mut self) {
        let carry = self.regs.p.c as u16;
        self.regs.p.c = (self.rd.w() & 0x8000) != 0;
        self.rd.set_w((self.rd.w() << 1) | carry);
        self.regs.p.n = (self.rd.w() & 0x8000) != 0;
        self.regs.p.z = self.rd.w() == 0;
    }

    #[inline(always)]
    pub fn op_ror_b(&mut self) {
        let carry = (self.regs.p.c as u8) << 7;
        self.regs.p.c = (self.rd.l() & 1) != 0;
        self.rd.set_l(carry | (self.rd.l() >> 1));
        self.regs.p.n = (self.rd.l() & 0x80) != 0;
        self.regs.p.z = self.rd.l() == 0;
    }

    #[inline(always)]
    pub fn op_ror_w(&mut self) {
        let carry = (self.regs.p.c as u16) << 15;
        self.regs.p.c = (self.rd.w() & 1) != 0;
        self.rd.set_w(carry | (self.rd.w() >> 1));
        self.regs.p.n = (self.rd.w() & 0x8000) != 0;
        self.regs.p.z = self.rd.w() == 0;
    }

    #[inline(always)]
    pub fn op_trb_b(&mut self) {
        self.regs.p.z = (self.rd.l() & self.regs.a.l()) == 0;
        self.rd.set_l(self.rd.l() & !self.regs.a.l());
    }

    #[inline(always)]
    pub fn op_trb_w(&mut self) {
        self.regs.p.z = (self.rd.w() & self.regs.a.w()) == 0;
        self.rd.set_w(self.rd.w() & !self.regs.a.w());
    }

    #[inline(always)]
    pub fn op_tsb_b(&mut self) {
        self.regs.p.z = (self.rd.l() & self.regs.a.l()) == 0;
        self.rd.set_l(self.rd.l() | self.regs.a.l());
    }

    #[inline(always)]
    pub fn op_tsb_w(&mut self) {
        self.regs.p.z = (self.rd.w() & self.regs.a.w()) == 0;
        self.rd.set_w(self.rd.w() | self.regs.a.w());
    }
}
