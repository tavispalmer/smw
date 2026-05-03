// code adapted from bsnes-mercury
// https://github.com/libretro/bsnes-mercury

use std::ops::BitAnd;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Flag {
    pub n: bool,
    pub v: bool,
    pub m: bool,
    pub x: bool,
    pub d: bool,
    pub i: bool,
    pub z: bool,
    pub c: bool,
}

impl Flag {
    #[inline]
    pub const fn new() -> Self {
        Self {
            n: false,
            v: false,
            m: false,
            x: false,
            d: false,
            i: false,
            z: false,
            c: false,
        }
    }
}

impl From<Flag> for u8 {
    #[inline]
    fn from(value: Flag) -> Self {
        ((value.n as u8) << 7)
            + ((value.v as u8) << 6)
            + ((value.m as u8) << 5)
            + ((value.x as u8) << 4)
            + ((value.d as u8) << 3)
            + ((value.i as u8) << 2)
            + ((value.z as u8) << 1)
            + ((value.c as u8) << 0)
    }
}

impl BitAnd<u8> for Flag {
    type Output = u8;

    #[inline]
    fn bitand(self, rhs: u8) -> u8 {
        BitAnd::bitand(<Self as Into<u8>>::into(self), rhs)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Reg16(u16);

impl Reg16 {
    #[inline]
    pub const fn w(&self) -> u16 {
        self.0
    }
    #[inline]
    pub const fn set_w(&mut self, w: u16) {
        self.0 = w
    }
    #[inline]
    pub const fn l(&self) -> u8 {
        self.0 as u8
    }
    #[inline]
    pub const fn set_l(&mut self, l: u8) {
        self.0 = (self.0 & !0xff) | l as u16
    }
    #[inline]
    pub const fn h(&self) -> u8 {
        (self.0 >> 8) as u8
    }
    #[inline]
    pub const fn set_h(&mut self, h: u8) {
        self.0 = (self.0 & !0xff00) | (h as u16) << 8
    }
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Reg24(u32);

impl Reg24 {
    #[inline]
    pub const fn d(&self) -> u32 {
        self.0
    }
    #[inline]
    pub const fn set_d(&mut self, d: u32) {
        self.0 = d
    }
    #[inline]
    pub const fn w(&self) -> u16 {
        self.0 as u16
    }
    #[inline]
    pub const fn set_w(&mut self, w: u16) {
        self.0 = (self.0 & !0xffff) | w as u32
    }
    #[inline]
    pub const fn wh(&self) -> u16 {
        (self.0 >> 16) as u16
    }
    #[inline]
    pub const fn set_wh(&mut self, wh: u16) {
        self.0 = (self.0 & !0xffff0000) | (wh as u32) << 16
    }
    #[inline]
    pub const fn l(&self) -> u8 {
        self.0 as u8
    }
    #[inline]
    pub const fn set_l(&mut self, l: u8) {
        self.0 = (self.0 & !0xff) | l as u32
    }
    #[inline]
    pub const fn h(&self) -> u8 {
        (self.0 >> 8) as u8
    }
    #[inline]
    pub const fn set_h(&mut self, h: u8) {
        self.0 = (self.0 & !0xff00) | (h as u32) << 8
    }
    #[inline]
    pub const fn b(&self) -> u8 {
        (self.0 >> 16) as u8
    }
    #[inline]
    pub const fn set_b(&mut self, b: u8) {
        self.0 = (self.0 & !0xff0000) | (b as u32) << 16
    }
    #[inline]
    pub const fn bh(&self) -> u8 {
        (self.0 >> 24) as u8
    }
    #[inline]
    pub const fn set_bh(&mut self, bh: u8) {
        self.0 = (self.0 & !0xff000000) | (bh as u32) << 24
    }
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }
}

pub struct Regs {
    pub pc: Reg24,
    pub a: Reg16,
    pub x: Reg16,
    pub y: Reg16,
    pub z: Reg16,
    pub s: Reg16,
    pub d: Reg16,
    pub p: Flag,
    pub db: u8,
    pub e: bool,

    pub irq: bool,
    pub wai: bool,
    pub mdr: u8,
    pub vector: u16,
}

impl Regs {
    #[inline]
    pub fn new() -> Self {
        Self {
            pc: Reg24::new(),
            a: Reg16::new(),
            x: Reg16::new(),
            y: Reg16::new(),
            z: Reg16::new(),
            s: Reg16::new(),
            d: Reg16::new(),
            p: Flag::new(),
            db: 0,
            e: false,
            irq: false,
            wai: false,
            mdr: 0,
            vector: 0,
        }
    }
}
