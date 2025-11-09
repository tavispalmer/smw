use crate::rom::Rom;

pub struct Mem {
    ram: Box<[u8; 0x20000]>,
    rom: Rom,
}

impl Mem {
    pub fn new(rom: Rom) -> Self {
        Self {
            ram: Box::new([0; _]),
            rom,
        }
    }

    pub fn read(&self, addr: u32) -> u8 {
        // chipset: rom + ram + battery
        // rom size: 1<<9 kb (512KB)
        // ram size: 1<<1 kb (2KB)
        // country: 0 (NTSC)
        // developer id: 1
        // rom version: 0
        // checksum complement: 0x737f
        // checksum: 0x8c80
        let bank = (addr >> 16) as u8;
        let addr = addr as u16;
        match (addr, bank) {
            // bus a
            (_,               0x7e..=0x7f)               => self.ram[(addr as usize) | (((bank as usize) & 1) << 16)],
            (0x4000..=0x4fff, 0x00..=0x3f | 0x80..=0xbf) => panic!(),
            (0x0000..=0x1fff, 0x00..=0x3f | 0x80..=0xbf) => self.ram[addr as usize],
            (0x0000..=0x7fff, 0x70..=0x7d | 0xf0..=0xff) => panic!("save ram"),
            (0x8000..=0xffff, 0x00..=0x7d | 0x80..=0xff) => self.rom[((addr as usize) & 0x7fff) | (((bank as usize) & 0xf) << 15)],
            // bus b
            (0x2000..=0x2fff, 0x00..=0x3f | 0x80..=0xbf) => panic!("ppu read ({addr:x})"),
            // open bus
            _ => panic!("open bus read @ 0x{bank:2x}{addr:4x}"),
        }
    }

    pub fn write(&mut self, addr: u32, value: u8) {
        let bank = (addr >> 16) as u8;
        let addr = addr as u16;
        match (addr, bank) {
            // bus a
            (_,               0x7e..=0x7f)               => self.ram[(addr as usize) | (((bank as usize) & 1) << 16)] = value,
            (0x4000..=0x4fff, 0x00..=0x3f | 0x80..=0xbf) => panic!(),
            (0x0000..=0x1fff, 0x00..=0x3f | 0x80..=0xbf) => self.ram[addr as usize] = value,
            (0x0000..=0x7fff, 0x70..=0x7d | 0xf0..=0xff) => panic!("save ram"),
            (0x8000..=0xffff, 0x00..=0x7d | 0x80..=0xff) => {}, // rom write
            // bus b
            (0x2000..=0x2fff, 0x00..=0x3f | 0x80..=0xbf) => panic!("ppu write ({addr:x})"),
            _ => panic!("unmapped write @ 0x{bank:2x}{addr:4x}"),
        }
    }

    pub fn read16(&self, addr: u32) -> u16 {
        let lsb = self.read(addr);
        let msb = self.read((addr + 1) & 0xffffff);
        ((msb as u16) << 8) | (lsb as u16)
    }
}
