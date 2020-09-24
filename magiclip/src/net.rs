use anyhow::Result;
use std::str;

pub fn decode_buffer<'a>(buffer: &'a [u8]) -> Result<&'a str> {
    Ok(str::from_utf8(&buffer)?.trim_matches(char::from(0)))
}
