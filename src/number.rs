use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::many1;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Number {
    PositiveInteger(u64),
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
/// // this will fail if number fails
/// assert_eq!(number("a"), Err(Err::Error(Error::new("a", ErrorKind::Char))));
/// # }
/// ```
pub fn number(input: &str) -> IResult<&str, Number> {
    map(integer, |u: u64| Number::PositiveInteger(u))(input)
}

/// Recognize integer
fn integer(input: &str) -> IResult<&str, u64> {
    digits(input)
}

/// Recognize digits
/// digits = digit
///        | digit digits
fn digits(input: &str) -> IResult<&str, u64> {
    let (rest, v) = many1(digit)(input)?;
    let str_vec: String = v.iter().map(|d| d.to_string()).collect::<String>();

    Ok((rest, str_vec.parse().unwrap()))
}

/// Recognize a digit
/// digit = zero
///       | onenine
fn digit(input: &str) -> IResult<&str, char> {
    alt((zero, onenine))(input)
}

/// Recognize '1' ... '9'
/// onenine = 1...9
fn onenine(input: &str) -> IResult<&str, char> {
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
    ))(input)
}

/// Recognize "0"
/// zero = 0
fn zero(input: &str) -> IResult<&str, char> {
    char('0')(input)
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};
    use nom::Err;

    use super::*;

    #[test]
    fn assert_zero() {
        assert_eq!(zero("0"), Ok(("", '0')));
    }

    #[test]
    fn failed_parse_one() {
        assert_eq!(zero("1"), Err(Err::Error(Error::new("1", ErrorKind::Char))))
    }

    #[test]
    fn parse_one() {
        assert_eq!(onenine("1"), Ok(("", '1')));
    }

    #[test]
    fn parse_nine() {
        assert_eq!(onenine("9"), Ok(("", '9')));
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
        assert_eq!(digit("0"), Ok(("", '0')));
    }

    #[test]
    fn digit_one() {
        assert_eq!(digit("1"), Ok(("", '1')));
    }

    #[test]
    fn digit_one_nine() {
        assert_eq!(digit("19"), Ok(("9", '1')));
    }

    #[test]
    fn digit_one_alpha() {
        assert_eq!(digit("1a"), Ok(("a", '1')));
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
        assert_eq!(Ok(("", 123)), digits("123"))
    }
}
