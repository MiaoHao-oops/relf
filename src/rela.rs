use crate::{
    Elf64Addr,
    Elf64Xword,
    Elf64Sxword,
};

/* Relocation table entry with addend (in section of type SHT_RELA).  */
#[repr(C)]
#[derive(Debug)]
pub struct Elf64Rela {
    pub r_offset: Elf64Addr,        /* Address */
    pub r_info: Elf64Xword,         /* Relocation type and symbol index */
    pub r_addend: Elf64Sxword,      /* Addend */
}

/* How to extract and insert information held in the r_info field.  */

impl Elf64Rela {
    pub fn r_sym(&self) -> usize {
        (self.r_info >> 32) as usize
    }

    pub fn r_type(&self) -> usize {
        (self.r_info & 0xffff_ffff) as usize
    }
}
