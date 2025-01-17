#![no_std]
#![allow(non_camel_case_types)]

#[macro_use] extern crate bitflags;
extern crate num_traits;

pub mod elf;
pub mod elf_header;
pub mod program_header;
pub mod section_header;

pub use elf::ElfGen;
pub use elf_header::{ElfHeaderGen};

pub use elf_header::{ElfAbi, ElfClass, ElfEndian, ElfMachine, ElfType};
pub use program_header::ProgramType;
pub use section_header::{SectionHeader, SectionHeaderFlags, SectionType};

type Elf32<'a> = elf::ElfGen<'a, u32>;
type Elf64<'a> = elf::ElfGen<'a, u64>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    BufferTooShort,
    InvalidMagic,
    InvalidClass,
}

#[derive(Debug)]
pub enum Elf<'a> {
    Elf32(Elf32<'a>),
    Elf64(Elf64<'a>)
}

impl<'a> Elf<'a> {
    pub fn from_bytes(elf_buf: &'a [u8]) -> Result<Self, Error> {
        use core::mem::size_of;

        if elf_buf.len() < size_of::<ElfHeaderGen::<u32>>() {
            return Err(Error::InvalidClass);
        }

        if !elf_buf.starts_with(&elf_header::ELF_MAGIC) {
            return Err(Error::InvalidMagic);
        }

        let tmp_elf = ElfGen::<u32>::new(elf_buf);
        match tmp_elf.header().class() {
            ElfClass::Elf64 => { 
                if elf_buf.len() < size_of::<ElfHeaderGen<u64>>() {
                    return Err(Error::InvalidMagic);
                }
                let elf = Elf64::new(elf_buf);
                if elf_buf.len() < elf.header().elf_header_size() as usize {
                    Err(Error::InvalidMagic)
                } else {
                    Ok(Elf::Elf64(elf))
                }
            }
            ElfClass::Elf32 => { 
                let elf = Elf32::new(elf_buf);
                if elf_buf.len() < elf.header().elf_header_size() as usize {
                    Err(Error::InvalidMagic)
                } else {
                    Ok(Elf::Elf32(elf))
                }
            }
            ElfClass::Unknown(_) => { Err(Error::InvalidClass) }
        }
    }
}
