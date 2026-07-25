use super::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Number(f64),
    Variable(String),
    BinaryOp {
        op: char,
        left: Box<Node>,
        right: Box<Node>,
    },
    UnaryOp {
        op: char,
        expr: Box<Node>,
    },
    FunctionCall {
        name: String,
        args: Vec<Node>,
    },
    Fraction {
        numerator: Box<Node>,
        denominator: Box<Node>,
    },
    Matrix(Vec<Vec<Node>>),
    Vector(Vec<Node>),
    Factorial(Box<Node>),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        let token = self.advance();
        if token == expected {
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, token))
        }
    }

    pub fn parse(&mut self) -> Result<Node, String> {
        let node = self.parse_expr(0)?;
        if self.peek() != &Token::Eof {
            return Err(format!("Unexpected trailing token: {:?}", self.peek()));
        }
        Ok(node)
    }

    fn parse_expr(&mut self, min_bp: u8) -> Result<Node, String> {
        let mut lhs = self.parse_prefix()?;

        loop {
            let op_info = match self.peek() {
                Token::Op(c) => match c {
                    '+' | '-' => Some((*c, 1, 2, false)),
                    '*' | '/' | '%' => Some((*c, 3, 4, false)),
                    _ => None,
                },
                Token::Power => Some(('^', 5, 6, true)),
                Token::Factorial => {
                    self.advance();
                    lhs = Node::Factorial(Box::new(lhs));
                    continue;
                }
                _ => None,
            };

            if let Some((op, l_bp, r_bp, right_assoc)) = op_info {
                if l_bp < min_bp {
                    break;
                }
                self.advance();
                let next_bp = if right_assoc { r_bp - 1 } else { r_bp };
                let rhs = self.parse_expr(next_bp)?;
                lhs = Node::BinaryOp {
                    op,
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                };
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Node, String> {
        match self.peek().clone() {
            Token::Number(val) => {
                self.advance();
                Ok(Node::Number(val))
            }
            Token::Identifier(name) => {
                self.advance();
                if self.peek() == &Token::LParen {
                    self.advance(); // consume '('
                    let mut args = Vec::new();
                    if self.peek() != &Token::RParen {
                        loop {
                            args.push(self.parse_expr(0)?);
                            if self.peek() == &Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::RParen)?;
                    Ok(Node::FunctionCall { name, args })
                } else {
                    Ok(Node::Variable(name))
                }
            }
            Token::LaTeXCommand(cmd) => {
                self.advance();
                self.parse_latex_command(&cmd)
            }
            Token::Op('-') => {
                self.advance();
                let expr = self.parse_expr(5)?;
                Ok(Node::UnaryOp {
                    op: '-',
                    expr: Box::new(expr),
                })
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr(0)?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::LBracket => {
                self.advance();
                self.parse_matrix_or_vector()
            }
            token => Err(format!("Unexpected token in prefix: {:?}", token)),
        }
    }

    fn parse_latex_command(&mut self, cmd: &str) -> Result<Node, String> {
        match cmd {
            "frac" => {
                self.expect(Token::LBrace)?;
                let num = self.parse_expr(0)?;
                self.expect(Token::RBrace)?;
                self.expect(Token::LBrace)?;
                let den = self.parse_expr(0)?;
                self.expect(Token::RBrace)?;
                Ok(Node::Fraction {
                    numerator: Box::new(num),
                    denominator: Box::new(den),
                })
            }
            "sqrt" => {
                self.expect(Token::LBrace)?;
                let expr = self.parse_expr(0)?;
                self.expect(Token::RBrace)?;
                Ok(Node::FunctionCall {
                    name: "sqrt".to_string(),
                    args: vec![expr],
                })
            }
            "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sinh" | "cosh" | "tanh" | "ln" | "log" => {
                let name = cmd.to_string();
                if self.peek() == &Token::LParen || self.peek() == &Token::LBrace {
                    let is_brace = self.peek() == &Token::LBrace;
                    self.advance();
                    let arg = self.parse_expr(0)?;
                    if is_brace {
                        self.expect(Token::RBrace)?;
                    } else {
                        self.expect(Token::RParen)?;
                    }
                    Ok(Node::FunctionCall { name, args: vec![arg] })
                } else {
                    let arg = self.parse_prefix()?;
                    Ok(Node::FunctionCall { name, args: vec![arg] })
                }
            }
            "pi" => Ok(Node::Variable("pi".to_string())),
            _ => Err(format!("Unknown LaTeX command: \\{}", cmd)),
        }
    }

    fn parse_matrix_or_vector(&mut self) -> Result<Node, String> {
        // Look ahead to check if nested matrix [[1,2],[3,4]] or vector [1,2,3]
        if self.peek() == &Token::LBracket {
            // Nested matrix
            let mut rows = Vec::new();
            while self.peek() == &Token::LBracket {
                self.advance(); // consume inner '['
                let mut row = Vec::new();
                loop {
                    row.push(self.parse_expr(0)?);
                    if self.peek() == &Token::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(Token::RBracket)?;
                rows.push(row);
                if self.peek() == &Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
            self.expect(Token::RBracket)?;
            Ok(Node::Matrix(rows))
        } else {
            // Vector or inline row [1, 2, 3] or [1, 2; 3, 4]
            let mut first_row = Vec::new();
            let mut rows = Vec::new();
            let mut is_matrix = false;

            while self.peek() != &Token::RBracket && self.peek() != &Token::Eof {
                first_row.push(self.parse_expr(0)?);
                if self.peek() == &Token::Comma {
                    self.advance();
                } else if self.peek() == &Token::Semicolon {
                    self.advance();
                    is_matrix = true;
                    rows.push(first_row);
                    first_row = Vec::new();
                } else {
                    break;
                }
            }
            self.expect(Token::RBracket)?;

            if is_matrix {
                if !first_row.is_empty() {
                    rows.push(first_row);
                }
                Ok(Node::Matrix(rows))
            } else {
                Ok(Node::Vector(first_row))
            }
        }
    }
}
