//! Definitions of structs and enums from the device tree specification.
//! reference to https://github.com/riscv-software-src/opensbi/blob/master/lib/utils/libfdt/fdt.h

/// .
#[repr(C)]
pub struct FdtHeader {
    /// magic word MAGIC
    pub magic: fdt32,
    /// total size of DT block
    pub totalsize: fdt32,
    /// offset to structure
    pub off_dt_struct: fdt32,
    /// offset to strings
    pub off_dt_strings: fdt32,
    /// offset of memory reserved
    pub off_mem_rsvmap: fdt32,
    /// format version
    pub version: fdt32,
    /// last compatible version
    pub last_comp_version: fdt32,
    /// Which physical CPU id we're booting on (V2 fields below)
    pub boot_cpuid_phys: fdt32,
    /// size of the strings block (V3 fields below)
    pub size_dt_strings: fdt32,
    /// size of the structure block (V17 fields below)
    pub size_dt_struct: fdt32,
}

pub mod token {
    /// Start node: full name
    pub const FDT_BEGIN_NODE: u32 = 0x1;
    /// End node
    pub const FDT_END_NODE: u32 = 0x2;
    /// Property: name off, size, content
    pub const FDT_PROP: u32 = 0x3;
    /// nop
    pub const FDT_NOP: u32 = 0x4;
    /// end
    pub const FDT_END: u32 = 0x9;
}

#[repr(C)]
pub struct FdtReserveEntry {
    pub address: fdt64,
    pub size: fdt64,
}

#[repr(C)]
pub struct FdtNodeHeader<'a> {
    pub tag: fdt32,
    pub name: &'a [u8],
}

#[repr(C)]
pub struct FdtProperty<'a> {
    pub tag: fdt32,
    pub len: fdt32,
    pub nameoff: fdt32,
    pub data: &'a [u8],
}

/* ---------- ---------- limit ---------- ---------- */

pub const FDT_MAX_PHANDLE: u32 = 0xfffffffe;

/* ---------- ---------- version fdt size ---------- ---------- */

pub const FDT_V1_SIZE: usize = 7 * size_of::<fdt32>();
pub const FDT_V2_SIZE: usize = FDT_V1_SIZE + size_of::<fdt32>();
pub const FDT_V3_SIZE: usize = FDT_V2_SIZE + size_of::<fdt32>();
pub const FDT_V16_SIZE: usize = FDT_V3_SIZE;
pub const FDT_V17_SIZE: usize = FDT_V16_SIZE + size_of::<fdt32>();

/* ---------- ---------- magic number ---------- ---------- */

pub const MAGIC: u32 = 0xd00d_feed;

/* ---------- ---------- big endian ---------- ---------- */

/// A simple struct to represent a device tree.(big endian)
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fdt32(u32);

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fdt64(u64);

/* ---------- ---------- some structure ---------- ---------- */
