// tests/integration_test.rs

use libfdt_rs::*;

#[test]
fn test_fdt32_from_u32() {
    println!("Size of FDTHdr: {}", size_of::<FDTHdr>());
    println!("Alignment of FDTHdr: {}", align_of::<FDTHdr>());
    println!("Size of fdt32: {}", size_of::<fdt32>());
}
