pub mod number;

use number::number;
use number::Number;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(Number),
}

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
