use crate::mem::Mem;

struct Cpu {
    // cpu state
    a: u16,
    x: u16,
    y: u16,
    sp: u16,
    d: u16,
    pc: u16,
    k: u8,
    dbr: u8,
    ps: u8,
    emulation_mode: bool,

    // memory
    mem: Mem,

    // other
    read_write_mask: u32,
    operand: u32,
}

impl Cpu {
    const CARRY: u8 = 0x01;
    const ZERO: u8 = 0x02;
    const IRQ_DISABLE: u8 = 0x04;
    const DECIMAL: u8 = 0x08;
    const INDEX_MODE_8: u8 = 0x10;
    const MEMORY_MODE_8: u8 = 0x20;
    const OVERFLOW: u8 = 0x40;
    const NEGATIVE: u8 = 0x80;

    const RESET_VECTOR: u32 = 0xfffc;

    pub fn new(mem: Mem) -> Cpu {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            sp: 0x1ff,
            d: 0,
            pc: mem.read16(Self::RESET_VECTOR),
            k: 0,
            dbr: 0,
            ps: Self::IRQ_DISABLE,
            emulation_mode: true,
            mem,
            read_write_mask: 0xffffff,
            operand: 0,
        }
    }

    // flags
    #[inline]
    const fn carry(&self) -> bool {
        (self.ps & Self::CARRY) != 0
    }
    #[inline]
    const fn set_carry(&mut self, carry: bool) {
        if carry {
            self.ps |= Self::CARRY
        } else {
            self.ps &= !Self::CARRY
        }
    }
    #[inline]
    const fn zero(&self) -> bool {
        (self.ps & Self::ZERO) != 0
    }
    #[inline]
    const fn set_zero(&mut self, zero: bool) {
        if zero {
            self.ps |= Self::ZERO
        } else {
            self.ps &= !Self::ZERO
        }
    }
    #[inline]
    const fn irq_disable(&self) -> bool {
        (self.ps & Self::IRQ_DISABLE) != 0
    }
    #[inline]
    const fn set_irq_disable(&mut self, irq_disable: bool) {
        if irq_disable {
            self.ps |= Self::IRQ_DISABLE
        } else {
            self.ps &= !Self::IRQ_DISABLE
        }
    }
    #[inline]
    const fn decimal(&self) -> bool {
        (self.ps & Self::DECIMAL) != 0
    }
    #[inline]
    const fn set_decimal(&mut self, decimal: bool) {
        if decimal {
            self.ps |= Self::DECIMAL
        } else {
            self.ps &= !Self::DECIMAL
        }
    }
    #[inline]
    const fn index_mode_8(&self) -> bool {
        (self.ps & Self::INDEX_MODE_8) != 0
    }
    #[inline]
    const fn set_index_mode_8(&mut self, index_mode_8: bool) {
        if index_mode_8 {
            self.ps |= Self::INDEX_MODE_8
        } else {
            self.ps &= !Self::INDEX_MODE_8
        }
    }
    #[inline]
    const fn memory_mode_8(&self) -> bool {
        (self.ps & Self::MEMORY_MODE_8) != 0
    }
    #[inline]
    const fn set_memory_mode_8(&mut self, memory_mode_8: bool) {
        if memory_mode_8 {
            self.ps |= Self::MEMORY_MODE_8
        } else {
            self.ps &= !Self::MEMORY_MODE_8
        }
    }
    #[inline]
    const fn overflow(&self) -> bool {
        (self.ps & Self::OVERFLOW) != 0
    }
    #[inline]
    const fn set_overflow(&mut self, overflow: bool) {
        if overflow {
            self.ps |= Self::OVERFLOW
        } else {
            self.ps &= !Self::OVERFLOW
        }
    }
    #[inline]
    const fn negative(&self) -> bool {
        (self.ps & Self::NEGATIVE) != 0
    }
    #[inline]
    const fn set_negative(&mut self, negative: bool) {
        if negative {
            self.ps |= Self::NEGATIVE
        } else {
            self.ps &= !Self::NEGATIVE
        }
    }

    #[inline]
    const fn set_zero_negative8(&mut self, value: u8) {
        self.set_zero(value == 0);
        self.set_negative((value as i8) < 0);
    }

    #[inline]
    const fn set_zero_negative16(&mut self, value: u16) {
        self.set_zero(value == 0);
        self.set_negative((value as i16) < 0);
    }

    // instructions
    const fn adc8(&mut self, rhs: u8) {
        let mut result;
        if self.decimal() {
            result = (self.a & 0x0f) + (rhs as u16 & 0x0f) + self.carry() as u16;

            if result > 0x09 { result += 0x06 }
            result = (self.a & 0xf0) + (rhs as u16 & 0xf0) + if result > 0x0f { 0x10 } else { 0 } + (result & 0x0f);
        } else {
            result = self.a + rhs as u16 + self.carry() as u16;
        }

        self.set_overflow((!(self.a as u8 ^ rhs) & (self.a as u8 ^ result as u8) & 0x80) != 0);

        if self.decimal() && result > 0x9f {
            result += 0x60
        }

        self.set_zero_negative8(result as u8);
        self.set_carry(result > 0xff);

        self.a = (self.a & 0xff00) | (result & 0xff)
    }
    const fn adc16(&mut self, rhs: u16) {
        let mut result;
        if self.decimal() {
            result = (self.a as u32 & 0x0f) + (rhs as u32 & 0x0f) + self.carry() as u32;

            if result > 0x09 { result += 0x06 }
            result = (self.a as u32 & 0xf0) + (rhs as u32 & 0xf0) + if result > 0x0f { 0x10 } else { 0 } + (result as u32 & 0x0f);

            if result > 0x9f { result += 0x60 }
            result = (self.a as u32 & 0xf00) + (rhs as u32 & 0xf00) + if result > 0xff { 0x100 } else { 0 } + (result as u32 & 0xff);

            if result > 0x9ff { result += 0x600 }
            result = (self.a as u32 & 0xf000) + (rhs as u32 & 0xf000) + if result > 0xfff { 0x1000 } else { 0 } + (result as u32 & 0xfff);
        } else {
            result = self.a as u32 + rhs as u32 + self.carry() as u32;
        }

        self.set_overflow((!(self.a ^ rhs) & (self.a ^ result as u16) & 0x8000) != 0);

        if self.decimal() && result > 0x9fff {
            result += 0x6000
        }

        self.set_zero_negative16(result as u16);
        self.set_carry(result > 0xffff);

        self.a = result as u16
    }
    const fn adc(&mut self, rhs: u16) {
        if self.memory_mode_8() {
            self.adc8(rhs as u8)
        } else {
            self.adc16(rhs as u16)
        }
    }

    const fn sbc8(&mut self, rhs: u8) {
        let rhs = !rhs;
        let mut result;
        if self.decimal() {
            result = (self.a & 0x0f) + (rhs as u16 & 0x0f) + self.carry() as u16;

            if result <= 0x0f { result -= 0x06 }
            result = (self.a & 0xf0) + (rhs as u16 & 0xf0) + if result > 0x0f { 0x10 } else { 0 } + (result & 0x0f);
        } else {
            result = self.a + rhs as u16 + self.carry() as u16;
        }

        self.set_overflow((!(self.a as u8 ^ rhs) & (self.a as u8 ^ result as u8) & 0x80) != 0);

        if self.decimal() && result <= 0xff {
            result -= 0x60
        }

        self.set_zero_negative8(result as u8);
        self.set_carry(result > 0xff);

        self.a = (self.a & 0xff00) | (result & 0xff)
    }
    const fn sbc16(&mut self, rhs: u16) {
        let rhs = !rhs;
        let mut result;
        if self.decimal() {
            result = (self.a as u32 & 0x0f) + (rhs as u32 & 0x0f) + self.carry() as u32;

            if result <= 0x0f { result -= 0x06 }
            result = (self.a as u32 & 0xf0) + (rhs as u32 & 0xf0) + if result > 0x0f { 0x10 } else { 0 } + (result as u32 & 0x0f);

            if result <= 0xff { result -= 0x60 }
            result = (self.a as u32 & 0xf00) + (rhs as u32 & 0xf00) + if result > 0xff { 0x100 } else { 0 } + (result as u32 & 0xff);

            if result <= 0xfff { result -= 0x600 }
            result = (self.a as u32 & 0xf000) + (rhs as u32 & 0xf000) + if result > 0xfff { 0x1000 } else { 0 } + (result as u32 & 0xfff);
        } else {
            result = self.a as u32 + rhs as u32 + self.carry() as u32;
        }

        self.set_overflow((!(self.a ^ rhs) & (self.a ^ result as u16) & 0x8000) != 0);

        if self.decimal() && result <= 0xffff {
            result -= 0x6000
        }

        self.set_zero_negative16(result as u16);
        self.set_carry(result > 0xffff);

        self.a = result as u16
    }
    const fn sbc(&mut self, rhs: u16) {
        if self.memory_mode_8() {
            self.sbc8(rhs as u8)
        } else {
            self.sbc16(rhs)
        }
    }

    // branch operations
    const fn bcc(&mut self, rhs: u8) {
        if !self.carry() { self.bra(rhs) }
    }
    const fn bcs(&mut self, rhs: u8) {
        if self.carry() { self.bra(rhs) }
    }
    const fn beq(&mut self, rhs: u8) {
        if self.zero() { self.bra(rhs) }
    }
    const fn bmi(&mut self, rhs: u8) {
        if self.negative() { self.bra(rhs) }
    }
    const fn bne(&mut self, rhs: u8) {
        if !self.zero() { self.bra(rhs) }
    }
    const fn bpl(&mut self, rhs: u8) {
        if !self.negative() { self.bra(rhs) }
    }
    const fn bra(&mut self, rhs: u8) {
        self.pc = self.pc.wrapping_add(rhs as i8 as u16)
    }
    const fn brl(&mut self, rhs: u16) {
        self.pc = self.pc.wrapping_add(rhs)
    }
    const fn bvc(&mut self, rhs: u8) {
        if !self.overflow() { self.bra(rhs) }
    }
    const fn bvs(&mut self, rhs: u8) {
        if self.overflow() { self.bra(rhs) }
    }

    // set/clear flag instructions
    const fn clc(&mut self) {
        self.set_carry(false)
    }
    const fn cld(&mut self) {
        self.set_decimal(false)
    }
    const fn cli(&mut self) {
        self.set_irq_disable(false)
    }
    const fn clv(&mut self) {
        self.set_overflow(false)
    }
    const fn sec(&mut self) {
        self.set_carry(true)
    }
    const fn sed(&mut self) {
        self.set_decimal(true)
    }
    const fn sei(&mut self) {
        self.set_irq_disable(true)
    }
    const fn rep(&mut self, rhs: u8) {
        self.ps &= !rhs;
        if self.emulation_mode {
            self.set_memory_mode_8(true);
            self.set_index_mode_8(true);
        }
    }
    const fn sep(&mut self, rhs: u8) {
        self.ps |= rhs;
        if self.index_mode_8() {
            self.x &= 0xff;
            self.y &= 0xff;
        }
    }

    // increment/decrement operations
    const fn dec8(&mut self, rhs: u8) -> u8 {
        let result = rhs.wrapping_sub(1);
        self.set_zero_negative8(result);
        result
    }
    const fn dec16(&mut self, rhs: u16) -> u16 {
        let result = rhs.wrapping_sub(1);
        self.set_zero_negative16(result);
        result
    }
    const fn inc8(&mut self, rhs: u8) -> u8 {
        let result = rhs.wrapping_add(1);
        self.set_zero_negative8(result);
        result
    }
    const fn inc16(&mut self, rhs: u16) -> u16 {
        let result = rhs.wrapping_add(1);
        self.set_zero_negative16(result);
        result
    }
    const fn dex(&mut self) {
        if self.index_mode_8() {
            self.x = (self.x & 0xff00) | self.dec8(self.x as u8) as u16
        } else {
            self.x = self.dec16(self.x)
        }
    }
    const fn dey(&mut self) {
        if self.index_mode_8() {
            self.y = (self.y & 0xff00) | self.dec8(self.y as u8) as u16
        } else {
            self.y = self.dec16(self.y)
        }
    }
    const fn inx(&mut self) {
        if self.index_mode_8() {
            self.x = (self.x & 0xff00) | self.inc8(self.x as u8) as u16
        } else {
            self.x = self.inc16(self.x)
        }
    }
    const fn iny(&mut self) {
        if self.index_mode_8() {
            self.y = (self.y & 0xff00) | self.inc8(self.y as u8) as u16
        } else {
            self.y = self.inc16(self.y)
        }
    }
    const fn dec_a(&mut self) {
        if self.memory_mode_8() {
            self.a = (self.a & 0xff00) | self.dec8(self.a as u8) as u16
        } else {
            self.a = self.dec16(self.a)
        }
    }
    const fn inc_a(&mut self) {
        if self.memory_mode_8() {
            self.a = (self.a & 0xff00) | self.inc8(self.a as u8) as u16
        } else {
            self.a = self.inc16(self.a)
        }
    }

    const fn cmp(&mut self, rhs: u16) {
        if self.memory_mode_8() {
            self.set_carry(self.a as u8 >= rhs as u8);
            self.set_zero_negative8((self.a as u8).wrapping_sub(rhs as u8));
        } else {
            self.set_carry(self.a >= rhs);
            self.set_zero_negative16(self.a.wrapping_sub(rhs));
        }
    }
    const fn cpx(&mut self, rhs: u16) {
        if self.index_mode_8() {
            self.set_carry(self.x as u8 >= rhs as u8);
            self.set_zero_negative8((self.x as u8).wrapping_sub(rhs as u8));
        } else {
            self.set_carry(self.x >= rhs);
            self.set_zero_negative16(self.x.wrapping_sub(rhs));
        }
    }
    const fn cpy(&mut self, rhs: u16) {
        if self.index_mode_8() {
            self.set_carry(self.y as u8 >= rhs as u8);
            self.set_zero_negative8((self.y as u8).wrapping_sub(rhs as u8));
        } else {
            self.set_carry(self.y >= rhs);
            self.set_zero_negative16(self.y.wrapping_sub(rhs));
        }
    }
}

