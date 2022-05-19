use nom::{bytes::complete::tag, combinator::value, IResult};

/// Recognize null
///
/// ```rust
/// use nom::error::{ErrorKind, Error};
/// use nom::Err;
/// use wjson::null::{null, Null};
/// # fn main() {
///
///
/// // the parser will parse "null"
/// assert_eq!(null("null"), Ok(("", Null)));
///
/// // this will fail
/// assert_eq!(null("a"), Err(Err::Error(Error::new("a", ErrorKind::Tag))));
/// # }
/// ```
// null = "null"
pub fn null(input: &str) -> IResult<&str, Null> {
    value(Null, tag("null"))(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Null;
