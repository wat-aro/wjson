use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::many1;
use nom::IResult;

fn zero(input: &str) -> IResult<&str, u64> {
    map(char('0'), |c| c.to_string().parse::<u64>().unwrap())(input)
}

fn onenine(input: &str) -> IResult<&str, u64> {
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
        |c| c.to_string().parse::<u64>().unwrap(),
    )(input)
}

pub fn digit(input: &str) -> IResult<&str, u64> {
    alt((zero, onenine))(input)
}

pub fn digits(input: &str) -> IResult<&str, u64> {
    let (rest, v) = many1(digit)(input)?;
    let str_vec: String = v.iter().map(|d| d.to_string()).collect::<String>();

    Ok((rest, str_vec.parse().unwrap()))
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};
    use nom::Err;

    use super::*;

    #[test]
    fn parse_zero() {
        assert_eq!(zero("0"), Ok(("", 0)));
    }

    #[test]
    fn failed_parse_one() {
        assert_eq!(zero("1"), Err(Err::Error(Error::new("1", ErrorKind::Char))))
    }

    #[test]
    fn parse_one() {
        assert_eq!(onenine("1"), Ok(("", 1)));
    }

    #[test]
    fn parse_nine() {
        assert_eq!(onenine("9"), Ok(("", 9)));
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
        assert_eq!(digit("0"), Ok(("", 0)));
    }

    #[test]
    fn digit_one() {
        assert_eq!(digit("1"), Ok(("", 1)));
    }

    #[test]
    fn digit_one_nine() {
        assert_eq!(digit("19"), Ok(("9", 1)));
    }

    #[test]
    fn digit_one_alpha() {
        assert_eq!(digit("1a"), Ok(("a", 1)));
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
        assert_eq!(digits("123"), Ok(("", 123)))
    }
}
