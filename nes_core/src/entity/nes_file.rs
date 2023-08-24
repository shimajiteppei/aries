use crate::util::{bit::PartialBit, vec::Slice};

use super::{cartridge::Cartridge, ppu::VerticalMirroring};

/// magic number os .nes file
/// "NES<EOF>"
pub const INES_MAGIC_NUMBER: [u8; 4] = [0x4eu8, 0x45u8, 0x53u8, 0x1au8];

#[derive(Debug)]
pub struct NesFileHeader {
    prg_rom_size_in_16kbyte_units: u8,
    chr_rom_size_in_8kbyte_units: u8,
    prm_ram_size_in_8kbyte_units: u8,
    flags6: Flags6,
    flags7: Flags7,
}

#[derive(Debug)]
struct Flags6 {
    vertical_mirroing: VerticalMirroring,
    has_trainer: bool,
    mapper_low_4bit: u8,
}

#[derive(Debug)]
struct Flags7 {
    mapper_high_4bit: u8,
}

impl NesFileHeader {
    fn read_header(file: &[u8]) -> Self {
        let file6 = file[6];
        let file7 = file[7];
        Self {
            prg_rom_size_in_16kbyte_units: file[4],
            chr_rom_size_in_8kbyte_units: file[5],
            prm_ram_size_in_8kbyte_units: file[8],
            flags6: Flags6 {
                vertical_mirroing: file6.bit_flag(0),
                has_trainer: file6.bit_flag(2),
                mapper_low_4bit: (file6 & 0b1111_0000) >> 4,
            },
            flags7: Flags7 {
                mapper_high_4bit: (file7 & 0b1111_0000) >> 4,
            },
        }
    }

    pub fn new_cartridge(file: Vec<u8>) -> Cartridge {
        let header = NesFileHeader::read_header(&file);
        let mapper_number = (header.flags7.mapper_high_4bit << 4) | header.flags6.mapper_low_4bit;

        // slice into each segments
        let head_end = 16;
        let trainer_end = head_end + if header.flags6.has_trainer { 512 } else { 0 };
        let prg_end = trainer_end + (header.prg_rom_size_in_16kbyte_units as usize) * 0x4000;
        let chr_end = prg_end + (header.chr_rom_size_in_8kbyte_units as usize) * 0x2000;

        let head = file.copy_slice(0..head_end);
        if head[0..4] != INES_MAGIC_NUMBER {
            panic!("invalid nes file magic number.")
        }
        let prg = file.copy_slice(trainer_end..prg_end);
        let chr = file.copy_slice(prg_end..chr_end);

        let mut cartridge = Cartridge {
            mapper_number,
            vertical_mirroring: header.flags6.vertical_mirroing,
            prg_rom: prg,
            prg_size: (header.prg_rom_size_in_16kbyte_units as u32) * 0x4000,
            prg_page_kbyte_units: 32,
            chr_rom: chr,
            chr_size: (header.chr_rom_size_in_8kbyte_units as u32) * 0x2000,
            chr_page_kbyte_units: 8,
            prg_ram: vec![0; header.prm_ram_size_in_8kbyte_units as usize * 0x2000],
            prg_map: [0; 4],
            chr_map: [0; 8],
        };

        cartridge.init_prg_map();
        cartridge.init_chr_map();

        cartridge
    }
}
