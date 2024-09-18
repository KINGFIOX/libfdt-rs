//! Definitions of structs and enums from the device tree specification

/// A simple struct to represent a device tree.(big endian)
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fdt32(u32);

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fdt64(u64);

impl From<fdt32> for u32 {
    fn from(value: fdt32) -> Self {
        value.0.swap_bytes()
    }
}

impl From<u32> for fdt32 {
    fn from(value: u32) -> Self {
        fdt32(value.swap_bytes())
    }
}

pub const MAGIC: u32 = 0xd00d_feed;

/// .
#[repr(C)]
pub struct FDTHdr {
    pub magic: fdt32,
    pub totalsize: fdt32,
    pub off_dt_struct: fdt32,
    pub off_dt_strings: fdt32,
    pub off_mem_rsvmap: fdt32, // offset of memory reserved
    pub version: fdt32,
    pub last_comp_version: fdt32,
    pub boot_cpuid_phys: fdt32,
    pub size_dt_strings: fdt32,
    pub size_dt_struct: fdt32,
}

/// flattened device tree property header
#[repr(C)]
pub struct FDTPropHdr {
    // length of the property data
    len: fdt32,
    /// offset of the property name string within the dt_string section
    nameoff: fdt32,
}

#[repr(C)]
pub struct FDTReserveEntry {
    pub address: fdt64,
    pub size: fdt64,
}
