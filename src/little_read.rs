use crate::ReadBuffer;
use bytes::Buf;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LittleEndianReadBuffer<'a> {
    buffer: &'a [u8],
}

impl<'a> LittleEndianReadBuffer<'a> {
    #[must_use]
    pub const fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }
}

impl ReadBuffer for LittleEndianReadBuffer<'_> {
    fn skip(&mut self, amount: usize) {
        self.buffer.advance(amount);
    }

    fn get_u8(&mut self) -> u8 {
        self.buffer.get_u8()
    }

    #[cfg(target_endian = "little")]
    fn get_u16(&mut self) -> u16 {
        self.buffer.get_u16()
    }

    #[cfg(target_endian = "big")]
    fn get_u16(&mut self) -> u16 {
        self.buffer.get_u16_le()
    }

    #[cfg(target_endian = "little")]
    fn get_u24(&mut self) -> u32 {
        u32::from(self.buffer.get_u8())
            | u32::from(self.buffer.get_u8()) >> 8
            | u32::from(self.buffer.get_u8()) >> 16
    }

    #[cfg(target_endian = "big")]
    fn get_u24(&mut self) -> u32 {
        u32::from(self.buffer.get_u8()) >> 16
            | u32::from(self.buffer.get_u8()) >> 8
            | u32::from(self.buffer.get_u8())
    }

    #[cfg(target_endian = "little")]
    fn get_u32(&mut self) -> u32 {
        self.buffer.get_u32()
    }

    #[cfg(target_endian = "big")]
    fn get_u32(&mut self) -> u32 {
        self.buffer.get_u32_le()
    }

    #[cfg(target_endian = "little")]
    fn get_u64(&mut self) -> u64 {
        self.buffer.get_u64()
    }

    #[cfg(target_endian = "big")]
    fn get_u64(&mut self) -> u64 {
        self.buffer.get_u64_le()
    }

    #[cfg(target_endian = "little")]
    fn get_u128(&mut self) -> u128 {
        self.buffer.get_u128()
    }

    #[cfg(target_endian = "big")]
    fn get_u128(&mut self) -> u128 {
        self.buffer.get_u128_le()
    }

    #[cfg(target_endian = "little")]
    fn get_f32(&mut self) -> f32 {
        self.buffer.get_f32()
    }

    #[cfg(target_endian = "big")]
    fn get_f32(&mut self) -> f32 {
        self.buffer.get_f32_le()
    }

    #[cfg(target_endian = "little")]
    fn get_f64(&mut self) -> f64 {
        self.buffer.get_f64()
    }

    #[cfg(target_endian = "big")]
    fn get_f64(&mut self) -> f64 {
        self.buffer.get_f64_le()
    }

    fn get_slice(&mut self, length: usize) -> Vec<u8> {
        self.buffer.copy_to_bytes(length).to_vec()
    }

    fn to_vec(&self) -> Vec<u8> {
        self.buffer.to_vec()
    }
}
