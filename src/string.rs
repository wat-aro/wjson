use crate::number::digit;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, satisfy},
    combinator::{map, recognize, value},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct JsonString(pub String);

/// Recognize string
/// ```rust
/// use wson::string::{string, JsonString};
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let value = string("\"\"")?;
/// assert_eq!(value, ("", JsonString("".to_string())));
///
/// let value = string("\"hello\"")?;
/// assert_eq!(value, ("", JsonString("hello".to_string())));
///
/// let value = string("\"こんにちは\"")?;
/// assert_eq!(value, ("", JsonString("こんにちは".to_string())));
///
/// let value = string("\"abc123\"")?;
/// assert_eq!(value, ("", JsonString("abc123".to_string())));
///
/// let value = string("\"He\\\"\\\"llo\"")?;
/// assert_eq!(value, ("", JsonString("He\\\"\\\"llo".to_string())));
///
/// # Ok(())
/// # }
/// ```
pub fn string(input: &str) -> IResult<&str, JsonString> {
    map(delimited(tag("\""), characters, tag("\"")), |str: &str| {
        JsonString(str.to_string())
    })(input)
}

fn characters(input: &str) -> IResult<&str, &str> {
    recognize(many0(character))(input)
}

fn character(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(tuple((tag("\\"), escape))),
        recognize(satisfy(|c| c != '"')),
        value("", one_of("")),
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
    use std::error::Error;

    use super::*;

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    fn parse_empty_string() -> TestResult {
        let value = string("\"\"")?;
        assert_eq!(value, ("", JsonString("".to_string())));
        Ok(())
    }

    #[test]
    fn parse_hello_string() -> TestResult {
        let value = string("\"hello\"")?;
        assert_eq!(value, ("", JsonString("hello".to_string())));
        Ok(())
    }

    #[test]
    fn parse_utf8_string() -> TestResult {
        let value = string("\"こんにちは\"")?;
        assert_eq!(value, ("", JsonString("こんにちは".to_string())));
        Ok(())
    }

    #[test]
    fn parse_alphanum_string() -> TestResult {
        let value = string("\"abc123\"")?;
        assert_eq!(value, ("", JsonString("abc123".to_string())));
        Ok(())
    }

    #[test]
    fn hex_five() -> TestResult {
        let value = hex("5")?;
        assert_eq!(value, ("", "5"));
        Ok(())
    }

    #[test]
    fn hex_f() -> TestResult {
        let value = hex("f")?;
        assert_eq!(value, ("", "f"));
        Ok(())
    }

    #[test]
    fn hex_large_f() -> TestResult {
        let value = hex("F")?;
        assert_eq!(value, ("", "F"));
        Ok(())
    }

    #[test]
    fn escape_slash() -> TestResult {
        let value = escape("/")?;
        assert_eq!(value, ("", "/"));
        Ok(())
    }

    #[test]
    fn escape_unicode() -> TestResult {
        let value = escape("u1234")?;
        assert_eq!(value, ("", "u1234"));
        Ok(())
    }

    #[test]
    fn character_unicode() -> TestResult {
        let value = character("\\u1234")?;
        assert_eq!(value, ("", "\\u1234"));
        Ok(())
    }
}
