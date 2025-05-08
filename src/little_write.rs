use crate::WriteBuffer;
use bytes::BufMut;

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LittleEndianWriteBuffer {
    buffer: Vec<u8>,
}

impl LittleEndianWriteBuffer {
    #[must_use]
    pub const fn new() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl WriteBuffer for LittleEndianWriteBuffer {
    #[must_use]
    fn into_vec(self) -> Vec<u8> {
        self.buffer
    }

    fn skip(&mut self, amount: usize) {
        let new_size = self.buffer.len() + amount;
        self.buffer.resize(new_size, 0x00);
    }

    fn put_u8(&mut self, value: u8) {
        self.buffer.put_u8(value);
    }

    #[cfg(target_endian = "little")]
    fn put_u16(&mut self, value: u16) {
        self.buffer.put_u16(value);
    }

    #[cfg(target_endian = "big")]
    fn put_u16(&mut self, value: u16) {
        self.buffer.put_u16_le(value);
    }

    #[cfg(target_endian = "little")]
    fn put_u24(&mut self, value: u32) {
        self.buffer.put_u8((value & 0x00_00_00_FF) as u8);
        self.buffer.put_u8(((value & 0x00_00_FF_00) >> 8) as u8);
        self.buffer.put_u8(((value & 0x00_FF_00_00) >> 16) as u8);
    }

    #[cfg(target_endian = "big")]
    fn get_u24(&mut self, value: u32) {
        self.buffer.put_u8(((value & 0x00_FF_00_00) >> 16) as u8);
        self.buffer.put_u8(((value & 0x00_00_FF_00) >> 8) as u8);
        self.buffer.put_u8((value & 0x00_00_00_FF) as u8);
    }

    #[cfg(target_endian = "little")]
    fn put_u32(&mut self, value: u32) {
        self.buffer.put_u32(value);
    }

    #[cfg(target_endian = "big")]
    fn put_u32(&mut self, value: u32) {
        self.buffer.put_u32_le(value);
    }

    #[cfg(target_endian = "little")]
    fn put_u64(&mut self, value: u64) {
        self.buffer.put_u64(value);
    }

    #[cfg(target_endian = "big")]
    fn put_u64(&mut self, value: u64) {
        self.buffer.put_u64_le(value);
    }

    #[cfg(target_endian = "little")]
    fn put_u128(&mut self, value: u128) {
        self.buffer.put_u128(value);
    }

    #[cfg(target_endian = "big")]
    fn put_u128(&mut self, value: u128) {
        self.buffer.put_u128_le(value);
    }

    #[cfg(target_endian = "little")]
    fn put_f32(&mut self, value: f32) {
        self.buffer.put_f32(value);
    }

    #[cfg(target_endian = "big")]
    fn put_f32(&mut self, value: f32) {
        self.buffer.put_f32_le(value);
    }

    #[cfg(target_endian = "little")]
    fn put_f64(&mut self, value: f64) {
        self.buffer.put_f64(value);
    }

    #[cfg(target_endian = "big")]
    fn put_f64(&mut self, value: f64) {
        self.buffer.put_f64_le(value);
    }

    fn put_slice(&mut self, slice: &[u8]) {
        self.buffer.extend(slice);
    }
}
