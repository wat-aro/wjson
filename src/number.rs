use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, recognize};
use nom::sequence::{pair, tuple};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Number {
    PositiveInteger(u64),
    NegativeInteger(i64),
}

/// Recognize digits
///
/// ```rust
/// use nom::error::{ErrorKind, Error};
/// use nom::Err;
/// use wjson::number::{number, Number};
/// # fn main() {
///
///
/// // the parser will parse "3"
/// assert_eq!(number("3"), Ok(("", Number::PositiveInteger(3))));
///
/// // the parser will parse "32"
/// assert_eq!(number("32"), Ok(("", Number::PositiveInteger(32))));
///
/// // the parser will parse "-32"
/// assert_eq!(number("-32"), Ok(("", Number::NegativeInteger(-32))));
///
/// // this will fail if number fails
/// assert_eq!(number("a"), Err(Err::Error(Error::new("a", ErrorKind::Char))));
/// # }
/// ```
pub fn number(input: &str) -> IResult<&str, Number> {
    integer(input)
}

/// Recognize integer
/// integer = digit
///         | onenine digits
///         | '-' digit
///         | '-' onenine digits
fn integer(input: &str) -> IResult<&str, Number> {
    alt((
        map(
            alt((
                recognize(tuple((char('-'), onenine, digits))),
                recognize(pair(char('-'), digit)),
            )),
            |str| Number::NegativeInteger(str.parse::<i64>().unwrap()),
        ),
        map(
            alt((
                map(recognize(pair(onenine, digits)), |str| str.to_string()),
                digit,
            )),
            |str| Number::PositiveInteger(str.parse::<u64>().unwrap()),
        ),
    ))(input)
}

/// Recognize digits
/// digits = digit
///        | digit digits
fn digits(input: &str) -> IResult<&str, String> {
    alt((
        map(recognize(pair(digit, digits)), |str| str.to_string()),
        digit,
    ))(input)
    // alt((recognize(pair(digit, digits)), digit))(input)
}

/// Recognize a digit
/// digit = zero
///       | onenine
fn digit(input: &str) -> IResult<&str, String> {
    alt((zero, onenine))(input)
}

/// Recognize '1' ... '9'
/// onenine = 1...9
fn onenine(input: &str) -> IResult<&str, String> {
    map(
        alt((
            char('1'),
            char('2'),
            char('3'),
            char('4'),
            char('5'),
            char('6'),
            char('7'),
            char('8'),
            char('9'),
        )),
        |c| c.to_string(),
    )(input)
}

/// Recognize "0"
/// zero = 0
fn zero(input: &str) -> IResult<&str, String> {
    map(char('0'), |c| c.to_string())(input)
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};
    use nom::Err;

    use super::*;

    #[test]
    fn assert_zero() {
        assert_eq!(zero("0"), Ok(("", "0".to_string())));
    }

    #[test]
    fn failed_parse_one() {
        assert_eq!(zero("1"), Err(Err::Error(Error::new("1", ErrorKind::Char))))
    }

    #[test]
    fn parse_one() {
        assert_eq!(onenine("1"), Ok(("", "1".to_string())));
    }

    #[test]
    fn parse_nine() {
        assert_eq!(onenine("9"), Ok(("", "9".to_string())));
    }

    #[test]
    fn failed_parse_a() {
        assert_eq!(
            digit("a"),
            Err(Err::Error(Error::new("a", ErrorKind::Char)))
        )
    }

    #[test]
    fn digit_zero() {
        assert_eq!(digit("0"), Ok(("", "0".to_string())));
    }

    #[test]
    fn digit_one() {
        assert_eq!(digit("1"), Ok(("", "1".to_string())));
    }

    #[test]
    fn digit_one_nine() {
        assert_eq!(digit("19"), Ok(("9", "1".to_string())));
    }

    #[test]
    fn digit_one_alpha() {
        assert_eq!(digit("1a"), Ok(("a", "1".to_string())));
    }

    #[test]
    fn digit_alpha() {
        assert_eq!(
            digit("a"),
            Err(Err::Error(Error::new("a", ErrorKind::Char)))
        );
    }

    #[test]
    fn digits1() {
        assert_eq!(digits("123"), Ok(("", "123".to_string())))
    }

    #[test]
    fn parse_negative_digit() {
        assert_eq!(integer("-1"), Ok(("", Number::NegativeInteger(-1))));
    }

    #[test]
    fn parse_negative_digits() {
        assert_eq!(integer("-123"), Ok(("", Number::NegativeInteger(-123))));
    }
}
