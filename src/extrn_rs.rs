use super::*;

impl FdtHeader {
    pub fn next_tag(&self, offset: i32) -> Result<Token, FdtErr> {
        let start_offset = offset;
        let tagp = self.struct_addr(start_offset, TAGSIZE as u32)?; // FIXME: 这里有待商榷
        let tag = unsafe {
            let tagp = tagp as *const u32;
            (*tagp).swap_bytes()
        };
        let offset = start_offset + (TAGSIZE as i32);
        if tag == (Token::FdtBeginNode as u32) {
            // skip name
        } else {
            return Ok(Token::FdtEnd);
        }
        todo!();
    }
}
