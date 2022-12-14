
use crate::prelude::*;
use super::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, SemiColon, Slash, Star,
  
  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,
  
  // Literals.
  Identifier, String, Number,
  
  // Keywords.
  And, Class, Else, False, Fun, Fn, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,
  
  SingleLineComment, Whitespace, Eof,
}

#[derive(Debug, Clone)]
pub enum TokenLiteral {
  String(String),
  Integer(i32),
  Float(f32),
  Bool(bool),
}

#[derive(Debug, Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub literal: Option<TokenLiteral>,
  pub line: i32,
}

impl Token {
  pub fn tokenize(source: String) -> Vec<Result<Self, Error>> {
    println!("Tokenizing source");
    let mut tokens: Vec<Result<Token, Error>> = Vec::new();
    let mut stream = source.chars().peekable();
    let mut line = 1;
    
    loop {
      if stream.clone().count() <= 0{
        println!("End of token stream");
        tokens.push(Self::try_parse(&mut stream, &mut line));
        break
      }
      tokens.push(Self::try_parse(&mut stream, &mut line));
    }
    
    tokens
  }
  
  fn try_parse<'a>(source: &mut Peekable<Chars<'a>>, mut line: &mut i32) -> Result<Self, Error> {
    use TokenType::*;
    let c = match source.next() {
      Some(v) => v,
      _ => return Ok(Token { token_type: TokenType::Eof, lexeme: '\0'.to_string(), literal: None, line: *line }),
    };

    match c {
      '(' => Ok(Token { token_type: LeftParen, lexeme: c.to_string(), literal: None, line: *line }),
      ')' => Ok(Token { token_type: RightParen, lexeme: c.to_string(), literal: None, line: *line }),
      '{' => Ok(Token { token_type: LeftBrace, lexeme: c.to_string(), literal: None, line: *line }),
      '}' => Ok(Token { token_type: RightBrace, lexeme: c.to_string(), literal: None, line: *line }),
      '.' => Ok(Token { token_type: Dot, lexeme: c.to_string(), literal: None, line: *line }),
      ',' => Ok(Token { token_type: Comma, lexeme: c.to_string(), literal: None, line: *line }),
      '-' => Ok(Token { token_type: Minus, lexeme: c.to_string(), literal: None, line: *line }),
      '+' => Ok(Token { token_type: Plus, lexeme: c.to_string(), literal: None, line: *line }),
      ';' => Ok(Token { token_type: SemiColon, lexeme: c.to_string(), literal: None, line: *line }),
      '*' => Ok(Token { token_type: Star, lexeme: c.to_string(), literal: None, line: *line }),
      '!' => {
        match source.peek() {
          Some('=') => {
            let nc = source.next().unwrap();
            Ok(Token { token_type: BangEqual, lexeme: format!("{c}{nc}").to_string(), literal: None, line: *line })
          },
          _ => Ok(Token { token_type: Bang, lexeme: c.to_string(), literal: None, line: *line })
        }
      },
      '=' => {
        match source.peek() {
          Some('=') => {
            let nc = source.next().unwrap();
            Ok(Token { token_type: EqualEqual, lexeme: format!("{c}{nc}").to_string(), literal: None, line: *line })
          },
          _ => Ok(Token { token_type: Equal, lexeme: c.to_string(), literal: None, line: *line })
        }
      },
      '<' => {
        match source.peek() {
          Some('=') => {
            let nc = source.next().unwrap();
            Ok(Token { token_type: LessEqual, lexeme: format!("{c}{nc}").to_string(), literal: None, line: *line })
          },
          _ => Ok(Token { token_type: Less, lexeme: c.to_string(), literal: None, line: *line })
        }
      },
      '>' => {
        match source.peek() {
          Some('=') => {
            let nc = source.next().unwrap();
            Ok(Token { token_type: GreaterEqual, lexeme: format!("{c}{nc}").to_string(), literal: None, line: *line })
          },
          _ => Ok(Token { token_type: Greater, lexeme: c.to_string(), literal: None, line: *line })
        }
      },
      '/' => {
        match source.peek() {
          Some('/') => {
            let mut comment = std::string::String::from(c);
            loop {
              match source.take_while(|sc| *sc != '\n').next() {
                Some(nc) => comment.push(nc),
                _ => {
                  *line += 1;
                  return Ok(Token { token_type: SingleLineComment, lexeme: comment, literal: None, line: *line-1 })
                }
              }
            }
          },
          _ => Ok(Token { token_type: Slash, lexeme: c.to_string(), literal: None, line: *line })
        }
      },
      ' ' | '\r' | '\t' => Ok(Token { token_type: Whitespace, lexeme: c.to_string(), literal: None, line: *line }),
      '\n' => {
        *line += 1;
        Ok(Token { token_type: Whitespace, lexeme: c.to_string(), literal: None, line: *line })
      },
      '"' => {
        let mut literal = std::string::String::default();
        loop {
          let c = source.next();
          match c {
            Some('"') => return Ok(Token { token_type: TokenType::String, lexeme: format!("\"{literal}\""), literal: Some(TokenLiteral::String(literal)), line: *line }),
            Some('\n') => {
              *line += 1;
              literal.push('\n');
            }
            Some(c) => literal.push(c),
            _ => return Err(Error::UnterminatedString(*line)),
          }
        }
      },
      '0'..='9' => {
        let mut literal = std::string::String::from(c);
        let mut is_decimal = false;
        loop {
          let c = source.peek();
          match c {
            Some(c) => {
              match c {
                '0'..='9' => literal.push(source.next().unwrap()),
                '.' => {
                  if is_decimal {
                    return Err(Error::UnparsableNumber(*line, "invalid float literal".to_string()))
                  }
                  is_decimal = true;
                  literal.push(source.next().unwrap());
                },
                _ => {
                  // We should be able to push on a trimmable whitespace character
                  // This should solve the double lookahead
                  literal.push(*c);
                  literal = literal.trim_end().to_string();
                  
                  if is_decimal {
                    return literal.parse::<f32>()
                      .map_err(|e| Error::UnparsableNumber(*line, e.to_string()))
                      .map(|float| Token { token_type: Number, lexeme: literal, literal: Some(TokenLiteral::Float(float)), line: *line })
                  }
                  return literal.parse::<i32>()
                    .map_err(|e| Error::UnparsableNumber(*line, e.to_string()))
                    .map(|int| Token { token_type: Number, lexeme: literal, literal: Some(TokenLiteral::Integer(int)), line: *line })
                }
              }
            },
            _ => {
              if is_decimal {
                return literal.parse::<f32>()
                  .map_err(|e| Error::UnparsableNumber(*line, e.to_string()))
                  .map(|float| Token { token_type: Number, lexeme: literal, literal: Some(TokenLiteral::Float(float)), line: *line })
              }
              return literal.parse::<i32>()
                .map_err(|e| Error::UnparsableNumber(*line, e.to_string()))
                .map(|int| Token { token_type: Number, lexeme: literal, literal: Some(TokenLiteral::Integer(int)), line: *line })
            }
          }
        }
      },
      'a'..='z' | 'A'..='Z' | '_' => {
        let mut literal = std::string::String::from(c);
        let keyword = |lit: std::string::String| -> TokenType {
          match lit.as_str() {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "for" => For,
            "fun" => Fun,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,
            _ => Identifier,
          }
        };
        loop {
          let c = source.peek();
          match c {
            Some(c) => {
              match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => literal.push(source.next().unwrap()),
                _ => {
                  let ttype = keyword(literal.clone());
                  if ttype == True || ttype == False {
                    return Ok(Token { token_type: ttype, lexeme: literal.clone(), literal: Some(TokenLiteral::Bool(literal.as_str() == "true")), line: *line })
                  }
                  return Ok(Token { token_type: ttype, lexeme: literal, literal: None, line: *line })
                },
              }
            },
            _ => {
              let ttype = keyword(literal.clone());
              if ttype == True || ttype == False {
                return Ok(Token { token_type: ttype, lexeme: literal.clone(), literal: Some(TokenLiteral::Bool(literal.as_str() == "true")), line: *line })
              }
              return Ok(Token { token_type: ttype, lexeme: literal, literal: None, line: *line })
            },
          }
        }
      }
      _ => Err(Error::UnexpectedToken(*line, c.to_string())),
    }
  }
}
