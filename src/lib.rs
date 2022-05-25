pub mod boolean;
pub mod null;
pub mod number;
pub mod string;

use boolean::{false_parser, true_parser};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{map, value},
    sequence::{delimited, separated_pair},
    IResult,
};
use null::null;
use number::{number, Number};
use std::{collections::HashMap, error::Error};
use string::string;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
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
/// # use std::error;
/// use std::collections::HashMap;
/// # fn main() -> Result<(), Box<dyn error::Error>> {
///
///
/// // the parser will parse "3"
/// let actual = parse("3")?;
/// assert_eq!(actual, Value::Number(Number::PositiveInteger(3)));
///
/// // the parser will parse " 3 "
/// let actual = parse(" 3 ")?;
/// assert_eq!(actual, Value::Number(Number::PositiveInteger(3)));
///
/// // the parser will parse "3.2E-1"
/// let actual = parse("3.2E-1")?;
/// assert_eq!(actual, Value::Number(Number::Float(0.32)));
///
/// // the parser will parse "null"
/// let actual = parse("null")?;
/// assert_eq!(actual, Value::Null);
///
/// // the parser will parse "true"
/// let actual = parse("true")?;
/// assert_eq!(actual, Value::True);
///
/// // the parser will parse "false"
/// let actual = parse("false")?;
/// assert_eq!(actual, Value::False);
///
/// // the parser will parse "\"hello\""
/// let actual = parse("\"hello\"")?;
/// assert_eq!(actual, Value::String("hello".to_string()));
///
/// // the parser will parse "{\"title\": \"TITLE1\", \"revision\": 12}"
/// let value = parse("{\"title\": \"TITLE1\", \"revision\": 12}")?;
/// let mut h = HashMap::new();
/// h.insert("title".to_string(), Value::String("TITLE1".to_string()));
/// h.insert(
///     "revision".to_string(),
///     Value::Number(Number::PositiveInteger(12)),
/// );
/// assert_eq!(value, Value::Object(h));
///
/// # Ok(())
/// # }
/// ```
pub fn parse<'a>(input: &'a str) -> Result<Value, Box<dyn Error + 'a>> {
    let (_, result) = json(input)?;

    Ok(result)
}

fn json(input: &str) -> IResult<&str, Value> {
    element(input)
}

fn value_parser(input: &str) -> IResult<&str, Value> {
    alt((
        map(object, |m| Value::Object(m)),
        map(array, |v| Value::Array(v)),
        map(number, |num| Value::Number(num)),
        map(string, |json_string| Value::String(json_string.0)),
        value(Value::Null, null),
        value(Value::True, true_parser),
        value(Value::False, false_parser),
    ))(input)
}

fn object(input: &str) -> IResult<&str, HashMap<String, Value>> {
    alt((
        value(HashMap::new(), delimited(tag("{"), ws, tag("}"))),
        map(delimited(tag("{"), members, tag("}")), |v| {
            let mut h = HashMap::new();
            for (key, value) in v.into_iter() {
                h.insert(key, value);
            }
            h
        }),
    ))(input)
}

fn members(input: &str) -> IResult<&str, Vec<(String, Value)>> {
    alt((
        map(separated_pair(member, tag(","), members), |(m, ms)| {
            let vec = vec![m];
            [vec, ms].concat()
        }),
        map(member, |p| vec![p]),
    ))(input)
}

fn member(input: &str) -> IResult<&str, (String, Value)> {
    map(
        separated_pair(delimited(ws, string, ws), tag(":"), element),
        |(key, value)| (key.0, value),
    )(input)
}

fn array(input: &str) -> IResult<&str, Vec<Value>> {
    alt((
        value(vec![], delimited(tag("["), ws, tag("]"))),
        delimited(tag("["), elements, tag("]")),
    ))(input)
}

fn elements(input: &str) -> IResult<&str, Vec<Value>> {
    alt((
        map(
            separated_pair(element, tag(","), elements),
            |(e, es): (Value, Vec<Value>)| {
                let vec = vec![e];
                [vec, es].concat()
            },
        ),
        map(element, |e| vec![e]),
    ))(input)
}

fn element(input: &str) -> IResult<&str, Value> {
    delimited(ws, value_parser, ws)(input)
}

fn ws(input: &str) -> IResult<&str, &str> {
    space0(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error;

    type TestResult = Result<(), Box<dyn error::Error>>;

    #[test]
    fn parse_zero() -> TestResult {
        let value = parse("0")?;
        assert_eq!(value, Value::Number(Number::PositiveInteger(0)));
        Ok(())
    }

    #[test]
    fn empty_array() -> TestResult {
        let value = array("[]")?;
        assert_eq!(value, ("", vec![]));
        Ok(())
    }

    #[test]
    fn a_number_array() -> TestResult {
        let value = array("[1]")?;
        assert_eq!(value, ("", vec![Value::Number(Number::PositiveInteger(1))]));
        Ok(())
    }

    #[test]
    fn multiple_number_array() -> TestResult {
        let value = array("[1, 2]")?;
        assert_eq!(
            value,
            (
                "",
                vec![
                    Value::Number(Number::PositiveInteger(1)),
                    Value::Number(Number::PositiveInteger(2))
                ]
            )
        );
        Ok(())
    }

    #[test]
    fn multiple_string_and_number_array() -> TestResult {
        let value = array("[1, \"str\", 2.5e3]")?;
        assert_eq!(
            value,
            (
                "",
                vec![
                    Value::Number(Number::PositiveInteger(1)),
                    Value::String("str".to_string()),
                    Value::Number(Number::Float(2500.0))
                ]
            )
        );
        Ok(())
    }

    #[test]
    fn parse_empty_object() -> TestResult {
        let value = object("{ }")?;
        assert_eq!(value, ("", HashMap::new()));
        Ok(())
    }

    #[test]
    fn parse_a_object() -> TestResult {
        let value = object("{\"key\": 1}")?;
        let mut expected = HashMap::new();
        expected.insert("key".to_string(), Value::Number(Number::PositiveInteger(1)));

        assert_eq!(value, ("", expected));
        Ok(())
    }

    #[test]
    fn a_members() -> TestResult {
        let value = members("\"key\": 1")?;
        assert_eq!(
            value,
            (
                "",
                vec![("key".to_string(), Value::Number(Number::PositiveInteger(1)))]
            )
        );
        Ok(())
    }

    #[test]
    fn multi_members() -> TestResult {
        let value = members("\"key1\": 1, \"key2\": 2")?;
        assert_eq!(
            value,
            (
                "",
                vec![
                    (
                        "key1".to_string(),
                        Value::Number(Number::PositiveInteger(1))
                    ),
                    (
                        "key2".to_string(),
                        Value::Number(Number::PositiveInteger(2))
                    ),
                ]
            )
        );
        Ok(())
    }

    #[test]
    fn parse_object() -> TestResult {
        let value = parse("{\"title\": \"TITLE1\", \"revision\": 12}")?;
        let mut h = HashMap::new();
        h.insert("title".to_string(), Value::String("TITLE1".to_string()));
        h.insert(
            "revision".to_string(),
            Value::Number(Number::PositiveInteger(12)),
        );
        assert_eq!(value, Value::Object(h));
        Ok(())
    }
}
