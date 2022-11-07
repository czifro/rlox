
use crate::prelude::*;
use super::error::Error;
use super::token::Token;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
  source: Peekable<Chars<'a>>,
  line: i32,
}

impl<'a> Scanner<'a> {
  pub fn new(source: &'a String) -> Self {
    let source: Peekable<Chars<'a>> = source.chars().peekable();

    Self {
      source: source.clone(),
      line: 1,
    }.clone()
  }
  
  fn is_at_end(&self) -> bool {
    self.source.clone().count() <= 0
  }
  
  fn scan_token(&mut self) -> Result<Token, Error> {
    Token::try_parse(&mut self.source, &mut self.line)
  }
  
  pub fn scan_tokens(&mut self) -> Vec<Result<Token, Error>> {
    println!("Scanning tokens");
    let mut tokens: Vec<Result<Token, Error>> = Vec::new();
    
    loop {
      if self.is_at_end() {
        println!("End of token stream");
        tokens.push(self.scan_token());
        break
      }
      tokens.push(self.scan_token());
    }
    
    tokens
  }
}