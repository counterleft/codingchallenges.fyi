pub mod types;

use bitstream_io::{BigEndian, ByteWrite, ByteWriter};
use types::Encoded;

pub fn from_binary(b: Vec<u8>) -> String {
    String::from_utf8(b).unwrap()
}

pub fn to_encoded_binary<W: std::io::Write>(
    t: Box<dyn Encoded>,
    writer: &mut ByteWriter<W, BigEndian>,
) {
    let binding = t.to_encoded_string();
    let bytes = binding.as_bytes();
    writer.write_bytes(bytes).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_binary() {
        let binary: Vec<u8> = vec![43, 104, 101, 108, 108, 111, 13, 10];
        assert_eq!(from_binary(binary), "+hello\r\n");
    }
}
