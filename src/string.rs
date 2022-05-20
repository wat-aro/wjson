use crate::number::digit;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, satisfy},
    combinator::{map, recognize, value},
    multi::many1,
    sequence::{delimited, tuple},
    AsChar, IResult,
};

#[derive(Debug, PartialEq)]
pub struct JsonString(pub String);

/// Recognize string
/// ```rust
/// use wson::string::{string, JsonString};
/// # fn main() {
/// if let Ok(value) = string("\"\"") {
///   assert_eq!(value, ("", JsonString("".to_string())))
/// }
///
/// if let Ok(value) = string("hello") {
///   assert_eq!(value, ("", JsonString("hello".to_string())))
/// }
///
/// if let Ok(value) = string("こんにちは") {
///   assert_eq!(value, ("", JsonString("こんにちは".to_string())))
/// }
///
/// if let Ok(value) = string("abc123") {
///   assert_eq!(value, ("", JsonString("abc123".to_string())))
/// }
///
/// if let Ok(value) = string("\"Hello\"") {
///   assert_eq!(value, ("", JsonString("\"Hello\"".to_string())))
/// }
/// # }
/// ```
pub fn string(input: &str) -> IResult<&str, JsonString> {
    map(delimited(tag("\""), characters, tag("\"")), |str: &str| {
        JsonString(str.to_string())
    })(input)
}

fn characters(input: &str) -> IResult<&str, &str> {
    alt((recognize(many1(character)), tag("")))(input)
}

fn character(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(many1(satisfy(|c| c.is_alphanum()))),
        recognize(tuple((tag("\\"), escape))),
        value("", tag("")),
    ))(input)
}

// escape = '"' DoubleQuote
//        | '\' Backslash
//        | '/' Slash
//        | 'b' Boundary
//        | 'f' FormFeed
//        | 'n' NewLine
//        | 'r' CarriageReturn
//        | 't' Tab
//        | 'u' hex hex hex hex
fn escape(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(one_of("\"\\/bfnrt")),
        recognize(tuple((tag("u"), hex, hex, hex, hex))),
    ))(input)
}

// hex = digit
//     | 'A' . 'F'
//     | 'a' . 'f'
fn hex(input: &str) -> IResult<&str, &str> {
    alt((recognize(digit), recognize(one_of("abcdefABCDEF"))))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_string() {
        if let Ok(value) = string("\"\"") {
            assert_eq!(value, ("", JsonString("".to_string())))
        }
    }

    #[test]
    fn parse_hello_string() {
        if let Ok(value) = string("hello") {
            assert_eq!(value, ("", JsonString("hello".to_string())))
        }
    }

    #[test]
    fn parse_utf8_string() {
        if let Ok(value) = string("こんにちは") {
            assert_eq!(value, ("", JsonString("こんにちは".to_string())))
        }
    }

    #[test]
    fn parse_alphanum_string() {
        if let Ok(value) = string("abc123") {
            assert_eq!(value, ("", JsonString("abc123".to_string())))
        }
    }

    #[test]
    fn hex_five() {
        if let Ok(value) = hex("5") {
            assert_eq!(value, ("", "5"))
        }
    }

    #[test]
    fn hex_f() {
        if let Ok(value) = hex("f") {
            assert_eq!(value, ("", "f"))
        }
    }

    #[test]
    fn hex_large_f() {
        if let Ok(value) = hex("F") {
            assert_eq!(value, ("", "F"))
        }
    }

    #[test]
    fn escape_slash() {
        if let Ok(value) = escape("/") {
            assert_eq!(value, ("", "/"))
        }
    }

    #[test]
    fn escape_unicode() {
        if let Ok(value) = escape("u1234") {
            assert_eq!(value, ("", "u1234"))
        }
    }

    #[test]
    fn character_unicode() {
        if let Ok(value) = character("\\u1234") {
            assert_eq!(value, ("", "\\u1234"))
        }
    }
}
