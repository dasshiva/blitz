use crate::file::Handle;
use crate::r#proc::{line_split, Token};
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    NOP = 0,
    MOV = 1,
    ADD = 2,
    SUB = 3,
    MUL = 4,
    DIV = 5,
    MOD = 6,
    OR = 7,
    AND = 8,
    XOR = 9,
    SHL = 10,
    SHR = 11,
    JMP = 12,
    JE = 13,
    JNE = 14,
    JGE = 15,
    JGT = 16,
    JLE = 17,
    JLT = 18,
    CALL = 19,
    FMOV = 20,
    FADD = 21,
    FSUB = 22,
    FMUL = 23,
    FDIV = 24,
    FMOD = 25,
    INC = 26,
    DEC = 27,
    FINC = 28,
    FDEC = 29,
    SET = 30,
    CLEAR = 31,
    FPUSH = 32,
    FPOP = 33,
    PUSH = 34,
    POP = 35,
    LEA = 36,
    RET = 37,
    CMP = 38,
    FCMP = 39,
}

impl Instruction {
    pub fn new(name: String) -> Result<(Self, usize), &'static str> {
        match name.as_ref() {
            "nop" | "NOP" => Ok((Instruction::NOP, 1)),
            "mov" | "MOV" => Ok((Instruction::MOV, 3)),
            "add" | "ADD" => Ok((Instruction::ADD, 4)),
            "sub" | "SUB" => Ok((Instruction::SUB, 4)),
            "mul" | "MUL" => Ok((Instruction::MUL, 4)),
            "div" | "DIV" => Ok((Instruction::DIV, 4)),
            "mod" | "MOD" => Ok((Instruction::MOD, 4)),
            "or" | "OR" => Ok((Instruction::OR, 4)),
            "and" | "AND" => Ok((Instruction::AND, 4)),
            "xor" | "XOR" => Ok((Instruction::XOR, 4)),
            "shl" | "SHL" => Ok((Instruction::SHL, 4)),
            "shr" | "SHR" => Ok((Instruction::SHR, 4)),
            "jmp" | "JMP" => Ok((Instruction::JMP, 2)),
            "je" | "JE" => Ok((Instruction::JE, 2)),
            "jne" | "JNE" => Ok((Instruction::JNE, 2)),
            "jge" | "JGE" => Ok((Instruction::JGE, 2)),
            "jgt" | "JGT" => Ok((Instruction::JGT, 2)),
            "jle" | "JLE" => Ok((Instruction::JLE, 2)),
            "jlt" | "JLT" => Ok((Instruction::JLT, 2)),
            "call" | "CALL" => Ok((Instruction::CALL, 2)),
            "fmov" | "FMOV" => Ok((Instruction::FMOV, 3)),
            "fadd" | "FADD" => Ok((Instruction::FADD, 4)),
            "fsub" | "FSUB" => Ok((Instruction::FSUB, 4)),
            "fmul" | "FMUL" => Ok((Instruction::FMUL, 4)),
            "fdiv" | "FDIV" => Ok((Instruction::FDIV, 4)),
            "fmod" | "FMOD" => Ok((Instruction::FMOD, 4)),
            "inc" | "INC" => Ok((Instruction::INC, 2)),
            "dec" | "DEC" => Ok((Instruction::DEC, 2)),
            "finc" | "FINC" => Ok((Instruction::FINC, 2)),
            "fdec" | "FDEC" => Ok((Instruction::FDEC, 2)),
            "set" | "SET" => Ok((Instruction::SET, 3)),
            "clear" | "CLEAR" => Ok((Instruction::CLEAR, 3)),
            "fpush" | "FPUSH" => Ok((Instruction::FPUSH, 2)),
            "fpop" | "FPOP" => Ok((Instruction::FPOP, 2)),
            "push" | "PUSH" => Ok((Instruction::PUSH, 2)),
            "pop" | "POP" => Ok((Instruction::POP, 2)),
            "lea" | "LEA" => Ok((Instruction::LEA, 3)),
            "ret" | "RET" => Ok((Instruction::RET, 1)),
            "cmp" | "CMP" => Ok((Instruction::CMP, 3)),
            "fcmp" | "FCMP" => Ok((Instruction::FCMP, 3)),
            _ => Err("Invalid instruction"),
        }
    }

    pub fn is_farg_nreg(&self) -> bool {
        match self {
            Instruction::JMP
            | Instruction::JE
            | Instruction::JNE
            | Instruction::JLT
            | Instruction::JLE
            | Instruction::JGT
            | Instruction::JGE
            | Instruction::CALL
            | Instruction::PUSH
            | Instruction::FPUSH
            | Instruction::MOV
            | Instruction::INC
            | Instruction::DEC => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Args {
    INT(i64),
    DECIMAL(f64),
    STRING(String),
    REGISTER(u8),
    OFFSET(u8, i64),
}

impl Args {
    pub fn new(token: &Token, defines: &Vec<Define>) -> Result<Self, &'static str> {
        match token {
            Token::IDENT(s) => {
                let id = match u8::from_str_radix(&s[1..], 10) {
                    Ok(s) => s,
                    Err(..) => 127,
                };
                if id >= 20 && id != 127 {
                    return Err("Invalid register number");
                }
                if id != 127 {
                    if s.starts_with('r') {
                        return Ok(Args::REGISTER(id + 60));
                    } else if s.starts_with('b') || s.starts_with('f') {
                        return Ok(Args::REGISTER(id));
                    } else if s.starts_with('w') {
                        return Ok(Args::REGISTER(id + 20));
                    } else if s.starts_with('d') {
                        return Ok(Args::REGISTER(id + 40));
                    } else if s == "sp" {
                        return Ok(Args::REGISTER(80));
                    }
                }
                for def in defines {
                    if s == &def.0 {
                        return Args::new(&def.1, defines);
                    }
                }
                return Ok(Args::STRING(s.to_string()));
            }
            Token::OFFSET(s) => {
                let mut arg = s.to_string();
                arg.push(' ');
                let split = match line_split(&arg.as_bytes()) {
                    Ok(s) => s,
                    Err(..) => unreachable!(),
                };
                let reg;
                match &split[0] {
                    Token::IDENT(..) => match Args::new(&split[0], defines)? {
                        Args::REGISTER(r) => reg = r,
                        _ => return Err("First argument to offset must be register"),
                    },
                    _ => return Err("First argument to offset must be register"),
                }

                if split.len() == 1 {
                    return Ok(Args::OFFSET(reg, 0));
                }

                if split.len() < 3 {
                    panic!("Offset argument incomplete");
                }

                let number = match &split[2] {
                    Token::INT(s) => *s,
                    _ => panic!("Offset must be int"),
                };
                match &split[1] {
                    Token::PLUS => {
                        return Ok(Args::OFFSET(reg, number));
                    }
                    Token::MINUS => {
                        return Ok(Args::OFFSET(reg, -number));
                    }
                    _ => return Err("Only operator + or - allowed in offset"),
                }
            }
            Token::INT(i) => Ok(Args::INT(*i)),
            Token::DECIMAL(j) => Ok(Args::DECIMAL(*j)),
            Token::STRING(str) => Ok(Args::STRING(str.to_owned())),
            _ => Err("Illegal argument to instruction"),
        }
    }
}

#[derive(Debug)]
pub struct Instr {
    pub name: Instruction,
    pub len: usize,
    pub args: Option<Vec<Args>>,
}

impl Instr {
    pub fn new(name: String) -> Result<Self, &'static str> {
        let (ins, len) = Instruction::new(name)?;
        Ok(Self {
            name: ins,
            len,
            args: {
                if len != 1 {
                    Some(Vec::<Args>::new())
                } else {
                    None
                }
            },
        })
    }

    pub fn add_args(&mut self, args: &[Token], defines: &Vec<Define>) -> Result<(), &'static str> {
        if args.len() + 1 != self.len {
            return Err("Instruction has been given more or less arguments than needed");
        }
        for arg in args {
            let ar = Args::new(arg, defines)?;
            self.args.as_mut().unwrap().push(ar);
        }
        if !self.name.is_farg_nreg() {
            match self.args.as_ref().unwrap()[0] {
                Args::REGISTER(..) => {}
                _ => return Err("First argument to instruction must be a register"),
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Attrs {
    FIRMWARE,
}

#[derive(Debug)]
pub struct Attr(pub Attrs, pub Option<Token>);
impl Attr {
    pub fn new(name: &str, _arg: Option<Token>) -> Result<Self, &'static str> {
        match name {
            "firmware" => Ok(Self(Attrs::FIRMWARE, None)),
            _ => Err("Unknown attribute or attribute not expected here"),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub ins: Vec<Instr>,
    pub attrs: Option<Vec<Attr>>,
}

impl Function {
    pub fn new(func: &Token) -> Result<Self, &'static str> {
        let name = match func {
            Token::IDENT(s) => s.to_string(),
            _ => return Err("Expected identifier after keyword 'func'"),
        };
        Ok(Self {
            name,
            ins: Vec::new(),
            attrs: Some(Vec::new()),
        })
    }

    pub fn add_ins(&mut self, ins: Instr) {
        self.ins.push(ins)
    }
}

pub struct Define(String, Token);

pub struct Parser {
    define: Vec<Define>,
    pub funcs: Vec<Function>,
    state: u8,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            define: Vec::new(),
            funcs: Vec::new(),
            state: 0u8,
        }
    }

    pub fn parse(&mut self, target: Vec<Token>) -> Result<(), &str> {
        let mut inlabel = false;
        let label;
        if target.len() == 0 {
            return Ok(());
        }
        if self.state == 0 {
            match &target[0] {
                Token::FUNC => {
                    self.state = 2;
                    if target.len() < 2 {
                        return Err("Expected identifier after keyword 'func'");
                    }
                    match Function::new(&target[1]) {
                        Ok(s) => self.funcs.push(s),
                        Err(e) => return Err(e),
                    }
                }
                Token::INCLUDE => {
                    if target.len() < 2 {
                        return Err("include keyword has to be followed by a file name");
                    }
                    let name = match &target[1] {
                        Token::STRING(s) => s,
                        _ => {
                            return Err(
                                "Expected string representing filename after define keyword",
                            )
                        }
                    };
                    let mut src = Handle::new(name);
                    let mut parser = Parser::new();
                    loop {
                        let mut line = src.read_line();
                        if &line == "EOF" {
                            break;
                        }
                        line.push(' ');
                        let split = match line_split(line.as_bytes()) {
                            Ok(s) => s,
                            Err(e) => src.error(e),
                        };
                        match parser.parse(split) {
                            Ok(..) => {}
                            Err(e) => src.error(e),
                        }
                    }
                    self.define.append(&mut parser.define);
                    self.funcs.append(&mut parser.funcs);
                }
                Token::DEFINE => {
                    if target.len() < 3 {
                        return Err(
                            "define keyword has to be followed by an identifier and a value",
                        );
                    }
                    let name = match &target[1] {
                        Token::IDENT(s) => s.to_string(),
                        _ => return Err("Expected identifier after define keyword"),
                    };
                    match &target[2] {
                        Token::INT(..)
                        | Token::DECIMAL(..)
                        | Token::STRING(..)
                        | Token::IDENT(..) => self.define.push(Define(name, target[2].clone())),
                        _ => return Err("Expected value here"),
                    }
                }
                _ => return Err("Only functions are allowed at top level"),
            }
        } else if self.state == 2 {
            match &target[0] {
                Token::IDENT(s) => {
                    let mut this_func = self.funcs.pop().unwrap();
                    let mut ins = Instr::new(s.to_owned())?;
                    if ins.len != 1 {
                        ins.add_args(&target[1..], &self.define)?;
                    }
                    this_func.add_ins(ins);
                    self.funcs.push(this_func);
                }
                Token::LABEL(s) => {
                    if inlabel {
                        label = Function::new(&Token::IDENT(s.to_string()))?;
                        self.funcs.push(label);
                    } else {
                        inlabel = true;
                        self.funcs
                            .push(Function::new(&Token::IDENT(s.to_string()))?);
                    }
                }
                Token::ATTR(s) => {
                    let mut this_func = self.funcs.pop().unwrap();
                    let attr = Attr::new(&s, target.get(1).cloned())?;
                    this_func.attrs.as_mut().unwrap().push(attr);
                    self.funcs.push(this_func);
                }
                Token::ENDFUNC => {
                    self.state = 0;
                    if inlabel {
                        inlabel = false;
                    }
                }
                _ => return Err("Expected instruction name here"),
            }
        }
        Ok(())
    }
}
