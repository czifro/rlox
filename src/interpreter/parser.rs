use super::{
    error::Error,
    expression::{AstPrinter, Decl, Expr, Stmt},
    token::*,
};

pub struct Parser {
    cursor: i32,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { cursor: 0, tokens }
    }

    pub fn parse(&mut self) -> Vec<Result<Decl, Error>> {
        let mut exprs: Vec<Result<Decl, Error>> = Vec::default();

        loop {
            if self.is_eof() {
                break;
            }

            let expr = self.declaration();
            if expr.is_err() {
                self.synchronize();
            }

            exprs.push(expr);
        }

        exprs
    }

    fn synchronize(&mut self) {
        use TokenType::*;
        self.advance();

        loop {
            if self.is_eof() {
                return;
            }
            if self.previous().token_type == SemiColon {
                return;
            }
            let Some(_) = self.advance_if(|t| {
                ![Class, Fun, Fn, Var, For, If, While, Print, Return].contains(&t.token_type)
            }) else {
                break;
            };
        }
    }
    fn declaration(&mut self) -> Result<Decl, Error> {
        use TokenType::*;
        let Some(_) = self.advance_if(|t| t.token_type == Var) else {
            let stmt = self.statement();
            self.advance_if(|t| t.token_type == SemiColon);
            return stmt.map(Decl::Statement);
        };

        let Some(ident) = self.advance_if(|t| t.token_type == Identifier) else {
            let tok = self.peek();
            return Error::UnexpectedToken(
                tok.line,
                format!("Found {:?}, expected {:?}", tok.token_type, Identifier),
            )
            .to_result();
        };
        let ident = ident.clone();

        let Some(_) = self.advance_if(|t| t.token_type == Equal) else {
            self.advance_if(|t| t.token_type == SemiColon);
            return Ok(Decl::Declaration(ident, None));
        };

        let expr = self.expression()?;
        self.advance_if(|t| t.token_type == SemiColon);

        Ok(Decl::Declaration(ident, Some(expr)))
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        use TokenType::*;
        match self.advance_if(|t| [Print, LeftBrace, If].contains(&t.token_type)) {
            Some(tok) if tok.token_type == LeftBrace => {
                let mut decls = vec![];
                while self.advance_if(|t| t.token_type == RightBrace).is_none() {
                    if self.is_eof() {
                        return Err(Error::UnexpectedEof(self.peek().line));
                    }
                    let decl = self.declaration()?;
                    decls.push(decl);
                }
                Ok(Stmt::Block(decls))
            }
            Some(tok) if tok.token_type == If => {
                let tok = tok.clone();
                let Some(_) = self.advance_if(|t| t.token_type == LeftParen) else {
                    return Error::WrongTokenType(
                        self.peek().line,
                        self.peek().lexeme.clone(),
                        "(".to_string(),
                    )
                    .to_result();
                };
                let expr = self.expression()?;
                let Some(_) = self.advance_if(|t| t.token_type == RightParen) else {
                    return Error::WrongTokenType(
                        self.peek().line,
                        self.peek().lexeme.clone(),
                        ")".to_string(),
                    )
                    .to_result();
                };
                let stmt = self.statement()?;
                let Some(_) = self.advance_if(|t| t.token_type == Else) else {
                    return Ok(Stmt::If(tok.clone(), expr, Box::from(stmt), None));
                };
                let else_stmt = self.statement()?;

                Ok(Stmt::If(
                    tok.clone(),
                    expr,
                    Box::from(stmt),
                    Some(Box::from(else_stmt)),
                ))
            }
            Some(tok) if tok.token_type == Print => {
                let expr = self.expression()?;
                self.advance_if(|t| t.token_type == SemiColon);
                Ok(Stmt::Print(expr))
            }
            _ => {
                let expr = self.expression()?;
                self.advance_if(|t| t.token_type == SemiColon);
                Ok(Stmt::Expression(expr))
            }
        }
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let mut expr = self.or()?;

        let Some(tok) = self.advance_if(|t| t.token_type == TokenType::Equal) else {
            return Ok(expr);
        };
        let tok = tok.clone();

        let value = self.assignment()?;

        match expr {
            Expr::Identifier(tok) => expr = Expr::Assign(tok, Box::from(value)),
            _ => {
                let mut printer = AstPrinter::default();
                let expr = expr.accept(&mut printer).unwrap();
                return Error::InvalidAssignmentTarget(tok.line, expr).to_result();
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, Error> {
        let mut expr = self.and()?;

        while let Some(tok) = self.advance_if(|t| t.token_type == TokenType::Or) {
            let tok = tok.clone();
            let right = self.and()?;
            expr = Expr::Logical(Box::from(expr), tok, Box::from(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, Error> {
        let mut expr = self.equality()?;

        while let Some(tok) = self.advance_if(|t| t.token_type == TokenType::And) {
            let tok = tok.clone();
            let right = self.equality()?;
            expr = Expr::Logical(Box::from(expr), tok, Box::from(right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        use TokenType::*;
        let mut expr = self.comparison()?;

        while let Some(tok) = self.advance_if(|t| [BangEqual, EqualEqual].contains(&t.token_type)) {
            let tok = tok.clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::from(expr), tok, Box::from(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        use TokenType::*;
        let mut expr = self.term()?;

        while let Some(tok) =
            self.advance_if(|t| [Greater, GreaterEqual, Less, LessEqual].contains(&t.token_type))
        {
            let tok = tok.clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::from(expr), tok, Box::from(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        use TokenType::*;
        let mut expr = self.factor()?;

        while let Some(tok) = self.advance_if(|t| [Plus, Minus].contains(&t.token_type)) {
            let tok = tok.clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::from(expr), tok, Box::from(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        use TokenType::*;
        let mut expr = self.unary()?;

        while let Some(tok) = self.advance_if(|t| [Star, Slash].contains(&t.token_type)) {
            let tok = tok.clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::from(expr), tok, Box::from(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        use TokenType::*;
        match self.advance_if(|t| [Minus, Bang].contains(&t.token_type)) {
            Some(tok) => {
                let tok = tok.clone();
                let right = self.unary()?;
                Ok(Expr::Unary(tok, Box::from(right)))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.is_eof() {
            return Error::UnexpectedEof(self.peek().line).to_result();
        }

        match self.peek().token_type {
            TokenType::True
            | TokenType::False
            | TokenType::Nil
            | TokenType::Integer
            | TokenType::Float
            | TokenType::String => Ok(Expr::Literal(self.advance().clone())),
            TokenType::Identifier => Ok(Expr::Identifier(self.advance().clone())),
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                match self.peek().token_type {
                    TokenType::RightParen => {
                        self.advance();
                        Ok(Expr::Grouping(Box::from(expr)))
                    }
                    _ => {
                        let token = self.peek();
                        Error::WrongTokenType(
                            token.clone().line,
                            token.clone().lexeme,
                            ")".to_string(),
                        )
                        .to_result()
                    }
                }
            }
            _ => {
                let token = self.peek();
                Error::WrongTokenType(token.clone().line, token.clone().lexeme, "(".to_string())
                    .to_result()
            }
        }
    }

    fn peek_offset(&self, offset: i32) -> &Token {
        self.tokens.get((self.cursor + offset) as usize).unwrap()
    }

    fn peek(&self) -> &Token {
        self.peek_offset(0)
    }

    fn previous(&self) -> &Token {
        self.peek_offset(-1)
    }

    fn is_eof(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn shift_cursor(&mut self, offset: i32) {
        self.cursor = self.cursor + offset;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_eof() {
            self.shift_cursor(1);
        }

        self.previous()
    }

    fn advance_if<F>(&mut self, f: F) -> Option<&Token>
    where
        F: FnOnce(&Token) -> bool,
    {
        if f(self.peek()) {
            return Some(self.advance());
        }
        None
    }
}
