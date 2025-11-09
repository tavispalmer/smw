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
    const fn clear_carry(&mut self) {
        self.ps &= !Self::CARRY
    }
    #[inline]
    const fn set_carry(&mut self) {
        self.ps |= Self::CARRY
    }
    #[inline]
    const fn zero(&self) -> bool {
        (self.ps & Self::ZERO) != 0
    }
    #[inline]
    const fn clear_zero(&mut self) {
        self.ps &= !Self::ZERO
    }
    #[inline]
    const fn set_zero(&mut self) {
        self.ps |= Self::ZERO
    }
    #[inline]
    const fn irq_disable(&self) -> bool {
        (self.ps & Self::IRQ_DISABLE) != 0
    }
    #[inline]
    const fn clear_irq_disable(&mut self) {
        self.ps &= !Self::IRQ_DISABLE
    }
    #[inline]
    const fn set_irq_disable(&mut self) {
        self.ps |= Self::IRQ_DISABLE
    }
    #[inline]
    const fn decimal(&self) -> bool {
        (self.ps & Self::DECIMAL) != 0
    }
    #[inline]
    const fn clear_decimal(&mut self) {
        self.ps &= !Self::DECIMAL
    }
    #[inline]
    const fn set_decimal(&mut self) {
        self.ps |= Self::DECIMAL
    }
    #[inline]
    const fn index_mode_8(&self) -> bool {
        (self.ps & Self::INDEX_MODE_8) != 0
    }
    #[inline]
    const fn set_index_mode_16(&mut self) {
        self.ps &= !Self::INDEX_MODE_8
    }
    #[inline]
    const fn set_index_mode_8(&mut self) {
        self.ps |= Self::INDEX_MODE_8
    }
    #[inline]
    const fn memory_mode_8(&self) -> bool {
        (self.ps & Self::MEMORY_MODE_8) != 0
    }
    #[inline]
    const fn set_memory_mode_16(&mut self) {
        self.ps &= !Self::MEMORY_MODE_8
    }
    #[inline]
    const fn set_memory_mode_8(&mut self) {
        self.ps |= Self::MEMORY_MODE_8
    }
    #[inline]
    const fn overflow(&self) -> bool {
        (self.ps & Self::OVERFLOW) != 0
    }
    #[inline]
    const fn clear_overflow(&mut self) {
        self.ps &= !Self::OVERFLOW
    }
    #[inline]
    const fn set_overflow(&mut self) {
        self.ps |= Self::OVERFLOW
    }
    #[inline]
    const fn negative(&self) -> bool {
        (self.ps & Self::NEGATIVE) != 0
    }
    #[inline]
    const fn clear_negative(&mut self) {
        self.ps &= !Self::NEGATIVE
    }
    #[inline]
    const fn set_negative(&mut self) {
        self.ps |= Self::NEGATIVE
    }
}

