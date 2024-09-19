//! This file defined some checks the sanity of fdt

use super::*;
use core::ffi::CStr;

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

    /// check dtb valid. should be read before read a dtb
    pub fn ro_probe(&self) -> Result<(), FdtErr> {
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

    /// check if dtb writable. should be called before write a dtb
    pub fn rw_probe(&mut self) -> Result<(), FdtErr> {
        if assume(Assume::ValidDtb) {
            return Ok(());
        }
        Self::ro_probe(self)?;
        if !assume(Assume::Latest) && self.version.to_le() < 17 {
            return Err(Into::into(DTBErr::BadVersion));
        }
        Self::check_ordered(self, size_of::<FdtReserveEntry>() as u32)?;
        if !assume(Assume::Latest) && self.version.to_le() > 17 {
            self.version = fdt32::to_be(17);
        }
        Ok(())
    }
}

impl FdtHeader {
    fn get_string(&self, stroffset: i32) -> Result<&str, FdtErr> {
        let base = self as *const _ as usize;
        if assume(Assume::ValidInput) {
            let s_ptr = base + (self.off_dt_strings.to_le() as usize) + (stroffset as usize);
            let s_ptr = s_ptr as *const i8;
            let s = unsafe {
                CStr::from_ptr(s_ptr)
                    .to_str()
                    .map_err(|_| ContentErr::BadValue)?
            };
            return Ok(s);
        }

        self.ro_probe()?; // check

        if self.off_dt_strings.to_le() + (stroffset as u32) >= self.totalsize.to_le() {
            return Err(Into::into(DTBErr::Truncated));
        }

        if self.magic.to_le() == MAGIC {
            if stroffset < 0 {
                return Err(Into::into(ParamErr::BadOffset));
            }
            if assume(Assume::Latest) || self.version.to_le() >= 17 {
                // FIXME: 这里的类型转换可能有问题
                if stroffset >= (self.size_dt_strings.to_le() as i32) {
                    return Err(Into::into(ParamErr::BadOffset));
                }
                if
                    // string section 还剩多少 bytes
                    (self.size_dt_strings.to_le() as i32) - stroffset >
                    // 整个 fdt 还剩多少 bytes
                    (self.totalsize.to_le() as i32) -
                        ((self.off_dt_strings.to_le() as i32) + stroffset)
                {
                    // 不应该出现: string 剩余长度 比 fdt 剩余长度 长
                    return Err(Into::into(DTBErr::Truncated));
                }
                // if ((fdt_size_dt_strings(fdt) - stroffset) < len) len = fdt_size_dt_strings(fdt) - stroffset;
            }
        } else if self.magic.to_le() == !MAGIC {
            // unfinished sequential-write blob
            if self.size_dt_struct.to_le() == 0 {
                return Err(Into::into(ParamErr::BadState));
            }
            // if (sw_stroffset < len) len = sw_stroffset;
        } else {
            return Err(Into::into(DTBErr::BadMagic));
        }

        let s_ptr = base + (self.off_dt_strings.to_le() as usize) + (stroffset as usize);
        let s_ptr = s_ptr as *const i8;
        let s = unsafe {
            CStr::from_ptr(s_ptr)
                .to_str()
                .map_err(|_| ContentErr::BadValue)?
        };
        Ok(s)
    }
}
