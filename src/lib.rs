pub mod boolean;
pub mod null;
pub mod number;
pub mod string;

use boolean::{false_parser, true_parser};
use nom::{
    branch::alt,
    character::complete::space0,
    combinator::{map, value},
    sequence::delimited,
    IResult,
};
use null::null;
use number::{number, Number};
use std::error::Error;
use string::string;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(Number),
    String(String),
    Null,
    True,
    False,
}

/// Parse json
///
/// ```rust
/// use nom::error::{ErrorKind, Error};
/// use nom::Err;
/// use wson::number::Number;
/// use wson::{parse, Value};
/// # fn main() {
///
///
/// // the parser will parse "3"
/// if let Ok(actual) = parse("3") {
///   assert_eq!(actual, Value::Number(Number::PositiveInteger(3)));
/// }
///
/// // the parser will parse " 3 "
/// if let Ok(actual) = parse(" 3 ") {
///   assert_eq!(actual, Value::Number(Number::PositiveInteger(3)));
/// }
///
/// // the parser will parse "3.2E-1"
/// if let Ok(actual) = parse("3.2E-1") {
///   assert_eq!(actual, Value::Number(Number::Float(0.32)));
/// }
/// // the parser will parse "null"
/// if let Ok(actual) = parse("null") {
///   assert_eq!(actual, Value::Null);
/// }
/// // the parser will parse "true"
/// if let Ok(actual) = parse("true") {
///   assert_eq!(actual, Value::True);
/// }
/// // the parser will parse "false"
/// if let Ok(actual) = parse("false") {
///   assert_eq!(actual, Value::False);
/// }
///
/// // the parser will parse "\"hello\""
/// if let Ok(actual) = parse("\"hello\"") {
///   assert_eq!(actual, Value::String("hello".to_string()))
/// }
/// # }
/// ```
pub fn parse<'a>(input: &'a str) -> Result<Value, Box<dyn Error + 'a>> {
    let (_, result) = json(input)?;

    Ok(result)
}

fn json(input: &str) -> IResult<&str, Value> {
    element(input)
}

fn element(input: &str) -> IResult<&str, Value> {
    delimited(ws, value_parser, ws)(input)
}

fn value_parser(input: &str) -> IResult<&str, Value> {
    alt((
        map(number, |num| Value::Number(num)),
        map(string, |json_string| Value::String(json_string.0)),
        value(Value::Null, null),
        value(Value::True, true_parser),
        value(Value::False, false_parser),
    ))(input)
}

fn ws(input: &str) -> IResult<&str, &str> {
    space0(input)
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
