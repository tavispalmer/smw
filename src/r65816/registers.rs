// code adapted from bsnes-mercury
// https://github.com/libretro/bsnes-mercury

use std::{
    mem,
    ops::{BitAnd, BitOr, BitOrAssign},
    slice,
};

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

    #[inline]
    pub const fn assign(&mut self, rhs: u8) {
        self.n = (rhs & 0x80) != 0;
        self.v = (rhs & 0x40) != 0;
        self.m = (rhs & 0x20) != 0;
        self.x = (rhs & 0x10) != 0;
        self.d = (rhs & 0x08) != 0;
        self.i = (rhs & 0x04) != 0;
        self.z = (rhs & 0x02) != 0;
        self.c = (rhs & 0x01) != 0;
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

impl BitOr<u8> for Flag {
    type Output = u8;

    #[inline]
    fn bitor(self, rhs: u8) -> u8 {
        BitOr::bitor(<Self as Into<u8>>::into(self), rhs)
    }
}

impl BitOrAssign<u8> for Flag {
    #[inline]
    fn bitor_assign(&mut self, rhs: u8) {
        self.assign(<Self as Into<u8>>::into(*self) | rhs)
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
    pub const fn w_mut(&mut self) -> &mut u16 {
        &mut self.0
    }
    #[inline]
    pub const fn l(&self) -> u8 {
        let data = unsafe { slice::from_raw_parts(&raw const self.0 as *const u8, 2) };
        if cfg!(target_endian = "little") {
            data[0]
        } else {
            data[1]
        }
    }
    #[inline]
    pub const fn l_mut(&mut self) -> &mut u8 {
        let data = unsafe { slice::from_raw_parts_mut(&raw mut self.0 as *mut u8, 2) };
        if cfg!(target_endian = "little") {
            &mut data[0]
        } else {
            &mut data[1]
        }
    }
    #[inline]
    pub const fn h(&self) -> u8 {
        let data = unsafe { slice::from_raw_parts(&raw const self.0 as *const u8, 2) };
        if cfg!(target_endian = "little") {
            data[1]
        } else {
            data[0]
        }
    }
    #[inline]
    pub const fn h_mut(&mut self) -> &mut u8 {
        let data = unsafe { slice::from_raw_parts_mut(&raw mut self.0 as *mut u8, 2) };
        if cfg!(target_endian = "little") {
            &mut data[1]
        } else {
            &mut data[0]
        }
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
    pub const fn d_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
    #[inline]
    pub const fn w(&self) -> u16 {
        let data = unsafe { slice::from_raw_parts(&raw const self.0 as *const u16, 2) };
        if cfg!(target_endian = "little") {
            data[0]
        } else {
            data[1]
        }
    }
    #[inline]
    pub const fn w_mut(&mut self) -> &mut u16 {
        let data = unsafe { slice::from_raw_parts_mut(&raw mut self.0 as *mut u16, 2) };
        if cfg!(target_endian = "little") {
            &mut data[0]
        } else {
            &mut data[1]
        }
    }
    #[inline]
    pub const fn wh(&self) -> u16 {
        let data = unsafe { slice::from_raw_parts(&raw const self.0 as *const u16, 2) };
        if cfg!(target_endian = "little") {
            data[1]
        } else {
            data[0]
        }
    }
    #[inline]
    pub const fn wh_mut(&mut self) -> &mut u16 {
        let data = unsafe { slice::from_raw_parts_mut(&raw mut self.0 as *mut u16, 2) };
        if cfg!(target_endian = "little") {
            &mut data[1]
        } else {
            &mut data[0]
        }
    }
    #[inline]
    pub const fn l(&self) -> u8 {
        let data = unsafe { slice::from_raw_parts(&raw const self.0 as *const u8, 4) };
        if cfg!(target_endian = "little") {
            data[0]
        } else {
            data[3]
        }
    }
    #[inline]
    pub const fn l_mut(&mut self) -> &mut u8 {
        let data = unsafe { slice::from_raw_parts_mut(&raw mut self.0 as *mut u8, 4) };
        if cfg!(target_endian = "little") {
            &mut data[0]
        } else {
            &mut data[3]
        }
    }
    #[inline]
    pub const fn h(&self) -> u8 {
        let data = unsafe { slice::from_raw_parts(&raw const self.0 as *const u8, 4) };
        if cfg!(target_endian = "little") {
            data[1]
        } else {
            data[2]
        }
    }
    #[inline]
    pub const fn h_mut(&mut self) -> &mut u8 {
        let data = unsafe { slice::from_raw_parts_mut(&raw mut self.0 as *mut u8, 4) };
        if cfg!(target_endian = "little") {
            &mut data[1]
        } else {
            &mut data[2]
        }
    }
    #[inline]
    pub const fn b(&self) -> u8 {
        let data = unsafe { slice::from_raw_parts(&raw const self.0 as *const u8, 4) };
        if cfg!(target_endian = "little") {
            data[2]
        } else {
            data[1]
        }
    }
    #[inline]
    pub const fn b_mut(&mut self) -> &mut u8 {
        let data = unsafe { slice::from_raw_parts_mut(&raw mut self.0 as *mut u8, 4) };
        if cfg!(target_endian = "little") {
            &mut data[2]
        } else {
            &mut data[1]
        }
    }
    #[inline]
    pub const fn bh(&self) -> u8 {
        let data = unsafe { slice::from_raw_parts(&raw const self.0 as *const u8, 4) };
        if cfg!(target_endian = "little") {
            data[3]
        } else {
            data[0]
        }
    }
    #[inline]
    pub const fn bh_mut(&mut self) -> &mut u8 {
        let data = unsafe { slice::from_raw_parts_mut(&raw mut self.0 as *mut u8, 4) };
        if cfg!(target_endian = "little") {
            &mut data[3]
        } else {
            &mut data[0]
        }
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

    #[inline]
    pub fn r(&self, index: usize) -> &Reg16 {
        match index {
            0 => &self.a,
            1 => &self.x,
            2 => &self.y,
            3 => &self.z,
            4 => &self.s,
            5 => &self.d,
            _ => panic!("index out of bounds"),
        }
    }
}
