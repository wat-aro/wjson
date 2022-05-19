use nom::{bytes::complete::tag, combinator::value, IResult};

pub fn true_parser(input: &str) -> IResult<&str, bool> {
    value(true, tag("true"))(input)
}

pub fn false_parser(input: &str) -> IResult<&str, bool> {
    value(false, tag("false"))(input)
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};
    use nom::Err;

    use super::*;

    #[test]
    fn pass_true() {
        assert_eq!(true_parser("true false"), Ok((" false", true)));
    }

    #[test]
    fn failed_true() {
        assert_eq!(
            true_parser("false"),
            Err(Err::Error(Error::new("false", ErrorKind::Tag)))
        );
    }

    #[test]
    fn pass_false() {
        assert_eq!(false_parser("false true"), Ok((" true", false)));
    }

    #[test]
    fn failed_false() {
        assert_eq!(
            false_parser("true"),
            Err(Err::Error(Error::new("true", ErrorKind::Tag)))
        );
    }
}
