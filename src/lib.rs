#![no_std]

/// A simple struct to represent a device tree.(big endian)
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdt32(u32);

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

/// .
#[repr(C)]
pub struct FDTHdr {
    magic: fdt32,
    totalsize: fdt32,
    off_dt_struct: fdt32,
    off_dt_strings: fdt32,
    off_mem_rsvmap: fdt32, // offset of memory reserved
    version: fdt32,
    last_comp_version: fdt32,
    boot_cpuid_phys: fdt32,
    size_dt_strings: fdt32,
    size_dt_struct: fdt32,
}
