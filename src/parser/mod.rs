pub mod ast;
pub mod lexer;
pub mod token;

pub use ast::{Node, Parser};
pub use lexer::Lexer;
pub use token::Token;

pub fn parse_expression(input: &str) -> Result<Node, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}
