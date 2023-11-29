use crate::{
    Elf64Word,
    Elf64Section,
    Elf64Addr,
    Elf64Xword,
};
/* Symbol table entry.  */

#[repr(C)]
#[derive(Debug)]
pub struct Elf64Sym
{
    pub st_name: Elf64Word,     /* Symbol name (string tbl index) */
    pub st_info: u8,            /* Symbol type and binding */
    pub st_other: u8,           /* Symbol visibility */
    pub st_shndx: Elf64Section, /* Section index */
    pub st_value: Elf64Addr,    /* Symbol value */
    pub st_size: Elf64Xword,    /* Symbol size */
}

/* How to extract and insert information held in the st_info field.  */

impl Elf64Sym {
    pub fn st_bind(&self) -> u8 {
        self.st_info >> 4
    }

    pub fn st_type(&self) -> u8 {
        self.st_info & 0xf
    }
}


/* Legal values for ST_BIND subfield of st_info (symbol binding).  */

pub const STB_LOCAL: u8 = 0;        /* Local symbol */
pub const STB_GLOBAL: u8 = 1;       /* Global symbol */
pub const STB_WEAK: u8 = 2;         /* Weak symbol */
pub const STB_NUM: u8 = 3;          /* Number of defined types.  */
pub const STB_LOOS: u8 = 10;        /* Start of OS-specific */
pub const STB_GNU_UNIQUE: u8 = 10;  /* Unique symbol.  */
pub const STB_HIOS: u8 = 12;        /* End of OS-specific */
pub const STB_LOPROC: u8 = 13;      /* Start of processor-specific */
pub const STB_HIPROC: u8 = 15;      /* End of processor-specific */

/* Legal values for ST_TYPE subfield of st_info (symbol type).  */

pub const STT_NOTYPE: u8 = 0;       /* Symbol type is unspecified */
pub const STT_OBJECT: u8 = 1;       /* Symbol is a data object */
pub const STT_FUNC: u8 = 2;         /* Symbol is a code object */
pub const STT_SECTION: u8 = 3;      /* Symbol associated with a section */
pub const STT_FILE: u8 = 4;         /* Symbol's name is file name */
pub const STT_COMMON: u8 = 5;       /* Symbol is a common data object */
pub const STT_TLS: u8 = 6;          /* Symbol is thread-local data object*/
pub const STT_NUM: u8 = 7;          /* Number of defined types.  */
pub const STT_LOOS: u8 = 10;        /* Start of OS-specific */
pub const STT_GNU_IFUNC: u8 = 10;   /* Symbol is indirect code object */
pub const STT_HIOS: u8 = 12;        /* End of OS-specific */
pub const STT_LOPROC: u8 = 13;      /* Start of processor-specific */
pub const STT_HIPROC: u8 = 15;      /* End of processor-specific */
