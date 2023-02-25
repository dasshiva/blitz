use crate::lexer;

pub enum Node {
  Immediate(lexer::LexerResult),
  Keyword(lexer::LexerResult)
}

type Parser = Vec<Node>;

impl Parser {
  
}