const TERMINATOR: &str = "\r\n";

pub trait Encoded {
    fn to_encoded_string(&self) -> String;
}

pub struct SimpleString {
    value: String,
}

impl SimpleString {
    pub fn new(v: String) -> Box<SimpleString> {
        Box::new(SimpleString { value: v })
    }
}

impl Encoded for SimpleString {
    fn to_encoded_string(&self) -> String {
        let mut result = String::from("+");
        result.push_str(&self.value);
        result.push_str(TERMINATOR);

        result
    }
}

pub struct Integer {
    value: i64,
}

impl Integer {
    pub fn new(n: i64) -> Box<Integer> {
        Box::new(Integer { value: n })
    }
}

impl Encoded for Integer {
    fn to_encoded_string(&self) -> String {
        let mut result = String::from(":");
        result.push_str(&self.value.to_string());
        result.push_str(TERMINATOR);

        result
    }
}

pub struct Error {
    value: String,
}

impl Error {
    pub fn new(v: String) -> Box<Error> {
        Box::new(Error { value: v })
    }
}

impl Encoded for Error {
    fn to_encoded_string(&self) -> String {
        let mut result = String::from("-");
        result.push_str(&self.value);
        result.push_str(TERMINATOR);

        result
    }
}

pub struct BulkString {
    value: String,
}

impl BulkString {
    pub fn new(v: String) -> Box<BulkString> {
        Box::new(BulkString { value: v })
    }
}

impl Encoded for BulkString {
    fn to_encoded_string(&self) -> String {
        let mut result = String::from("$");
        result.push_str(&self.value.len().to_string());
        result.push_str(TERMINATOR);

        result.push_str(&self.value);
        result.push_str(TERMINATOR);

        result
    }
}

pub struct Array {
    entries: Vec<Box<BulkString>>,
}

impl Array {
    pub fn new() -> Box<Array> {
        Box::new(Array {
            entries: Vec::new(),
        })
    }

    pub fn push_bulk_string(&mut self, s: Box<BulkString>) {
        self.entries.push(s);
    }
}

impl Encoded for Array {
    fn to_encoded_string(&self) -> String {
        let mut result = String::from("*");
        result.push_str(&self.entries.len().to_string());
        result.push_str(TERMINATOR);

        for e in self.entries.iter() {
            result.push_str(&e.to_encoded_string());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::to_encoded_binary;

    use bitstream_io::{BigEndian, ByteWriter};

    #[test]
    fn test_integer_to_encoded_string() {
        assert_eq!(Integer::new(1).to_encoded_string(), ":1\r\n");
        assert_eq!(Integer::new(0).to_encoded_string(), ":0\r\n");
        assert_eq!(Integer::new(-1).to_encoded_string(), ":-1\r\n");

        assert_eq!(
            Integer::new(i64::MAX).to_encoded_string(),
            ":9223372036854775807\r\n"
        );

        assert_eq!(
            Integer::new(i64::MIN).to_encoded_string(),
            ":-9223372036854775808\r\n"
        );
    }

    #[test]
    fn test_error_to_encoded_string() {
        let s: Box<Error> = Error::new(String::from("error message"));
        assert_eq!(s.to_encoded_string(), "-error message\r\n");
    }

    #[test]
    fn test_error_to_encoded_string_empty() {
        let s: Box<Error> = Error::new(String::from(""));
        assert_eq!(s.to_encoded_string(), "-\r\n");
    }

    #[test]
    fn test_error_to_encoded_binary() {
        let mut actual: Vec<u8> = Vec::new();
        let mut writer = ByteWriter::endian(&mut actual, BigEndian);

        let s: Box<Error> = Error::new(String::from("error message"));

        to_encoded_binary(s, &mut writer);

        assert_eq!(
            actual,
            vec![45, 101, 114, 114, 111, 114, 32, 109, 101, 115, 115, 97, 103, 101, 13, 10]
        );
    }

    #[test]
    fn test_error_to_encoded_binary_empty() {
        let mut actual: Vec<u8> = Vec::new();
        let mut writer = ByteWriter::endian(&mut actual, BigEndian);

        let s: Box<Error> = Error::new(String::from(""));

        to_encoded_binary(s, &mut writer);

        assert_eq!(actual, vec![45, 13, 10]);
    }

    #[test]
    fn test_simple_string_to_encoded_string() {
        let s: Box<SimpleString> = SimpleString::new(String::from("meow"));
        assert_eq!(s.to_encoded_string(), "+meow\r\n");
    }

    #[test]
    fn test_simple_string_to_encoded_string_empty() {
        let s: Box<SimpleString> = SimpleString::new(String::from(""));
        assert_eq!(s.to_encoded_string(), "+\r\n");
    }

    #[test]
    fn test_simple_string_to_encoded_binary() {
        let mut actual: Vec<u8> = Vec::new();
        let mut writer = ByteWriter::endian(&mut actual, BigEndian);

        let s: Box<SimpleString> = SimpleString::new(String::from("hello"));

        to_encoded_binary(s, &mut writer);

        assert_eq!(actual, vec![43, 104, 101, 108, 108, 111, 13, 10]);
    }

    #[test]
    fn test_simple_string_to_encoded_binary_empty() {
        let mut actual: Vec<u8> = Vec::new();
        let mut writer = ByteWriter::endian(&mut actual, BigEndian);

        let s: Box<SimpleString> = SimpleString::new(String::from(""));

        to_encoded_binary(s, &mut writer);

        assert_eq!(actual, vec![43, 13, 10]);
    }

    #[test]
    fn test_array_to_encoded_binary() {
        let mut actual: Vec<u8> = Vec::new();
        let mut writer = ByteWriter::endian(&mut actual, BigEndian);

        let mut array: Box<Array> = Array::new();
        array.push_bulk_string(BulkString::new(String::from("hello")));
        array.push_bulk_string(BulkString::new(String::from("world")));

        to_encoded_binary(array, &mut writer);

        assert_eq!(
            actual,
            vec![
                42, 50, 13, 10, 36, 53, 13, 10, 104, 101, 108, 108, 111, 13, 10, 36, 53, 13, 10,
                119, 111, 114, 108, 100, 13, 10
            ],
        );
    }

    #[test]
    fn test_array_to_encoded_string() {
        let mut array = Array::new();
        array.push_bulk_string(BulkString::new(String::from("hello")));
        array.push_bulk_string(BulkString::new(String::from("world")));

        assert_eq!(
            array.to_encoded_string(),
            "*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_array_to_encoded_string_empty() {
        let array = Array::new();

        assert_eq!(array.to_encoded_string(), "*0\r\n",);
    }

    #[test]
    fn test_bulk_string_to_encoded_binary() {
        let mut actual: Vec<u8> = Vec::new();
        let mut writer = ByteWriter::endian(&mut actual, BigEndian);

        let bulk_string = BulkString::new(String::from("hello"));
        to_encoded_binary(bulk_string, &mut writer);

        assert_eq!(
            actual,
            vec![36, 53, 13, 10, 104, 101, 108, 108, 111, 13, 10]
        );
    }

    #[test]
    fn test_bulk_string_to_encoded_binary_empty() {
        let mut actual: Vec<u8> = Vec::new();
        let mut writer = ByteWriter::endian(&mut actual, BigEndian);

        let bulk_string = BulkString::new(String::from(""));
        to_encoded_binary(bulk_string, &mut writer);

        // expected: "+0\r\n\r\n" as bytes
        assert_eq!(actual, vec![36, 48, 13, 10, 13, 10]);
    }
}
