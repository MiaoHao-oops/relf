use crate::{
    Elf64Word,
    Elf64Off,
    Elf64Addr,
    Elf64Xword,
};

/* Program segment header.  */

#[repr(C)]
#[derive(Debug)]
pub struct Elf64Phdr
{
    pub p_type: Elf64Word,      /* Segment type */
    pub p_flags: Elf64Word,     /* Segment flags */
    pub p_offset: Elf64Off,     /* Segment file offset */
    pub p_vaddr: Elf64Addr,     /* Segment virtual address */
    pub p_paddr: Elf64Addr,     /* Segment physical address */
    pub p_filesz: Elf64Xword,   /* Segment size in file */
    pub p_memsz: Elf64Xword,    /* Segment size in memory */
    pub p_align: Elf64Xword,    /* Segment alignment */
}

/* Legal values for p_type (segment type).  */

pub const PT_LOAD: Elf64Word = 1;   /* Loadable program segment */

/* Legal values for p_flags (segment flags).  */

pub const PF_X: Elf64Word = 1 << 0; /* Segment is executable */
pub const PF_W: Elf64Word =	1 << 1; /* Segment is writable */
pub const PF_R: Elf64Word =	1 << 2; /* Segment is readable */
