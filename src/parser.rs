use std::{
    iter::Enumerate,
    ops::{Range, RangeFrom, RangeFull, RangeTo},
};

use nom::bytes::complete::take;
use nom::combinator::verify;
use nom::{AsBytes, IResult, InputIter, InputLength, InputTake, Slice};

use crate::lexer::Token;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Nil,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Symbol(String),
    Lambda(Vec<String>, Vec<Expr>),
    List(Vec<Expr>),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tokens<'a> {
    tokens: &'a [Token],
    start: usize,
    end: usize,
}

impl<'a> Tokens<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Tokens {
            tokens,
            start: 0,
            end: tokens.len(),
        }
    }
}

impl<'a> InputLength for Tokens<'a> {
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl<'a> InputTake for Tokens<'a> {
    fn take(&self, count: usize) -> Self {
        Tokens {
            tokens: &self.tokens[0..count],
            start: 0,
            end: count,
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (first, second) = self.tokens.split_at(count);
        (
            Tokens {
                tokens: second,
                start: 0,
                end: second.len(),
            },
            Tokens {
                tokens: first,
                start: 0,
                end: first.len(),
            },
        )
    }
}

impl InputLength for Token {
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        Tokens {
            tokens: self.tokens.slice(range.clone()),
            start: self.start + range.start,
            end: self.start + range.end,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.end)
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
    fn slice(&self, range: RangeFull) -> Self {
        Tokens {
            tokens: self.tokens,
            start: self.start,
            end: self.end,
        }
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item = &'a Token;
    type Iter = Enumerate<::std::slice::Iter<'a, Token>>;
    type IterElem = ::std::slice::Iter<'a, Token>;

    fn iter_elements(&self) -> Self::IterElem {
        self.tokens.iter()
    }

    fn iter_indices(&self) -> Self::Iter {
        self.tokens.iter().enumerate()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.tokens.iter().position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        if self.tokens.len() >= count {
            Ok(count)
        } else {
            Err(nom::Needed::Unknown)
        }
    }
}

#[derive(Debug)]
pub struct CustomParserError(String);

// macro_rules! tag_token (
//     ($func_name:ident, $tag: expr) => (
//         fn $func_name(tokens: Tokens) -> IResult<Tokens, Tokens> {
//             verify(take(1usize), |x: &Tokens| x.tokens[0] == $tag)(tokens)
//         }
//     )
//   );

macro_rules! tag_token (
    ($func_name:ident, $tag: pat) => (
        fn $func_name(tokens: Tokens) -> IResult<Tokens, Tokens> {
            verify(take(1usize), |x: &Tokens| match x.tokens[0] {
                $tag => true,
                _ => false,
            })(tokens)
        }
    )
  );

tag_token!(tag_lparan, Token::LParan);
tag_token!(tag_rparan, Token::RParan);
tag_token!(tag_integer, Token::Integer(_));
tag_token!(tag_symbol, Token::Symbol(_));

// fn tag_lparan(tokens: Tokens) -> IResult<Tokens, Tokens> {
//     verify(take(1usize), |x: &Tokens| match x.tokens[0] {
//         Token::LParan => true,
//         _ => false,
//     })(tokens)
// }

// fn parse_open_paran(tokens: &Vec<Token>) -> IResult<&Vec<Token>, Expr> {
//     if let Some(fisrt) = tokens.first() {
//         match first {
//             Token::LParan => {}
//             _ => Err,
//         }
//     }
// }

// pub fn parse_list(tokens: &Vec<Token>) -> IResult<&Vec<Token>, Expr> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn take_test() {
        let tokens = vec![Token::LParan];
        assert_eq!(
            take::<usize, Tokens<'_>, nom::error::Error<Tokens<'_>>>(1usize)(Tokens::new(&tokens)),
            Ok((Tokens::new(&vec![]), Tokens::new(&vec![Token::LParan])))
        );
    }

    #[test]
    fn tag_lparan_test() {
        assert_eq!(
            tag_lparan(Tokens::new(&vec![Token::LParan, Token::RParan])).unwrap(),
            (
                Tokens::new(&vec![Token::RParan]),
                Tokens::new(&vec![Token::LParan])
            )
        );
    }

    #[test]
    fn tag_rparan_test() {
        assert_eq!(
            tag_rparan(Tokens::new(&vec![Token::RParan, Token::LParan])).unwrap(),
            (
                Tokens::new(&vec![Token::LParan]),
                Tokens::new(&vec![Token::RParan]),
            )
        );
    }

    #[test]
    fn tag_integer_test() {
        assert_eq!(
            tag_integer(Tokens::new(&vec![Token::Integer(42), Token::RParan])).unwrap(),
            (
                Tokens::new(&vec![Token::RParan]),
                Tokens::new(&vec![Token::Integer(42)]),
            )
        );
    }

    #[test]
    fn tagsymbolr_test() {
        assert_eq!(
            tag_symbol(Tokens::new(&vec![Token::Symbol("()".to_owned()), Token::RParan])).unwrap(),
            (
                Tokens::new(&vec![Token::RParan]),
                Tokens::new(&vec![Token::Symbol("()".to_owned())]),
            )
        );
    }
}
