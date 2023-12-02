use core::mem::size_of;
use crate::{
    Elf64Word,
    Elf64Xword,
    Elf64Addr,
    Elf64Off,
};

/* Section header.  */

#[repr(C)]
#[derive(Debug)]
pub struct Elf64Shdr {
    pub sh_name: Elf64Word,         /* Section name (string tbl index) */
    pub sh_type: Elf64Word,         /* Section type */
    pub sh_flages: Elf64Xword,      /* Section flags */
    pub sh_addr: Elf64Addr,         /* Section virtual addr at execution */
    pub sh_offset: Elf64Off,        /* Section file offset */
    pub sh_size: Elf64Xword,        /* Section size in bytes */
    pub sh_link: Elf64Word,         /* Link to another section */
    pub sh_info: Elf64Word,         /* Additional section information */
    pub sh_addralign: Elf64Xword,   /* Section alignment */
    pub sh_entsize: Elf64Xword,     /* Entry size if section holds table */
}

pub const SHT_NULL: Elf64Word = 0;                      /* Section header table entry unused */
pub const SHT_PROGBITS: Elf64Word = 1;                  /* Program data */
pub const SHT_SYMTAB: Elf64Word = 2;                    /* Symbol table */
pub const SHT_STRTAB: Elf64Word = 3;                    /* String table */
pub const SHT_RELA: Elf64Word = 4;                      /* Relocation entries with addends */
pub const SHT_HASH: Elf64Word = 5;                      /* Symbol hash table */
pub const SHT_DYNAMIC: Elf64Word = 6;                   /* Dynamic linking information */
pub const SHT_NOTE: Elf64Word = 7;                      /* Notes */
pub const SHT_NOBITS: Elf64Word = 8;                    /* Program space with no data (bss) */
pub const SHT_REL: Elf64Word = 9;                       /* Relocation entries, no addends */
pub const SHT_SHLIB: Elf64Word = 10;                    /* Reserved */
pub const SHT_DYNSYM: Elf64Word = 11;                   /* Dynamic linker symbol table */
pub const SHT_INIT_ARRAY: Elf64Word = 14;               /* Array of constructors */
pub const SHT_FINI_ARRAY: Elf64Word = 15;               /* Array of destructors */
pub const SHT_PREINIT_ARRAY: Elf64Word = 16;            /* Array of pre-constructors */
pub const SHT_GROUP: Elf64Word = 17;                    /* Section group */
pub const SHT_SYMTAB_SHNDX: Elf64Word = 18;             /* Extended section indices */
pub const SHT_NUM: Elf64Word = 19;                      /* Number of defined types.  */
pub const SHT_LOOS: Elf64Word = 0x60000000;             /* Start OS-specific.  */
pub const SHT_GNU_ATTRIBUTE: Elf64Word = 0x6ffffff5;    /* Object attributes.  */
pub const SHT_GNU_HASH: Elf64Word = 0x6ffffff6;         /* GNU-style hash table.  */
pub const SHT_GNU_LIBLIST: Elf64Word = 0x6ffffff7;      /* Prelink library list */
pub const SHT_CHECKSUM: Elf64Word = 0x6ffffff8;         /* Checksum for DSO content.  */
pub const SHT_LOSUNW: Elf64Word = 0x6ffffffa;           /* Sun-specific low bound.  */
pub const SHT_SUNW_MOVE: Elf64Word = 0x6ffffffa;
pub const SHT_SUNW_COMDAT: Elf64Word = 0x6ffffffb;
pub const SHT_SUNW_SYMINFO: Elf64Word = 0x6ffffffc;
pub const SHT_GNU_VERDEF: Elf64Word = 0x6ffffffd;       /* Version definition section.  */
pub const SHT_GNU_VERNEED: Elf64Word = 0x6ffffffe;      /* Version needs section.  */
pub const SHT_GNU_VERSYM: Elf64Word = 0x6fffffff;       /* Version symbol table.  */
pub const SHT_HISUNW: Elf64Word = 0x6fffffff;           /* Sun-specific high bound.  */
pub const SHT_HIOS: Elf64Word = 0x6fffffff;             /* End OS-specific type */
pub const SHT_LOPROC: Elf64Word = 0x70000000;           /* Start of processor-specific */
pub const SHT_HIPROC: Elf64Word = 0x7fffffff;           /* End of processor-specific */
pub const SHT_LOUSER: Elf64Word = 0x80000000;           /* Start of application-specific */
pub const SHT_HIUSER: Elf64Word = 0x8fffffff;           /* End of application-specific */

impl Elf64Shdr {
    pub fn get_section_as_slice<T>(&self, start: usize) -> &'static [T] {
        unsafe{
            core::slice::from_raw_parts(
                (start + self.sh_offset as usize) as *const T,
                self.sh_size as usize / size_of::<T>()
            )
        }
    }

    pub fn get_name<'a>(&'a self, start: usize, name_idx: Elf64Word) -> &'a str {
        assert!(self.sh_type == SHT_STRTAB);
        let mut end = name_idx as usize;
        let section = self.get_section_as_slice::<u8>(start);
        while end < section.len() {
            if section[end] == 0 {
                break;
            }
            end += 1;
        }
        core::str::from_utf8(&section[name_idx as usize..end]).unwrap()
    }

    pub fn get_table<T>(&self, start: usize) -> &'static [T] {
        assert!(size_of::<T>() == self.sh_entsize as usize);
        unsafe {
            core::slice::from_raw_parts(
                (start + self.sh_offset as usize) as *const T,
                (self.sh_size / self.sh_entsize) as usize
            )
        }
    }
}
