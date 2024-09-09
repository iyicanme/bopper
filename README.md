# bopper

`bopper` is a Rust crate for reading and writing byte buffers in endian-aware fashion.

`LittleEndian-` and `BigEndian-` variants of `-ReadBuffer` and `-WriteBuffer` structs enable endian agnostic `get_` and `put_` operations.
Operations on a buffer with the same endianness as the target systems do not cause an endianness conversion.

## Examples

First example shows how to read an TCP header from a network packet:

```rust
fn read_tcp_header(bytes: &[u8]) {
    let buffer = BigEndianReadBuffer::new(&bytes);

    let source_port = buffer.get_u16();
    let destination_port = buffer.get_u16();

    let sequence_number = buffer.get_u32();
    let acknowledgement_number = buffer.get_u32();

    let data_offset = (buffer.get_u8() & 0xF0) >> 4;
    let flags = buffer.get_u8();
    let window_size = buffer.get_u16();

    let checksum = buffer.get_u16();
    let urgent_pointer = buffer.get_u16();

    dbg!(
        source_port, destination_port, sequence_number, acknowledgement_number,
        data_offset, flags, window_size, checksum, urgent_pointer,
    );
}
```

Second example shows how to write a UDP header to a network packet:

```rust
fn write_udp_header(source_port: u16, destination_port: u16, length: u16, checksum: u16) -> Vec<u8> {
    let buffer = BigEndianWriteBuffer::new();
    
    buffer.put_u16(source_port);
    buffer.put_u16(destination_port);
    
    buffer.put_u16(length);
    buffer.put_u16(checksum);
    
    buffer.into_vec()
}
```

## Future Work

### No alloc write buffer

Write buffer is always allocates.
Write buffer that works on a slice needs to be improved so the crate can be `no_std`

### Handling out of bounds

Currently get operations panic if not enough bytes are remaining in the read buffer.

With planned addition of no alloc write buffer, put operations will panic if there's not enough empty space.

Out of bounds should not cause a panic, instead these functions should return and `Option`.

## License

This crate is licensed under MIT License.

## Contributing

Please discuss any changes, requests and problems through GitHub issues.