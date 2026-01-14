use std::collections::HashMap;

use crate::token_type::{self, Literal, Token};
use crate::token_type::{TokenType, TokenType::*};

#[derive(Debug)]
pub enum ScanError {
    UnexpectedCharacter { line: usize, ch: char },
    UnterminatedString { line: usize },
}
// #[derive(Default, Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<token_type::Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), AND);
        keywords.insert("class".to_string(), CLASS);
        keywords.insert("else".to_string(), ELSE);
        keywords.insert("false".to_string(), FALSE);
        keywords.insert("for".to_string(), FOR);
        keywords.insert("fun".to_string(), FUN);
        keywords.insert("if".to_string(), IF);
        keywords.insert("nil".to_string(), NIL);
        keywords.insert("or".to_string(), OR);
        keywords.insert("print".to_string(), PRINT);
        keywords.insert("return".to_string(), RETURN);
        keywords.insert("super".to_string(), SUPER);
        keywords.insert("this".to_string(), THIS);
        keywords.insert("true".to_string(), TRUE);
        keywords.insert("var".to_string(), VAR);
        keywords.insert("while".to_string(), WHILE);
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: keywords,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        let c = self.source.as_bytes()[self.current - 1 as usize] as char;
        c
    }

    fn add_token_without_literal(&mut self, typ: TokenType) {
        self.add_token_with_literal(typ, None)
    }

    fn add_token_with_literal(
        &mut self,
        typ: token_type::TokenType,
        literal: Option<token_type::Literal>,
    ) {
        let text = self.source[self.start as usize..self.current as usize].to_string();
        self.tokens.push(Token::new(typ, text, self.line, literal));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source.as_bytes()[self.current] as char != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }

    fn is_digit(&self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if ch >= '0' && ch <= '9' {
            return true;
        } else {
            return false;
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.as_bytes()[self.current + 1] as char;
    }

    fn is_alpha(&self, ch: char) -> bool {
        if self.is_at_end() {
            false
        } else if ch <= 'z' && ch >= 'a' || ch <= 'Z' && ch >= 'A' || ch == '_' {
            true
        } else {
            false
        }
    }

    fn is_alphanumeric(&self, ch: char) -> bool {
        if self.is_alpha(ch) || self.is_digit(ch) {
            true
        } else {
            false
        }
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let typ = self.keywords.get(&text);
        match typ {
            Some(val) => {
                self.add_token_without_literal(val.clone());
            }
            None => {
                self.add_token_without_literal(IDENTIFIER);
            }
        }
    }

    fn string(&mut self) -> Result<(), ScanError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScanError::UnterminatedString { line: self.line });
        }

        self.advance();
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(STRING, Some(Literal::String(value)));
        return Ok(());
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let num_str = &self.source[self.start..self.current];
        if let Ok(val) = num_str.parse::<f64>() {
            self.add_token_with_literal(NUMBER, Some(Literal::Number(val)));
        }
    }

    fn scan_token(&mut self, errors: &mut Vec<ScanError>) {
        if !self.source.is_empty() {
            let c = self.advance();
            match c {
                '(' => {
                    self.add_token_without_literal(LEFT_PAREN);
                }
                ')' => {
                    self.add_token_without_literal(RIGHT_PAREN);
                }
                '{' => {
                    self.add_token_without_literal(LEFT_BRACE);
                }
                '}' => {
                    self.add_token_without_literal(RIGHT_BRACE);
                }
                ',' => {
                    self.add_token_without_literal(COMMA);
                }
                '.' => {
                    self.add_token_without_literal(DOT);
                }
                '+' => {
                    self.add_token_without_literal(PLUS);
                }
                '-' => {
                    self.add_token_without_literal(MINUS);
                }
                '*' => {
                    self.add_token_without_literal(STAR);
                }
                ';' => {
                    self.add_token_without_literal(SEMICOLON);
                }
                '=' => {
                    let token_typ = if self.match_char('=') {
                        EQUAL_EQUAL
                    } else {
                        EQUAL
                    };
                    self.add_token_without_literal(token_typ);
                }
                '!' => {
                    let token_typ = if self.match_char('=') {
                        BANG_EQUAL
                    } else {
                        BANG
                    };
                    self.add_token_without_literal(token_typ);
                }
                '<' => {
                    let token_typ = if self.match_char('=') {
                        LESS_EQUAL
                    } else {
                        LESS
                    };
                    self.add_token_without_literal(token_typ);
                }
                '>' => {
                    let token_typ = if self.match_char('=') {
                        GREATER_EQUAL
                    } else {
                        GREATER
                    };
                    self.add_token_without_literal(token_typ);
                }
                '"' => match self.string() {
                    Ok(val) => {}
                    Err(err) => errors.push(err),
                },
                ' ' | '\r' | '\t' => {}
                '\n' => {
                    self.line += 1;
                }
                '/' => {
                    if self.match_char('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token_without_literal(SLASH);
                    }
                }
                p if self.is_digit(p)=>{self.number();}
                p if self.is_alpha(p)=>{self.identifier();}
                _ => {
                        errors.push(ScanError::UnexpectedCharacter {
                            line: self.line,
                            ch: c,
                        })
                }
            }
        } else {
            println!("EOF  null");
        }
    }

    pub fn scan_tokens(&mut self) -> (Vec<Token>, Vec<ScanError>) {
        let mut errors = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(&mut errors);
        }

        self.tokens
            .push(token_type::Token::new(EOF, "".to_string(), self.line, None));
        (std::mem::take(&mut self.tokens), errors)
    }
}
