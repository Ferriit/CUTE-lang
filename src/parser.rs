use super::lexer::{Token, Lexeme};

pub struct Program {
    pub imports: Vec<Import>,
    pub declarations: Vec<Declaration>,
}

pub struct Import {
    pub path: String,
}

pub enum Declaration {
    Namespace {
        name: String,
        declarations: Vec<Declaration>,
    },

    Class {
        name: String,
        inherits: Option<Type>,
        members: Vec<ClassMember>,
    },

    Struct {
        name: String,
        members: Vec<ClassMember>,
    },

    Enum {
        name: String,
        variants: Vec<EnumVariant>,
    },

    Function {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Block,
    },

    Variable {
        name: String,
        ty: Option<Type>,
        initializer: Option<Expression>,
    },

    Constant {
        name: String,
        ty: Type,
        value: Expression,
    },
}

pub enum ClassMember {
    Variable {
        visibility: Visibility,
        declaration: Box<Declaration>,
    },

    Function {
        visibility: Visibility,
        declaration: Box<Declaration>,
    },

    Constructor {
        parameters: Vec<Parameter>,
        body: Block,
    },

    Destructor {
        body: Block,
    },
}

pub enum Visibility {
    Public,
    Private,
}

pub struct Parameter {
    pub name: String,
    pub ty: Type,
}

pub struct EnumVariant {
    pub name: String,
    pub value: Option<Expression>,
}

pub struct Block {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Block(Block),

    Declaration(Declaration),

    Expression(Expression),

    Return(Option<Expression>),

    If {
        condition: Expression,
        then_block: Block,
        else_branch: Option<Box<Statement>>,
    },

    While {
        condition: Expression,
        body: Block,
    },

    For {
        initializer: Option<Box<Statement>>,
        condition: Option<Expression>,
        increment: Option<Expression>,
        body: Block,
    },

    RangeFor {
        variable: Declaration,
        range: Expression,
        body: Block,
    },

    Run(Block),

    Jump(String),

    Label(String),
}

pub enum Expression {
    Literal(Literal),

    Identifier(String),

    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },

    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },

    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },

    MemberAccess {
        object: Box<Expression>,
        member: String,
    },

    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
    },

    ArrayLiteral(Vec<Expression>),
}

pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
}

pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    ShiftLeft,
    ShiftRight,

    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    LogicalAnd,
    LogicalOr,

    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
}

pub enum UnaryOperator {
    Negate,
    Not,
    PreIncrement,
    PreDecrement,
    PostIncrement,
    PostDecrement,
}

pub enum Type {
    Void,
    Bool,

    Integer(IntegerType),
    Float(FloatType),

    Char,
    C16,
    C32,

    String,
    StringView,

    Pointer(IntegerPointerType),

    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),

    Optional(Box<Type>),
    Result(Box<Type>),

    Array {
        element_type: Box<Type>,
        size: Option<usize>,
    },

    Custom(String),
}

pub enum IntegerType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Err,
}

pub enum FloatType {
    F32,
    F64,
}

pub enum IntegerPointerType {
    Iptr,
    Uptr,
}

fn parse_imports(tokens: Vec<Token>) -> Vec<Import> {
    let mut curr_include: String = "".to_string();
    let mut build_include: bool = false;

    let mut includes: Vec<Import> = Vec::new();
    let mut expect_semicolon: bool = false;
    
    for tok in tokens {
        match tok.lex {
            Lexeme::KeyW(s) => {
                if expect_semicolon {
                    panic!(
                        "{}:{} Expected semicolon.",
                        tok.line,
                        tok.column
                    );
                }
                if s == "using" {
                    build_include = true;
                }
            },
            Lexeme::Op(s) => {
                if expect_semicolon {
                    panic!(
                        "{}:{} Expected semicolon.",
                        tok.line,
                        tok.column
                    );
                }
                if build_include {
                    if s == "<" && curr_include.is_empty() {
                        curr_include += s.as_str();
                    }
                    else if s == ">" {
                        curr_include += s.as_str();
                        build_include = false;

                        includes.push(Import {
                            path: curr_include.clone()
                        });

                        curr_include = "".to_string();
                        expect_semicolon = true;
                    }
                    else if s == "<" && !curr_include.is_empty() {
                        panic!(
                            "{}:{} Invalid import statement. Expected '<' or \'\"\'. Got {}.", 
                            tok.line, 
                            tok.column,
                            s
                        );
                    }
                    else {
                        curr_include += s.as_str();
                    }
                }
            },
            Lexeme::Iden(s) => {
                if expect_semicolon {
                    panic!(
                        "{}:{} Expected semicolon.",
                        tok.line,
                        tok.column
                    );
                }
                curr_include += s.as_str();
            },
            Lexeme::StringLit(s) => {
                if expect_semicolon {
                    panic!(
                        "{}:{} Expected semicolon.",
                        tok.line,
                        tok.column
                    );
                }
                if build_include {
                    if curr_include.is_empty() {
                        curr_include += s.as_str();
                        build_include = false;

                        includes.push(Import {
                            path: curr_include.clone()
                        });
                        expect_semicolon = true;
                    }
                    else {
                        panic!(
                            "{}:{} Invalid import statement. Expected '<' or \'\"\'. Got {}.", 
                            tok.line, 
                            tok.column,
                            s.as_bytes()[0]
                        );
                    }
                }
            },
            Lexeme::Semicolon => {
                if expect_semicolon {
                    expect_semicolon = false;
                }
            },
            _ => {
                if expect_semicolon {
                    panic!(
                        "{}:{} Expected semicolon.",
                        tok.line,
                        tok.column
                    );
                }
            }
        }
    }

    includes
}

pub fn parse(tokens: Vec<Token>) -> Program {
    Program {
        imports: parse_imports(tokens),
        declarations: Vec::new()
    }
}
