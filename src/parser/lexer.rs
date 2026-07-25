use super::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(&(_idx, ch)) = self.chars.peek() {
            if ch.is_whitespace() {
                self.chars.next();
                continue;
            }

            if ch.is_ascii_digit() || ch == '.' {
                let num = self.read_number()?;
                tokens.push(Token::Number(num));
                continue;
            }

            if ch == '\\' {
                self.chars.next(); // consume '\\'
                if let Some(&(_, '\\')) = self.chars.peek() {
                    self.chars.next();
                    tokens.push(Token::DoubleBackslash);
                } else {
                    let cmd = self.read_identifier()?;
                    tokens.push(Token::LaTeXCommand(cmd));
                }
                continue;
            }

            if ch.is_alphabetic() {
                let ident = self.read_identifier()?;
                tokens.push(Token::Identifier(ident));
                continue;
            }

            match ch {
                '+' | '-' | '*' | '/' | '%' => {
                    self.chars.next();
                    tokens.push(Token::Op(ch));
                }
                '×' => {
                    self.chars.next();
                    tokens.push(Token::Op('*'));
                }
                '÷' => {
                    self.chars.next();
                    tokens.push(Token::Op('/'));
                }
                '√' => {
                    self.chars.next();
                    tokens.push(Token::Identifier("sqrt".to_string()));
                }
                '^' => {
                    self.chars.next();
                    tokens.push(Token::Power);
                }
                '!' => {
                    self.chars.next();
                    tokens.push(Token::Factorial);
                }
                '(' => {
                    self.chars.next();
                    tokens.push(Token::LParen);
                }
                ')' => {
                    self.chars.next();
                    tokens.push(Token::RParen);
                }
                '[' => {
                    self.chars.next();
                    tokens.push(Token::LBracket);
                }
                ']' => {
                    self.chars.next();
                    tokens.push(Token::RBracket);
                }
                '{' => {
                    self.chars.next();
                    tokens.push(Token::LBrace);
                }
                '}' => {
                    self.chars.next();
                    tokens.push(Token::RBrace);
                }
                ',' => {
                    self.chars.next();
                    tokens.push(Token::Comma);
                }
                ';' => {
                    self.chars.next();
                    tokens.push(Token::Semicolon);
                }
                '&' => {
                    self.chars.next();
                    tokens.push(Token::Ampersand);
                }
                _ => {
                    return Err(format!("Unexpected character: '{}'", ch));
                }
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn read_number(&mut self) -> Result<f64, String> {
        let start_idx = self.chars.peek().unwrap().0;
        let mut end_idx = start_idx;
        let mut has_dot = false;

        while let Some(&(idx, ch)) = self.chars.peek() {
            if ch.is_ascii_digit() {
                end_idx = idx + ch.len_utf8();
                self.chars.next();
            } else if ch == '.' {
                if has_dot {
                    break;
                }
                has_dot = true;
                end_idx = idx + ch.len_utf8();
                self.chars.next();
            } else {
                break;
            }
        }

        let slice = &self.input[start_idx..end_idx];
        slice
            .parse::<f64>()
            .map_err(|_| format!("Invalid number: '{}'", slice))
    }

    fn read_identifier(&mut self) -> Result<String, String> {
        let mut ident = String::new();
        while let Some(&(_, ch)) = self.chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }
        Ok(ident)
    }
}
