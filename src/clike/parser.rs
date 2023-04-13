use crate::tokeniser::{Lexer, Token, Ttype};

pub enum Expr {
    INT(i64)
}
pub enum Statement {
    RETURN(Box<Expr>)
}
pub enum Program {
    FUNC(Box<Token>, Box<Token>, Vec<Statement>)
}

pub fn parse(mut lexer: Lexer) {
    let mut prog: Vec<Program> = Vec::new();
    loop {
        let mut token = lexer.lex();
        match token.2 {
            Ttype::INT => {
                let ident = lexer.lex();
                if let Ttype::IDENT(s) = ident.2 {
                    lexer.expect(Ttype::OPAR);
                    lexer.expect(Ttype::CPAR);
                    prog.push(parse_func(token, ident, &mut lexer));
                }
            }
            _ => unreachable!()
        }
    }
}

fn parse_func(ty: Token, ident: Token, lexer: &mut Lexer) -> Program {
    lexer.expect(Ttype::OBRACE);
    let mut stats: Vec<Statement> = Vec::new();
    Program::FUNC(Box::new(ty), Box::new(ident), stats)
}