//! provide: suffix __ without assume_
//! provide: suffix _ with assume_

use super::*;

/// get the size of fdt header
impl FdtHeader {
    pub fn hdr_size__(&self) -> u32 {
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

    pub fn hdr_size_(&self) -> u32 {
        if assume_(Assume::Latest) {
            return FDT_V17_SIZE as u32;
        }
        self.hdr_size__()
    }
}

/// get the ptr(abs addr) of the offset in struct section
impl FdtHeader {
    pub fn struct_addr_(&self, offset: i32) -> usize {
        // FIXME: 这里的类型转换可能有问题
        (self as *const _ as usize) + (self.off_dt_struct.to_le() as usize) + (offset as usize)
    }

    pub fn struct_addr(&self, offset: i32, checklen: u32) -> Result<usize, FdtErr> {
        if offset < 0 {
            return Err(Into::into(ParamErr::BadOffset));
        }
        if
            !assume_(Assume::ValidInput) &&
            ((self.off_dt_struct.to_le() as i64) + (offset as i64) > (u32::MAX as i64) || // 检查溢出
                (self.off_dt_struct.to_le() as i64) + (offset as i64) + (checklen as i64) >
                    (u32::MAX as i64) || // 检查溢出
                (self.off_dt_struct.to_le() as i64) + (offset as i64) + (checklen as i64) >
                    (self.totalsize.to_le() as i64)) // 检查是否越界
        {
            return Err(Into::into(DTBErr::Truncated));
        }
        if
            (assume_(Assume::Latest) || self.version.to_le() >= 17) &&
            // FIXME: 这里的类型转换可能有问题
            ((offset as i64) + (checklen as i64) > (u32::MAX as i64) ||
                (offset as i64) + (checklen as i64) > (self.size_dt_struct.to_le() as i64))
        {
            return Err(Into::into(DTBErr::Truncated));
        }
        Ok(self.struct_addr_(offset))
    }
}
