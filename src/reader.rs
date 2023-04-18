#[derive(Debug)]
pub struct Reader {
    pub buf: Vec<u8>,
    pub pos: usize,
    len: usize,
}

impl From<Vec<u8>> for Reader {
    fn from(buf: Vec<u8>) -> Reader {
        let len = buf.len();
        Reader { buf: buf, pos: 0, len }
    }
}

impl Reader {
    pub fn read_u8(&mut self) -> Option<u8> {
        if self.pos < self.len {
            let val = self.buf[self.pos];
            self.pos += 1;
            Some(val)
        } else { None }
    }

    pub fn read_u16(&mut self, be: bool) -> Option<u16> {
        if self.pos < self.len {
            let buf = &self.buf[self.pos..self.pos + 2];
            self.pos += 2;
            if be {
                Some(u16::from_be_bytes(buf.try_into().unwrap()))
            } else {
                Some(u16::from_le_bytes(buf.try_into().unwrap()))
            }
        } else { None }
    }

    pub fn read_i32(&mut self, be: bool) -> Option<i32> {
        if self.pos < self.len {
            let buf = &self.buf[self.pos..self.pos + 4];
            self.pos += 4;
            if be {
                Some(i32::from_be_bytes(buf.try_into().unwrap()))
            } else {
                Some(i32::from_le_bytes(buf.try_into().unwrap()))
            }
        } else { None }
    }

    pub fn read_u32(&mut self, be: bool) -> Option<u32> {
        if self.pos < self.len {
            let buf = &self.buf[self.pos..self.pos + 4];
            self.pos += 4;
            if be {
                Some(u32::from_be_bytes(buf.try_into().unwrap()))
            } else {
                Some(u32::from_le_bytes(buf.try_into().unwrap()))
            }
        } else { None }
    }

    pub fn read_u64(&mut self, be: bool) -> Option<u64> {
        if self.pos < self.len {
            let buf = &self.buf[self.pos..self.pos + 8];
            self.pos += 8;
            if be {
                Some(u64::from_be_bytes(buf.try_into().unwrap()))
            } else {
                Some(u64::from_le_bytes(buf.try_into().unwrap()))
            }
        } else { None }
    }
}