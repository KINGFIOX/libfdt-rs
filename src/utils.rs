pub fn align8(val: u32) -> u32 {
    (val + 7) & !7
}

/* ---------- ---------- big endian ---------- ---------- */

/// A simple struct to represent a device tree.(big endian)
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fdt32(u32);

impl fdt32 {
    pub fn to_le(&self) -> u32 {
        self.0.swap_bytes()
    }

    pub fn to_be(val: u32) -> Self {
        fdt32(val.swap_bytes())
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fdt64(u64);
