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
        todo!()
    }

    pub fn ro_probe(&self) -> Result<(), FDTErr> {
        if assume(Assume::ValidDtb) {
            return Ok(());
        }

        // if ((uintptr_t)fdt & 7) return -FDT_ERR_ALIGNMENT;  // 当从 raw pointer -> ref 的时候, 就已经 check 了

        if self.version.to_le() == MAGIC {
            if !assume(Assume::Latest) {
                if self.version.to_le() < config::FIRST_SUPPORTED_VERSION {
                    return Err(Into::into(DTBErr::BadVersion));
                }
                if self.version.to_le() > config::LAST_SUPPORTED_VERSION {
                    return Err(Into::into(DTBErr::BadVersion));
                }
            }
        } else if self.version.to_le() == !MAGIC {
            // unfinished sequential-write blob
            if !assume(Assume::ValidInput) && self.size_dt_struct.to_le() == 0 {
                return Err(Into::into(ParamErr::BadState));
            }
        } else {
            return Err(Into::into(DTBErr::BadVersion));
        }

        if self.totalsize.to_le() >= (i32::MAX as u32) {
            return Err(Into::into(DTBErr::Truncated));
        }
        Ok(())
    }
}
