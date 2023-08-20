use super::{
    nes_file::{NesFileHeader, INES_MAGIC_NUMBER},
    ppu::VerticalMirroring,
};

pub struct Cartridge {
    pub mapper_number: u8,
    pub vertical_mirroring: VerticalMirroring,
    pub prg_rom: Vec<u8>,
    pub prg_size: u32,
    pub prg_page_kbyte_units: u8,
    pub chr_rom: Vec<u8>,
    pub chr_size: u32,
    pub chr_page_kbyte_units: u8,
    pub prg_ram: Vec<u8>,
    pub prg_map: [u32; 4],
    pub chr_map: [u32; 8],
}

impl Cartridge {
    pub fn new(file: Vec<u8>) -> Self {
        let first_4byte = file[0..4]
            .try_into()
            .expect("Could not read first 4 bytes from file.");
        match first_4byte {
            INES_MAGIC_NUMBER => NesFileHeader::new_cartridge(file),
            _ => panic!("invalid file header"),
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn read_prg(&self, addr: u16) -> u8 {
        if addr < 0x8000 {
            return 0;
        }
        self.prg_rom[self.prg_map[(addr as usize - 0x8000) / 0x2000] as usize
            + (addr as usize - 0x8000) % 0x2000]
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn read_chr(&self, addr: u16) -> u8 {
        self.chr_rom[self.chr_map[addr as usize / 0x400] as usize + (addr as usize) % 0x400]
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn write_prg(&self, _: u16, value: u8) -> u8 {
        value
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn write_chr(&self, _: u16, value: u8) -> u8 {
        value
    }

    pub fn init_prg_map(&mut self) {
        for i in 0..(self.prg_page_kbyte_units as usize / 8) {
            self.prg_map[i] = (0x2000 * i as u32) % self.prg_size;
        }
    }

    pub fn init_chr_map(&mut self) {
        for i in 0..self.chr_page_kbyte_units as usize {
            self.chr_map[i] = (0x400 * i as u32) % self.chr_size;
        }
    }
}
