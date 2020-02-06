use std::fmt::Write;
use std::str;

pub fn encode(val: u8, mut buf: &mut String) -> &[u8] {
    let _ = write!(&mut buf, "{}", val); // some computation of the message data to be sent
    buf.as_bytes()
}

pub fn decode(val: &[u8]) -> String {
    str::from_utf8(val).unwrap().to_owned()
}
