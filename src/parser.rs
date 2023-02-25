use crate::lexer::*;

macro_rules! error {
  ($e:expr, $($es:expr),+) => {
    panic!("In File {}, Error at line {} position {}: {}", $e.file, $e.line, $e.pos, format!("{}", ($($es), +)))
  }
}

#[derive(Debug)]
pub enum Node {
    Immediate(LexerResult),
    Ident(LexerResult),
    Keyword(LexerResult),
    Assign(LexerResult, Vec<Node>),
}

pub struct Parser;

impl Parser {
    pub fn new(lex: &mut Lexer) -> Vec<Node> {
        let mut ret: Vec<Node> = Vec::new();
        let mut expr = true;
        loop {
            let res = lex.next();
            if res.token == Token::EOF {
                break;
            }
            match res.token {
                Token::KW_INT => ret.push(Node::Keyword(res)),
                Token::IDENT(..) => ret.push(Node::Ident(res)),
                Token::OASSIGN => {
                    let ident = match ret.pop() {
                        Some(s) => s,
                        None => error!(res, "Expected identifier before '=' operator"),
                    };
                    let lres = match ident {
                        Node::Ident(s) => s,
                        _ => error!(res, "Expected identifier before '=' operator"),
                    };
                    ret.push(Node::Assign(lres, Vec::new()));
                    expr = true;
                }
                Token::NUMBER(s) => {
                    if ret.len() < 1 {
                        error!(res, "Value of type number was not expected here");
                    }
                    if expr {
                        let node = ret.pop().unwrap();
                        match node {
                            Node::Assign(a, mut e) => {
                                e.push(Node::Immediate(res));
                                ret.push(Node::Assign(a, e));
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => {}
            }
        }
        ret
    }
    
    pub fn sem_analyse(ast: Vec<Node>) {
      
    }
}
