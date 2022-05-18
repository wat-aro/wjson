pub mod number;

use number::number;
use number::Number;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(Number),
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
/// // the parser will parse "3.2E-1"
/// if let Ok(actual) = parse("3.2E-1") {
///   assert_eq!(actual, Value::Number(Number::Float(0.32)));
/// }
/// # }
/// ```
pub fn parse<'a>(input: &'a str) -> Result<Value, Box<dyn Error + 'a>> {
    let (_, result) = number(input)?;

    Ok(Value::Number(result))
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
