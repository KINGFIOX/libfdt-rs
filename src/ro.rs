use super::*;

use core::ffi::CStr;

impl FdtHeader {
    /// Get a string from the strings block.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn string(&self, stroffset: i32) -> Result<&str, FdtErr> {
        let base = self as *const _ as usize;
        if assume_(Assume::ValidInput) {
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
            if assume_(Assume::Latest) || self.version.to_le() >= 17 {
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
