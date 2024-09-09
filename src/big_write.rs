use bytes::BufMut;

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BigEndianWriteBuffer {
    buffer: Vec<u8>,
}

impl BigEndianWriteBuffer {
    #[must_use]
    pub const fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }

    pub fn skip(&mut self, amount: usize) {
        let new_size = self.buffer.len() + amount;
        self.buffer.resize(new_size, 0x00);
    }

    pub fn put_u8(&mut self, value: u8) {
        self.buffer.put_u8(value);
    }

    #[cfg(target_endian = "big")]
    pub fn put_u16(&mut self, value: u16) {
        self.buffer.put_u16(value);
    }

    #[cfg(target_endian = "little")]
    pub fn put_u16(&mut self, value: u16) {
        self.buffer.put_u16_le(value);
    }

    #[cfg(target_endian = "big")]
    pub fn put_u32(&mut self, value: u32) {
        self.buffer.put_u32(value);
    }

    #[cfg(target_endian = "little")]
    pub fn put_u32(&mut self, value: u32) {
        self.buffer.put_u32_le(value);
    }

    #[cfg(target_endian = "big")]
    pub fn put_u64(&mut self, value: u64) {
        self.buffer.put_u64(value);
    }

    #[cfg(target_endian = "little")]
    pub fn put_u64(&mut self, value: u64) {
        self.buffer.put_u64_le(value);
    }

    #[cfg(target_endian = "big")]
    pub fn put_u128(&mut self, value: u128) {
        self.buffer.put_u128(value);
    }

    #[cfg(target_endian = "little")]
    pub fn put_u128(&mut self, value: u128) {
        self.buffer.put_u128_le(value);
    }

    pub fn put_slice(&mut self, slice: &[u8]) {
        self.buffer.extend(slice);
    }
}
