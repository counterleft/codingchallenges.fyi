use crate::resp::types::Encoded;
use crate::SimpleString;
use std::collections::VecDeque;

pub fn execute(_args: &mut VecDeque<String>) -> Box<dyn Encoded> {
    SimpleString::new(String::from("PONG"))
}
