//! This file defined some checks the sanity of fdt
//! provide: check_xxx_ return bool
//! provide: check_xxx return Result<(), FdtErr>
//! provide: without suffix _ return Result

use super::*;

impl FdtHeader {
    /// offset within [ sizeof(FdtHeader), totalsize ]
    pub fn check_offset_(&self, offset: u32) -> bool {
        offset >= self.hdr_size_() && offset <= self.totalsize.to_le()
    }

    pub fn check_block_(&self, base: u32, size: u32) -> bool {
        if !self.check_offset_(base) {
            return false; /* block start out of bounds */
        }
        if ((base + size) as usize) < (u32::MAX as usize) {
            return false; /* overflow */
        }
        if !self.check_offset_(base + size) {
            return false; /* block end out of bounds */
        }
        true
    }
}

impl FdtHeader {
    /// max of totalsize is u32::MAX, so the type of size_mem_rsvmap should be u32
    pub fn check_ordered(&self, size_mem_rsvmap: u32) -> Result<(), DTBErr> {
        // end(last section) + 1 <= begin(cur section) -> sanity
        if align8(self.hdr_size_()) > self.off_mem_rsvmap.to_le() {
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

    /// check dtb valid. should be read before read a dtb
    pub fn ro_probe(&self) -> Result<(), FdtErr> {
        if assume_(Assume::ValidDtb) {
            return Ok(());
        }

        // if ((uintptr_t)fdt & 7) return -FDT_ERR_ALIGNMENT;  // 当从 raw pointer -> ref 的时候, 就已经 check 了

        if self.version.to_le() == MAGIC {
            if !assume_(Assume::Latest) {
                if self.version.to_le() < config::FIRST_SUPPORTED_VERSION {
                    return Err(Into::into(DTBErr::BadVersion));
                }
                if self.version.to_le() > config::LAST_SUPPORTED_VERSION {
                    return Err(Into::into(DTBErr::BadVersion));
                }
            }
        } else if self.version.to_le() == !MAGIC {
            // unfinished sequential-write blob
            if !assume_(Assume::ValidInput) && self.size_dt_struct.to_le() == 0 {
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

    pub fn check_header(&self) -> Result<(), FdtErr> {
        if ((self as *const _ as usize) & 7) != 0 {
            return Err(Into::into(ContentErr::Alignment));
        }

        if self.magic.to_le() != MAGIC {
            return Err(Into::into(DTBErr::BadMagic));
        }

        if !assume_(Assume::Latest) {
            if
                self.version.to_le() < config::FIRST_SUPPORTED_VERSION ||
                self.last_comp_version.to_le() > config::LAST_SUPPORTED_VERSION
            {
                return Err(Into::into(DTBErr::BadVersion));
            }
            if self.version.to_le() < self.last_comp_version.to_le() {
                return Err(Into::into(DTBErr::BadVersion));
            }
        }

        if !assume_(Assume::ValidDtb) {
            if
                self.totalsize.to_le() < self.hdr_size_() ||
                self.totalsize.to_le() > (i32::MAX as u32)
            {
                return Err(Into::into(DTBErr::Truncated));
            }

            if !self.check_offset_(self.off_mem_rsvmap.to_le()) {
                return Err(Into::into(DTBErr::Truncated));
            }
        }

        if !assume_(Assume::ValidDtb) {
            if !assume_(Assume::Latest) && self.version.to_le() < 17 {
                if !self.check_offset_(self.off_dt_struct.to_le()) {
                    return Err(Into::into(DTBErr::Truncated));
                }
            } else if !self.check_block_(self.off_dt_struct.to_le(), self.size_dt_struct.to_le()) {
                return Err(Into::into(DTBErr::Truncated));
            }

            /* bounds check strings block */
            if !self.check_block_(self.off_dt_strings.to_le(), self.size_dt_strings.to_le()) {
                return Err(Into::into(DTBErr::Truncated));
            }
        }

        Ok(())
    }

    /// check if dtb writable. should be called before write a dtb
    pub fn rw_probe(&mut self) -> Result<(), FdtErr> {
        if assume_(Assume::ValidDtb) {
            return Ok(());
        }
        Self::ro_probe(self)?;
        if !assume_(Assume::Latest) && self.version.to_le() < 17 {
            return Err(Into::into(DTBErr::BadVersion));
        }
        Self::check_ordered(self, size_of::<FdtReserveEntry>() as u32)?;
        if !assume_(Assume::Latest) && self.version.to_le() > 17 {
            self.version = fdt32::to_be(17);
        }
        Ok(())
    }
}
