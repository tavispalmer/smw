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

    pub fn run(&mut self) {
        loop {
            self.exec()
        }
    }

    fn exec(&mut self) {
        match self.mem.read(((self.k as u32) << 16) | self.pc as u32) {
            0x00 => panic!("unknown opcode 0x00"),
            0x01 => panic!("unknown opcode 0x01"),
            0x02 => panic!("unknown opcode 0x02"),
            0x03 => panic!("unknown opcode 0x03"),
            0x04 => panic!("unknown opcode 0x04"),
            0x05 => panic!("unknown opcode 0x05"),
            0x06 => panic!("unknown opcode 0x06"),
            0x07 => panic!("unknown opcode 0x07"),
            0x08 => panic!("unknown opcode 0x08"),
            0x09 => panic!("unknown opcode 0x09"),
            0x0a => panic!("unknown opcode 0x0a"),
            0x0b => panic!("unknown opcode 0x0b"),
            0x0c => panic!("unknown opcode 0x0c"),
            0x0d => panic!("unknown opcode 0x0d"),
            0x0e => panic!("unknown opcode 0x0e"),
            0x0f => panic!("unknown opcode 0x0f"),
            0x10 => panic!("unknown opcode 0x10"),
            0x11 => panic!("unknown opcode 0x11"),
            0x12 => panic!("unknown opcode 0x12"),
            0x13 => panic!("unknown opcode 0x13"),
            0x14 => panic!("unknown opcode 0x14"),
            0x15 => panic!("unknown opcode 0x15"),
            0x16 => panic!("unknown opcode 0x16"),
            0x17 => panic!("unknown opcode 0x17"),
            0x18 => panic!("unknown opcode 0x18"),
            0x19 => panic!("unknown opcode 0x19"),
            0x1a => panic!("unknown opcode 0x1a"),
            0x1b => panic!("unknown opcode 0x1b"),
            0x1c => panic!("unknown opcode 0x1c"),
            0x1d => panic!("unknown opcode 0x1d"),
            0x1e => panic!("unknown opcode 0x1e"),
            0x1f => panic!("unknown opcode 0x1f"),
            0x20 => panic!("unknown opcode 0x20"),
            0x21 => panic!("unknown opcode 0x21"),
            0x22 => panic!("unknown opcode 0x22"),
            0x23 => panic!("unknown opcode 0x23"),
            0x24 => panic!("unknown opcode 0x24"),
            0x25 => panic!("unknown opcode 0x25"),
            0x26 => panic!("unknown opcode 0x26"),
            0x27 => panic!("unknown opcode 0x27"),
            0x28 => panic!("unknown opcode 0x28"),
            0x29 => panic!("unknown opcode 0x29"),
            0x2a => panic!("unknown opcode 0x2a"),
            0x2b => panic!("unknown opcode 0x2b"),
            0x2c => panic!("unknown opcode 0x2c"),
            0x2d => panic!("unknown opcode 0x2d"),
            0x2e => panic!("unknown opcode 0x2e"),
            0x2f => panic!("unknown opcode 0x2f"),
            0x30 => panic!("unknown opcode 0x30"),
            0x31 => panic!("unknown opcode 0x31"),
            0x32 => panic!("unknown opcode 0x32"),
            0x33 => panic!("unknown opcode 0x33"),
            0x34 => panic!("unknown opcode 0x34"),
            0x35 => panic!("unknown opcode 0x35"),
            0x36 => panic!("unknown opcode 0x36"),
            0x37 => panic!("unknown opcode 0x37"),
            0x38 => panic!("unknown opcode 0x38"),
            0x39 => panic!("unknown opcode 0x39"),
            0x3a => panic!("unknown opcode 0x3a"),
            0x3b => panic!("unknown opcode 0x3b"),
            0x3c => panic!("unknown opcode 0x3c"),
            0x3d => panic!("unknown opcode 0x3d"),
            0x3e => panic!("unknown opcode 0x3e"),
            0x3f => panic!("unknown opcode 0x3f"),
            0x40 => panic!("unknown opcode 0x40"),
            0x41 => panic!("unknown opcode 0x41"),
            0x42 => panic!("unknown opcode 0x42"),
            0x43 => panic!("unknown opcode 0x43"),
            0x44 => panic!("unknown opcode 0x44"),
            0x45 => panic!("unknown opcode 0x45"),
            0x46 => panic!("unknown opcode 0x46"),
            0x47 => panic!("unknown opcode 0x47"),
            0x48 => panic!("unknown opcode 0x48"),
            0x49 => panic!("unknown opcode 0x49"),
            0x4a => panic!("unknown opcode 0x4a"),
            0x4b => panic!("unknown opcode 0x4b"),
            0x4c => panic!("unknown opcode 0x4c"),
            0x4d => panic!("unknown opcode 0x4d"),
            0x4e => panic!("unknown opcode 0x4e"),
            0x4f => panic!("unknown opcode 0x4f"),
            0x50 => panic!("unknown opcode 0x50"),
            0x51 => panic!("unknown opcode 0x51"),
            0x52 => panic!("unknown opcode 0x52"),
            0x53 => panic!("unknown opcode 0x53"),
            0x54 => panic!("unknown opcode 0x54"),
            0x55 => panic!("unknown opcode 0x55"),
            0x56 => panic!("unknown opcode 0x56"),
            0x57 => panic!("unknown opcode 0x57"),
            0x58 => panic!("unknown opcode 0x58"),
            0x59 => panic!("unknown opcode 0x59"),
            0x5a => panic!("unknown opcode 0x5a"),
            0x5b => panic!("unknown opcode 0x5b"),
            0x5c => panic!("unknown opcode 0x5c"),
            0x5d => panic!("unknown opcode 0x5d"),
            0x5e => panic!("unknown opcode 0x5e"),
            0x5f => panic!("unknown opcode 0x5f"),
            0x60 => panic!("unknown opcode 0x60"),
            0x61 => panic!("unknown opcode 0x61"),
            0x62 => panic!("unknown opcode 0x62"),
            0x63 => panic!("unknown opcode 0x63"),
            0x64 => panic!("unknown opcode 0x64"),
            0x65 => panic!("unknown opcode 0x65"),
            0x66 => panic!("unknown opcode 0x66"),
            0x67 => panic!("unknown opcode 0x67"),
            0x68 => panic!("unknown opcode 0x68"),
            0x69 => panic!("unknown opcode 0x69"),
            0x6a => panic!("unknown opcode 0x6a"),
            0x6b => panic!("unknown opcode 0x6b"),
            0x6c => panic!("unknown opcode 0x6c"),
            0x6d => panic!("unknown opcode 0x6d"),
            0x6e => panic!("unknown opcode 0x6e"),
            0x6f => panic!("unknown opcode 0x6f"),
            0x70 => panic!("unknown opcode 0x70"),
            0x71 => panic!("unknown opcode 0x71"),
            0x72 => panic!("unknown opcode 0x72"),
            0x73 => panic!("unknown opcode 0x73"),
            0x74 => panic!("unknown opcode 0x74"),
            0x75 => panic!("unknown opcode 0x75"),
            0x76 => panic!("unknown opcode 0x76"),
            0x77 => panic!("unknown opcode 0x77"),
            0x78 => panic!("unknown opcode 0x78"),
            0x79 => panic!("unknown opcode 0x79"),
            0x7a => panic!("unknown opcode 0x7a"),
            0x7b => panic!("unknown opcode 0x7b"),
            0x7c => panic!("unknown opcode 0x7c"),
            0x7d => panic!("unknown opcode 0x7d"),
            0x7e => panic!("unknown opcode 0x7e"),
            0x7f => panic!("unknown opcode 0x7f"),
            0x80 => panic!("unknown opcode 0x80"),
            0x81 => panic!("unknown opcode 0x81"),
            0x82 => panic!("unknown opcode 0x82"),
            0x83 => panic!("unknown opcode 0x83"),
            0x84 => panic!("unknown opcode 0x84"),
            0x85 => panic!("unknown opcode 0x85"),
            0x86 => panic!("unknown opcode 0x86"),
            0x87 => panic!("unknown opcode 0x87"),
            0x88 => panic!("unknown opcode 0x88"),
            0x89 => panic!("unknown opcode 0x89"),
            0x8a => panic!("unknown opcode 0x8a"),
            0x8b => panic!("unknown opcode 0x8b"),
            0x8c => panic!("unknown opcode 0x8c"),
            0x8d => panic!("unknown opcode 0x8d"),
            0x8e => panic!("unknown opcode 0x8e"),
            0x8f => panic!("unknown opcode 0x8f"),
            0x90 => panic!("unknown opcode 0x90"),
            0x91 => panic!("unknown opcode 0x91"),
            0x92 => panic!("unknown opcode 0x92"),
            0x93 => panic!("unknown opcode 0x93"),
            0x94 => panic!("unknown opcode 0x94"),
            0x95 => panic!("unknown opcode 0x95"),
            0x96 => panic!("unknown opcode 0x96"),
            0x97 => panic!("unknown opcode 0x97"),
            0x98 => panic!("unknown opcode 0x98"),
            0x99 => panic!("unknown opcode 0x99"),
            0x9a => panic!("unknown opcode 0x9a"),
            0x9b => panic!("unknown opcode 0x9b"),
            0x9c => panic!("unknown opcode 0x9c"),
            0x9d => panic!("unknown opcode 0x9d"),
            0x9e => panic!("unknown opcode 0x9e"),
            0x9f => panic!("unknown opcode 0x9f"),
            0xa0 => panic!("unknown opcode 0xa0"),
            0xa1 => panic!("unknown opcode 0xa1"),
            0xa2 => panic!("unknown opcode 0xa2"),
            0xa3 => panic!("unknown opcode 0xa3"),
            0xa4 => panic!("unknown opcode 0xa4"),
            0xa5 => panic!("unknown opcode 0xa5"),
            0xa6 => panic!("unknown opcode 0xa6"),
            0xa7 => panic!("unknown opcode 0xa7"),
            0xa8 => panic!("unknown opcode 0xa8"),
            0xa9 => panic!("unknown opcode 0xa9"),
            0xaa => panic!("unknown opcode 0xaa"),
            0xab => panic!("unknown opcode 0xab"),
            0xac => panic!("unknown opcode 0xac"),
            0xad => panic!("unknown opcode 0xad"),
            0xae => panic!("unknown opcode 0xae"),
            0xaf => panic!("unknown opcode 0xaf"),
            0xb0 => panic!("unknown opcode 0xb0"),
            0xb1 => panic!("unknown opcode 0xb1"),
            0xb2 => panic!("unknown opcode 0xb2"),
            0xb3 => panic!("unknown opcode 0xb3"),
            0xb4 => panic!("unknown opcode 0xb4"),
            0xb5 => panic!("unknown opcode 0xb5"),
            0xb6 => panic!("unknown opcode 0xb6"),
            0xb7 => panic!("unknown opcode 0xb7"),
            0xb8 => panic!("unknown opcode 0xb8"),
            0xb9 => panic!("unknown opcode 0xb9"),
            0xba => panic!("unknown opcode 0xba"),
            0xbb => panic!("unknown opcode 0xbb"),
            0xbc => panic!("unknown opcode 0xbc"),
            0xbd => panic!("unknown opcode 0xbd"),
            0xbe => panic!("unknown opcode 0xbe"),
            0xbf => panic!("unknown opcode 0xbf"),
            0xc0 => panic!("unknown opcode 0xc0"),
            0xc1 => panic!("unknown opcode 0xc1"),
            0xc2 => panic!("unknown opcode 0xc2"),
            0xc3 => panic!("unknown opcode 0xc3"),
            0xc4 => panic!("unknown opcode 0xc4"),
            0xc5 => panic!("unknown opcode 0xc5"),
            0xc6 => panic!("unknown opcode 0xc6"),
            0xc7 => panic!("unknown opcode 0xc7"),
            0xc8 => panic!("unknown opcode 0xc8"),
            0xc9 => panic!("unknown opcode 0xc9"),
            0xca => panic!("unknown opcode 0xca"),
            0xcb => panic!("unknown opcode 0xcb"),
            0xcc => panic!("unknown opcode 0xcc"),
            0xcd => panic!("unknown opcode 0xcd"),
            0xce => panic!("unknown opcode 0xce"),
            0xcf => panic!("unknown opcode 0xcf"),
            0xd0 => panic!("unknown opcode 0xd0"),
            0xd1 => panic!("unknown opcode 0xd1"),
            0xd2 => panic!("unknown opcode 0xd2"),
            0xd3 => panic!("unknown opcode 0xd3"),
            0xd4 => panic!("unknown opcode 0xd4"),
            0xd5 => panic!("unknown opcode 0xd5"),
            0xd6 => panic!("unknown opcode 0xd6"),
            0xd7 => panic!("unknown opcode 0xd7"),
            0xd8 => panic!("unknown opcode 0xd8"),
            0xd9 => panic!("unknown opcode 0xd9"),
            0xda => panic!("unknown opcode 0xda"),
            0xdb => panic!("unknown opcode 0xdb"),
            0xdc => panic!("unknown opcode 0xdc"),
            0xdd => panic!("unknown opcode 0xdd"),
            0xde => panic!("unknown opcode 0xde"),
            0xdf => panic!("unknown opcode 0xdf"),
            0xe0 => panic!("unknown opcode 0xe0"),
            0xe1 => panic!("unknown opcode 0xe1"),
            0xe2 => panic!("unknown opcode 0xe2"),
            0xe3 => panic!("unknown opcode 0xe3"),
            0xe4 => panic!("unknown opcode 0xe4"),
            0xe5 => panic!("unknown opcode 0xe5"),
            0xe6 => panic!("unknown opcode 0xe6"),
            0xe7 => panic!("unknown opcode 0xe7"),
            0xe8 => panic!("unknown opcode 0xe8"),
            0xe9 => panic!("unknown opcode 0xe9"),
            0xea => panic!("unknown opcode 0xea"),
            0xeb => panic!("unknown opcode 0xeb"),
            0xec => panic!("unknown opcode 0xec"),
            0xed => panic!("unknown opcode 0xed"),
            0xee => panic!("unknown opcode 0xee"),
            0xef => panic!("unknown opcode 0xef"),
            0xf0 => panic!("unknown opcode 0xf0"),
            0xf1 => panic!("unknown opcode 0xf1"),
            0xf2 => panic!("unknown opcode 0xf2"),
            0xf3 => panic!("unknown opcode 0xf3"),
            0xf4 => panic!("unknown opcode 0xf4"),
            0xf5 => panic!("unknown opcode 0xf5"),
            0xf6 => panic!("unknown opcode 0xf6"),
            0xf7 => panic!("unknown opcode 0xf7"),
            0xf8 => panic!("unknown opcode 0xf8"),
            0xf9 => panic!("unknown opcode 0xf9"),
            0xfa => panic!("unknown opcode 0xfa"),
            0xfb => panic!("unknown opcode 0xfb"),
            0xfc => panic!("unknown opcode 0xfc"),
            0xfd => panic!("unknown opcode 0xfd"),
            0xfe => panic!("unknown opcode 0xfe"),
            0xff => panic!("unknown opcode 0xff"),
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

    // compare instructions
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

    const fn jml(&mut self, rhs: u32) {
        self.k = (rhs >> 16) as u8;
        self.pc = rhs as u16;
    }
    const fn jmp(&mut self, rhs: u16) {
        self.pc = rhs
    }
}

