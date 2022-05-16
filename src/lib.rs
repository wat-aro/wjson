use nom::{character::complete::digit1, combinator::map_res, IResult};

pub fn integer(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};
    use nom::Err;

    use super::*;

    #[test]
    fn correct_integer() {
        assert_eq!(integer("123"), Ok(("", 123)));
    }

    #[test]
    fn incorrect_integer() {
        assert_eq!(
            integer("a"),
            Err(Err::Error(Error::new("a", ErrorKind::Digit)))
        );
    }
}
