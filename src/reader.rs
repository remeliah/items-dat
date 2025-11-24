use crate::error::ItemsDatError;

pub struct ByteReader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    #[inline]
    fn ensure(&self, n: usize) -> Result<(), ItemsDatError> {
        if self.pos + n > self.bytes.len() {
            Err(ItemsDatError::UnexpectedEof)
        } else {
            Ok(())
        }
    }

    pub fn read_i16(&mut self) -> Result<i16, ItemsDatError> {
        self.ensure(2)?;
        let v = i16::from_le_bytes([self.bytes[self.pos], self.bytes[self.pos + 1]]);
        self.pos += 2;

        Ok(v)
    }

    pub fn read_u16(&mut self) -> Result<u16, ItemsDatError> {
        self.ensure(2)?;
        let v = u16::from_le_bytes([self.bytes[self.pos], self.bytes[self.pos + 1]]);
        self.pos += 2;

        Ok(v)
    }

    pub fn read_i32(&mut self) -> Result<i32, ItemsDatError> {
        self.ensure(4)?;
        let v = i32::from_le_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
        ]);
        self.pos += 4;

        Ok(v)
    }

    pub fn read_u32(&mut self) -> Result<u32, ItemsDatError> {
        self.ensure(4)?;
        let v = u32::from_le_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
        ]);
        self.pos += 4;

        Ok(v)
    }

    pub fn read_u8(&mut self) -> Result<u8, ItemsDatError> {
        self.ensure(1)?;
        let v = self.bytes[self.pos];
        self.pos += 1;

        Ok(v)
    }

    pub fn read_n_bytes(&mut self, n: usize) -> Result<Vec<u8>, ItemsDatError> {
        self.ensure(n)?;
        let slice = self.bytes[self.pos..self.pos + n].to_vec();
        self.pos += n;

        Ok(slice)
    }

    pub fn read_length_prefixed_string(&mut self) -> Result<String, ItemsDatError> {
        let len = self.read_i16()?;
        if len <= 0 {
            return Ok(String::new());
        }
        
        self.ensure(len as usize)?;
        let bytes_slice = &self.bytes[self.pos..self.pos + len as usize];
        self.pos += len as usize;
        
        match std::str::from_utf8(bytes_slice) {
            Ok(s) => Ok(s.to_string()),
            Err(_) => Ok(String::from_utf8_lossy(bytes_slice).to_string()),
        }
    }

    pub fn read_xor_encrypted_string(&mut self, item_id: u32, xor_key: &[u8]) -> Result<String, ItemsDatError> {
        let len = self.read_i16()?;
        if len <= 0 {
            return Ok(String::new());
        }

        self.ensure(len as usize)?;
        let bytes_slice = &self.bytes[self.pos..self.pos + len as usize];
        self.pos += len as usize;
        
        let mut result = String::new();
        for (j, &b) in bytes_slice.iter().enumerate() {
            let k = xor_key[((j as u32 + item_id) as usize) % xor_key.len()];
            let dec = b ^ k;
            result.push(dec as char);
        }

        Ok(result)
    }
}
