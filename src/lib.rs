pub mod null;
pub mod number;

use nom::{
    branch::alt,
    combinator::{map, value},
};
use null::null;
use number::{number, Number};
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(Number),
    Null,
}

/// Parse json
///
/// ```rust
/// use nom::error::{ErrorKind, Error};
/// use nom::Err;
/// use wjson::number::Number;
/// use wjson::{parse, Value};
/// # fn main() {
///
///
/// // the parser will parse "3"
/// if let Ok(actual) = parse("3") {
///   assert_eq!(actual, Value::Number(Number::PositiveInteger(3)));
/// }
///
/// // the parser will parse "3"
/// if let Ok(actual) = parse("3") {
///   assert_eq!(actual, Value::Number(Number::PositiveInteger(3)));
/// }
/// // the parser will parse "3.2E-1"
/// if let Ok(actual) = parse("3.2E-1") {
///   assert_eq!(actual, Value::Number(Number::Float(0.32)));
/// }
/// // the parser will parse "null"
/// if let Ok(actual) = parse("null") {
///   assert_eq!(actual, Value::Null);
/// }
/// # }
/// ```
pub fn parse<'a>(input: &'a str) -> Result<Value, Box<dyn Error + 'a>> {
    let (_, result) = alt((
        map(number, |num| Value::Number(num)),
        value(Value::Null, null),
    ))(input)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_zero() {
        if let Ok(value) = parse("0") {
            assert_eq!(value, Value::Number(Number::PositiveInteger(0)))
        }
    }
}
