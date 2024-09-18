//! This file defined some checks the sanity of fdt

use super::*;

impl FdtHeader {
    /// max of totalsize is u32::MAX, so the type of size_mem_rsvmap should be u32
    pub fn check_ordered(&self, size_mem_rsvmap: u32) -> Result<(), DTBErr> {
        // end(last section) + 1 <= begin(cur section) -> sanity
        if align8(size_of::<Self>()) > (self.off_mem_rsvmap.to_le() as usize) {
            return Err(DTBErr::BadLayout);
        }
        if self.off_mem_rsvmap.to_le() + size_mem_rsvmap > self.off_dt_struct.to_le() {
            return Err(DTBErr::BadLayout);
        }
        if self.off_dt_struct.to_le() + self.size_dt_struct.to_le() > self.off_dt_strings.to_le() {
            return Err(DTBErr::BadLayout);
        }
        if self.off_dt_strings.to_le() + self.size_dt_strings.to_le() > self.totalsize.to_le() {
            return Err(DTBErr::BadLayout);
        }
        Ok(())
    }

    /// check if writable
    pub fn rw_probe(&mut self) -> Result<(), DTBErr> {
        if assume(Assume::ValidDtb) {
            return Ok(());
        }
        Ok(())
    }
}
