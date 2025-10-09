use crate::rom::Rom;

pub struct Mem {
    ram: Box<[u8; 0x2000]>,
    rom: Rom,
}

impl Mem {
    pub fn new(rom: Rom) -> Self {
        Self {
            ram: Box::new([0; _]),
            rom,
        }
    }

    pub fn read(&self, addr: u16, bank: u8) -> u8 {
        match (addr, bank) {
            (0x0000..=0x1fff, 0x00..=0x3f | 0x80..=0xbf) => self.ram[addr as usize],
            (0x8000..=0xffff, _) => self.rom[((addr as usize) & 0x7fff) | (((bank as usize) & 0xf) << 15)],
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, bank: u8, value: u8) {
        match (addr, bank) {
            (0x0000..=0x1fff, 0x00..=0x3f | 0x80..=0xbf) => self.ram[addr as usize] = value,
            (0x8000..=0xffff, _) => self.rom[((addr as usize) & 0x7fff) | (((bank as usize) & 0xf) << 15)] = value,
            _ => unreachable!(),
        }
    }
}
