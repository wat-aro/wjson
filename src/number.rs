use nom::branch::alt;
use nom::character::complete::{char, one_of};
use nom::combinator::{map, opt, recognize};
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Number {
    PositiveInteger(u64),
    NegativeInteger(i64),
    Float(f64),
}

#[derive(Debug)]
struct Num {
    integer: Integer,
    fraction: Option<String>,
}

impl Into<Number> for Num {
    fn into(self) -> Number {
        match (self.integer, self.fraction) {
            (Integer::Positive(str), None) => Number::PositiveInteger(str.parse::<u64>().unwrap()),
            (Integer::Negative(str), None) => Number::NegativeInteger(str.parse::<i64>().unwrap()),
            (int, Some(decimal)) => Number::Float(
                format!("{}.{}", int.to_string(), decimal)
                    .parse::<f64>()
                    .unwrap(),
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Integer {
    Positive(String),
    Negative(String),
}

impl ToString for Integer {
    fn to_string(&self) -> String {
        match self {
            Self::Positive(str) => str.to_string(),
            Self::Negative(str) => str.to_string(),
        }
    }
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
/// // parser will parse "3.21"
/// assert_eq!(number("3.21"), Ok(("", Number::Float(3.21))));
///
/// // parser will parse "-3.21"
/// assert_eq!(number("-3.21"), Ok(("", Number::Float(-3.21))));
///
/// // this will fail if number fails
/// assert_eq!(number("a"), Err(Err::Error(Error::new("a", ErrorKind::OneOf))));
/// # }
/// ```
// number = integer fraction
pub fn number(input: &str) -> IResult<&str, Number> {
    let (rest, integer) = integer(input)?;
    let (rest, fraction) = fraction(rest)?;
    let num = Num { integer, fraction };

    Ok((rest, num.into()))
}

/// Recognize integer
/// integer = digit
///         | onenine digits
///         | '-' digit
///         | '-' onenine digits
fn integer(input: &str) -> IResult<&str, Integer> {
    alt((
        map(
            alt((
                recognize(tuple((char('-'), onenine, digits))),
                recognize(pair(char('-'), digit)),
            )),
            |str| Integer::Negative(str.to_string()),
        ),
        map(
            alt((
                map(recognize(pair(onenine, digits)), |str| str.to_string()),
                digit,
            )),
            |str| Integer::Positive(str.to_string()),
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
    map(one_of("123456789"), |c| c.to_string())(input)
}

/// Recognize "0"
/// zero = 0
fn zero(input: &str) -> IResult<&str, String> {
    map(char('0'), |c| c.to_string())(input)
}

/// graction = ""
///          | "." digits
fn fraction(input: &str) -> IResult<&str, Option<String>> {
    opt(preceded(char('.'), digits))(input)
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
            Err(Err::Error(Error::new("a", ErrorKind::OneOf)))
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
            Err(Err::Error(Error::new("a", ErrorKind::OneOf)))
        );
    }

    #[test]
    fn digits1() {
        assert_eq!(digits("123"), Ok(("", "123".to_string())))
    }

    #[test]
    fn parse_negative_digit() {
        assert_eq!(integer("-1"), Ok(("", Integer::Negative("-1".to_string()))));
    }

    #[test]
    fn parse_negative_digits() {
        assert_eq!(
            integer("-123"),
            Ok(("", Integer::Negative("-123".to_string())))
        );
    }

    #[test]
    fn empty_fraction() {
        assert_eq!(fraction(""), Ok(("", None)))
    }

    #[test]
    fn rest_fraction() {
        assert_eq!(fraction(".123"), Ok(("", Some("123".to_string()))))
    }
}
