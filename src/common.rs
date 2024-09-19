use super::*;

impl FdtHeader {
    pub fn hdr_size_(&self) -> u32 {
        if assume_(Assume::Latest) {
            return FDT_V17_SIZE as u32;
        }
        let version = self.version.to_le();
        if version <= 1 {
            FDT_V1_SIZE as u32
        } else if version <= 2 {
            FDT_V2_SIZE as u32
        } else if version <= 3 {
            FDT_V3_SIZE as u32
        } else if version <= 16 {
            FDT_V16_SIZE as u32
        } else {
            FDT_V17_SIZE as u32
        }
    }
}
