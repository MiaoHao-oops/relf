#![no_std]
/* Type for a 16-bit quantity.  */
pub type Elf32Half = u16;
pub type Elf64Half = u16;

/* Types for signed and unsigned 32-bit quantities.  */
pub type Elf32Word = u32;
pub type Elf32Sword = i32;
pub type Elf64Word = u32;
pub type Elf64Sword = i32;

/* Types for signed and unsigned 64-bit quantities.  */
pub type Elf32Xword = u64;
pub type Elf32Sxword = i64;
pub type Elf64Xword = u64;
pub type Elf64Sxword = i64;

/* Type of addresses.  */
pub type Elf32Addr = u32;
pub type Elf64Addr = u64;

/* Type of file offsets.  */
pub type Elf32Off = u32;
pub type Elf64Off = u64;

/* Type for section indices, which are 16-bit quantities.  */
pub type Elf32Section = u16;
pub type Elf64Section = u16;

/* Type for version symbol information.  */
pub type Elf32Versym = Elf32Half;
pub type Elf64Versym = Elf64Half;

pub const EI_NIDENT: usize = 16;

/* The ELF file header.  This appears at the start of every ELF file.  */

#[repr(C)]
#[derive(Debug)]
pub struct Elf64Ehdr {
    pub e_ident: [u8; EI_NIDENT],   /* Magic number and other info */
    pub e_type: Elf64Half,          /* Object file type */
    pub e_machine: Elf64Half,       /* Architecture */
    pub e_version: Elf64Word,       /* Object file version */
    pub e_entry: Elf64Addr,         /* Entry point virtual address */
    pub e_phoff: Elf64Off,          /* Program header table file offset */
    pub e_shoff: Elf64Off,          /* Section header table file offset */
    pub e_flags: Elf64Word,         /* Processor-specific flags */
    pub e_ehsize: Elf64Half,        /* ELF header size in bytes */
    pub e_phentsize: Elf64Half,     /* Program header table entry size */
    pub e_phnum: Elf64Half,         /* Program header table entry count */
    pub e_shentsize: Elf64Half,     /* Section header table entry size */
    pub e_shnum: Elf64Half,         /* Section header table entry count */
    pub e_shstrndx: Elf64Half       /* Section header string table index */
}

/* Fields in the e_ident array.  The EI_* macros are indices into the
   array.  The macros under each EI_* macro are the values the byte
   may have. 
    */

pub const EI_MAG: [usize; 4] = [
    0,  /* File identification byte 0 index */
    1,  /* File identification byte 1 index */
    2,  /* File identification byte 2 index */
    3,  /* File identification byte 3 index */
];
pub const ELFMAG: [u8; 4] = [
    0x7f,   /* Magic number byte 0 */
    b'E',   /* Magic number byte 1 */
    b'L',   /* Magic number byte 2 */
    b'F',   /* Magic number byte 3 */
];

impl Elf64Ehdr {
    pub fn get_sht(&self, start: usize) -> &'static [Elf64Shdr] {
        unsafe {
            core::slice::from_raw_parts(
                (start + self.e_shoff as usize) as *const Elf64Shdr,
                self.e_shnum as usize
            )
        }
    }

    pub fn get_pht(&self, start: usize) -> &'static [Elf64Phdr] {
        unsafe {
            core::slice::from_raw_parts(
                (start + self.e_phoff as usize) as *const Elf64Phdr,
                self.e_phnum as usize
            )
        }
    }

    pub fn get_shstrtab_hdr(&self, start: usize) -> &'static Elf64Shdr {
        &self.get_sht(start)[self.e_shstrndx as usize]
    }

    pub fn get_she(&self, start: usize, name: &str) -> Option<&'static Elf64Shdr> {
        let shstrtab_hdr = self.get_shstrtab_hdr(start);
        for she in self.get_sht(start) {
            if shstrtab_hdr.get_name(start, she.sh_name) == name {
                return Some(she);
            }
        }
        None
    }
}

pub mod shdr;
pub mod rela;
pub mod sym;
pub mod phdr;

pub use shdr::Elf64Shdr as Elf64Shdr;
pub use rela::Elf64Rela as Elf64Rela;
pub use sym::Elf64Sym as Elf64Sym;
pub use phdr::Elf64Phdr as Elf64Phdr;
