use std::process::Output;
use std::slice::Iter;

use combine::*;
use combine::parser::function;
use combine::{
  parser::{
    choice,
    token::{
      satisfy,
      satisfy_map,
    },
  },
  stream::*,
};

use crate::interpreter::error::Error;
use crate::interpreter::token::*;
use crate::prelude::*;

use super::*;

pub type TokenStream<'a> = SliceStream<'a, Token>;

pub fn expression(tokens: Vec<Token>) {
  let tokens = tokens.as_slice();
  
}

fn whitespace_parser<'a>() -> impl Parser<TokenStream<'a>> {
  satisfy::<TokenStream<'a>, _>(|t| t.token_type == TokenType::Whitespace)
}

fn single_line_comment_parser<'a>() -> impl Parser<TokenStream<'a>> {
  satisfy::<TokenStream<'a>, _>(|t| t.token_type == TokenType::SingleLineComment)
}

enum UnaryTokenTree {
  Unary(Token, Box<UnaryTokenTree>),
  Primary(Token),
}

fn unary_parser<'a>() -> impl Parser<TokenStream<'a>, Output = UnaryTokenTree> {
  (
    satisfy::<TokenStream<'a>, _>(|token| token.token_type == TokenType::Bang)
      .or(satisfy::<TokenStream<'a>, _>(|token| token.token_type == TokenType::Minus)),
    function::parser::<TokenStream<'a>, UnaryTokenTree, _>(|input| {
      let _: &mut TokenStream<'a> = input;
      
      unary_parser::<'a>()
        .or(primary_parser::<'a>().map(|t| UnaryTokenTree::Primary(t)))
        .parse_stream(input)
        .into_result()
    })
  )
    .map(|(token, tree)| UnaryTokenTree::Unary(token.clone(), Box::from(tree)))
}

fn primary_parser<'a>() -> impl Parser<TokenStream<'a>, Output = Token> {
  satisfy::<TokenStream<'a>, _>(|token| {
    match token.token_type {
      TokenType::String | TokenType::Integer | TokenType::Float |
      TokenType::True | TokenType::False | TokenType::Nil => true,
      _ => false,
    }
  })
    // .or(())
    .map(|t| t.clone())
}
