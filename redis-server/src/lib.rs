mod commands;
mod resp;

use resp::types::{Encoded, Error, SimpleString};

use std::collections::VecDeque;
use std::fmt;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::net::{TcpListener, TcpStream};

pub fn listen(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let _ = handle_connection(&mut stream);
            }
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    // XXX Max length of command is 512B. We should be able to do up to 512MB based on the RESP
    // protocol.
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer[..])?;

    let mut input: Vec<u8> = vec![];
    for i in 0..bytes_read {
        input.push(buffer[i]);
    }
    println!("{:?}", input);

    let mut decoded = match decode_resp(&input) {
        Ok(decoded) => decoded,
        Err(e) => {
            let reply = Error::new(String::from(e.to_string())).to_encoded_string();
            let _ = stream.write_all(reply.as_bytes());
            return Ok(());
        }
    };

    if decoded.len() < 1 {
        let reply = Error::new(String::from("expected a command")).to_encoded_string();
        let _ = stream.write_all(reply.as_bytes());
        return Ok(());
    }

    let mut cmd = Command {
        command: decoded.pop_front().unwrap(),
        args: decoded,
    };

    println!("{:?}", cmd);

    let reply = handle_reply(&mut cmd);

    let _ = stream.write_all(reply.as_bytes());
    Ok(())
}

fn handle_reply(cmd: &mut Command) -> String {
    let reply: Box<dyn Encoded> = match cmd.command.to_lowercase().as_str() {
        "ping" => commands::ping::execute(&mut cmd.args),
        "echo" => commands::echo::execute(&mut cmd.args),
        _ => Error::new(String::from("UNIMPLEMENTED")),
    };

    reply.to_encoded_string()
}

#[derive(Debug)]
struct Command {
    command: String,
    args: VecDeque<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum RedisError {
    NotAnArrayError,
}

impl fmt::Display for RedisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RedisError::NotAnArrayError => write!(f, "expected an array resp type (*)"),
        }
    }
}

impl std::error::Error for RedisError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            RedisError::NotAnArrayError => None,
        }
    }
}

type Result<T> = std::result::Result<T, RedisError>;

fn decode_resp(input: &Vec<u8>) -> Result<VecDeque<String>> {
    let mut cursor = Cursor::new(input);

    let decoded_type = read_bytes(&mut cursor, 1);
    match decoded_type.as_str() {
        "*" => Ok(do_decode_array(&mut cursor)),
        _ => Err(RedisError::NotAnArrayError),
    }
}

/// Reads num_bytes through the cursor and returns the results as a String.
fn read_bytes(cursor: &mut Cursor<&Vec<u8>>, num_bytes: usize) -> String {
    let mut buf = vec![0u8; num_bytes];
    cursor.read_exact(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

/// Reads sequentially through the cursor until it hits a CRLF.
/// At that point, it parses the underlying Vec<u8> into String and from there into an integer.
fn read_integer(mut cursor: &mut Cursor<&Vec<u8>>) -> usize {
    let mut result = read_bytes(&mut cursor, 1);

    while !result.ends_with("\r\n") {
        result = result + &read_bytes(&mut cursor, 1);
    }

    let trimmed = String::from(result.trim_end());
    trimmed.parse().unwrap()
}

fn do_decode_bulk_string(mut cursor: &mut Cursor<&Vec<u8>>) -> String {
    let bulk_string_len = read_integer(&mut cursor);
    let result = read_bytes(&mut cursor, bulk_string_len);

    // Seek past the TERMINATOR
    cursor.seek(SeekFrom::Current(2)).unwrap();

    result
}

fn do_decode_array(mut cursor: &mut Cursor<&Vec<u8>>) -> VecDeque<String> {
    let array_len = read_integer(&mut cursor);

    let mut elements: VecDeque<String> = VecDeque::with_capacity(array_len);

    for _i in 0..array_len {
        // Seek past the bulk string type character
        // XXX It may make more sense to check that the type is a bulk string type (defense for bad input)
        cursor.seek(SeekFrom::Current(1)).unwrap();

        let elem = do_decode_bulk_string(&mut cursor);
        elements.push_back(elem);
    }

    elements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_algo_array() -> Result<()> {
        // Input: *1\r\n$4\r\nPING\r\n
        let input: Vec<u8> = vec![42, 49, 13, 10, 36, 52, 13, 10, 80, 73, 78, 71, 13, 10];

        assert_eq!(decode_resp(&input)?, VecDeque::from([String::from("PING")]));

        Ok(())
    }

    #[test]
    fn test_decode_error_not_an_array() -> Result<()> {
        // Input: $3\r\nBAD\r\n
        let input: Vec<u8> = vec![36, 51, 13, 10, 66, 65, 68, 13, 10];

        match decode_resp(&input) {
            Ok(_) => panic!("Expecting error"),
            Err(e) => assert_eq!(e, RedisError::NotAnArrayError),
        }

        Ok(())
    }
}
