pub use big_read::BigEndianReadBuffer;
pub use big_write::BigEndianWriteBuffer;
pub use little_read::LittleEndianReadBuffer;
pub use little_write::LittleEndianWriteBuffer;

mod big_read;
mod big_write;
mod little_read;
mod little_write;

pub trait ReadBuffer {
    fn skip(&mut self, amount: usize) -> Option<()>;

    fn get_u8(&mut self) -> Option<u8>;

    fn get_u16(&mut self) -> Option<u16>;

    fn get_u24(&mut self) -> Option<u32>;

    fn get_u32(&mut self) -> Option<u32>;

    fn get_u64(&mut self) -> Option<u64>;

    fn get_u128(&mut self) -> Option<u128>;

    fn get_f32(&mut self) -> Option<f32>;

    fn get_f64(&mut self) -> Option<f64>;

    fn get_slice(&mut self, length: usize) -> Option<Vec<u8>>;
    
    fn get_buffer(&mut self, length: usize) -> Option<impl ReadBuffer>;

    fn to_vec(&self) -> Vec<u8>;

    fn remaining(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn offset(&self) -> usize;
}

pub trait WriteBuffer {
    #[must_use]
    fn into_vec(self) -> Vec<u8>;

    fn skip(&mut self, amount: usize);

    fn put_u8(&mut self, value: u8);

    fn put_u16(&mut self, value: u16);

    fn put_u24(&mut self, value: u32);

    fn put_u32(&mut self, value: u32);

    fn put_u64(&mut self, value: u64);

    fn put_u128(&mut self, value: u128);

    fn put_f32(&mut self, value: f32);

    fn put_f64(&mut self, value: f64);

    fn put_slice(&mut self, slice: &[u8]);
}
