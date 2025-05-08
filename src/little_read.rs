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
    fn skip(&mut self, amount: usize) -> bool {
        if amount > self.buffer.len() {
            return false;
        }

        self.buffer.advance(amount);

        true
    }

    fn get_u8(&mut self) -> Option<u8> {
        if self.buffer.is_empty() {
            return None;
        }

        Some(self.buffer.get_u8())
    }

    #[cfg(target_endian = "little")]
    fn get_u16(&mut self) -> Option<u16> {
        if self.buffer.len() < size_of::<u16>() {
            return None;
        }

        Some(self.buffer.get_u16())
    }

    #[cfg(target_endian = "big")]
    fn get_u16(&mut self) -> Option<u16> {
        if self.buffer.len() < size_of::<u16>() {
            return None;
        }

        Some(self.buffer.get_u16_le())
    }

    #[cfg(target_endian = "little")]
    fn get_u24(&mut self) -> Option<u32> {
        if self.buffer.len() < 3 {
            return None;
        }

        let u24 = u32::from(self.buffer.get_u8())
            | u32::from(self.buffer.get_u8()) >> 8
            | u32::from(self.buffer.get_u8()) >> 16;

        Some(u24)
    }

    #[cfg(target_endian = "big")]
    fn get_u24(&mut self) -> Option<u32> {
        if self.buffer.len() < 3 {
            return None;
        }

        let u24 = u32::from(self.buffer.get_u8()) >> 16
            | u32::from(self.buffer.get_u8()) >> 8
            | u32::from(self.buffer.get_u8());

        Some(u24)
    }

    #[cfg(target_endian = "little")]
    fn get_u32(&mut self) -> Option<u32> {
        if self.buffer.len() < size_of::<u32>() {
            return None;
        }

        Some(self.buffer.get_u32())
    }

    #[cfg(target_endian = "big")]
    fn get_u32(&mut self) -> Option<u32> {
        if self.buffer.len() < size_of::<u32>() {
            return None;
        }

        Some(self.buffer.get_u32_le())
    }

    #[cfg(target_endian = "little")]
    fn get_u64(&mut self) -> Option<u64> {
        if self.buffer.len() < size_of::<u64>() {
            return None;
        }

        Some(self.buffer.get_u64())
    }

    #[cfg(target_endian = "big")]
    fn get_u64(&mut self) -> Option<u64> {
        if self.buffer.len() < size_of::<u64>() {
            return None;
        }

        Some(self.buffer.get_u64_le())
    }

    #[cfg(target_endian = "little")]
    fn get_u128(&mut self) -> Option<u128> {
        if self.buffer.len() < size_of::<u128>() {
            return None;
        }

        Some(self.buffer.get_u128())
    }

    #[cfg(target_endian = "big")]
    fn get_u128(&mut self) -> Option<u128> {
        if self.buffer.len() < size_of::<u128>() {
            return None;
        }

        Some(self.buffer.get_u128_le())
    }

    #[cfg(target_endian = "little")]
    fn get_f32(&mut self) -> Option<f32> {
        if self.buffer.len() < size_of::<f32>() {
            return None;
        }

        Some(self.buffer.get_f32())
    }

    #[cfg(target_endian = "big")]
    fn get_f32(&mut self) -> Option<f32> {
        if self.buffer.len() < size_of::<f32>() {
            return None;
        }

        Some(self.buffer.get_f32_le())
    }

    #[cfg(target_endian = "little")]
    fn get_f64(&mut self) -> Option<f64> {
        if self.buffer.len() < size_of::<f64>() {
            return None;
        }

        Some(self.buffer.get_f64())
    }

    #[cfg(target_endian = "big")]
    fn get_f64(&mut self) -> Option<f64> {
        if self.buffer.len() < size_of::<f64>() {
            return None;
        }

        Some(self.buffer.get_f64_le())
    }

    fn get_slice(&mut self, length: usize) -> Option<Vec<u8>> {
        if self.buffer.len() < length {
            return None;
        }

        Some(self.buffer.copy_to_bytes(length).to_vec())
    }

    fn get_buffer(&mut self, length: usize) -> Option<impl ReadBuffer> {
        if self.buffer.len() < length {
            return None;
        }
        
        let (buffer, own) = self.buffer.split_at(length);
        self.buffer = own;
        
        Some(Self { buffer })
    }

    fn to_vec(&self) -> Vec<u8> {
        self.buffer.to_vec()
    }
}
