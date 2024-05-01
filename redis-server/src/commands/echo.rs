use crate::resp::types::Encoded;
use crate::SimpleString;
use std::collections::VecDeque;

pub fn execute(args: &mut VecDeque<String>) -> Box<dyn Encoded> {
    let joined: String = args
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();
    SimpleString::new(joined)
}
