use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{self, alphanumeric1, char, i64, multispace0},
    combinator::{all_consuming, value},
    error::{Error, ErrorKind},
    multi::{many0, many1},
    sequence::{preceded, terminated},
    Err, IResult,
};

pub fn do_nothing(i: &str) -> IResult<&str, &str> {
    Ok((i, ""))
}

pub fn tag_add(i: &str) -> IResult<&str, &str> {
    tag("add")(i)
}

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    LParan,
    RParan,
    Integer(i64),
    // Float(f64),
    Symbol(String),
}

fn lex_lparan(input: &str) -> IResult<&str, Token> {
    value(Token::LParan {}, preceded(multispace0, char('(')))(input)
}

fn lex_rparan(input: &str) -> IResult<&str, Token> {
    value(Token::RParan {}, preceded(multispace0, char(')')))(input)
}

fn lex_integer(input: &str) -> IResult<&str, Token> {
    let (rem, int) = preceded(multispace0, i64)(input)?;
    Ok((rem, Token::Integer(int)))
}

// pub fn parse_float(input: &str) -> IResult<&str, Token> {
//     let (rem, float) = preceded(multispace0, f64)(input)?;
//     Ok((rem, Token::Float(float)))
// }

fn lex_symbol(input: &str) -> IResult<&str, Token> {
    let (input, matched) = preceded(
        multispace0,
        take_while1(|x: char| x.is_alphanumeric() || x == '_'),
    )(input)?;
    Ok((input, Token::Symbol(matched.to_owned())))
}

pub fn lexer(input: &str) -> IResult<&str, Vec<Token>> {
    all_consuming(many0(alt((
        lex_lparan,
        lex_rparan,
        lex_integer,
        lex_symbol,
    ))))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_nothing_test() {
        assert_eq!(do_nothing("hi").unwrap(), ("hi", ""));
    }

    #[test]
    fn tag_add_test() {
        assert_eq!(tag_add("add2").unwrap(), ("2", "add"));
    }

    #[test]
    fn lex_lparan_test() {
        assert_eq!(lex_lparan(" (").unwrap(), ("", Token::LParan));
    }

    #[test]
    fn lex_rparan_test() {
        assert_eq!(lex_rparan("  ) ").unwrap(), (" ", Token::RParan));
    }

    #[test]
    fn lex_integer_test() {
        assert_eq!(lex_integer(" 42 ").unwrap(), (" ", Token::Integer(42)));
    }

    // #[test]
    // fn parse_float_test() {
    //     assert_eq!(parse_float(" 42. ").unwrap(), (" ", Token::Float(42.)));
    // }

    #[test]
    fn lex_symbol_test() {
        assert_eq!(
            lex_symbol(" some_name = 42").unwrap(),
            (" = 42", Token::Symbol("some_name".to_owned()))
        );
    }

    #[test]
    fn lexer_test() {
        assert_eq!(
            lexer("(some_name 42)").unwrap().1,
            vec![
                Token::LParan,
                Token::Symbol("some_name".to_owned()),
                Token::Integer(42),
                Token::RParan
            ]
        )
    }
}
