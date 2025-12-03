// frontend/ast.rs

#[derive(Debug)]
pub struct Module {
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Assign {
        targets: Vec<Expr>,
        value: Expr,
    },
    AugAssign {
        target: Expr,
        op: String,
        value: Expr,
    },
    FuncDef {
        name: String,
        params: Vec<Param>,
        body: Vec<Stmt>,
        decorators: Vec<String>,
    },
    ClassDef {
        name: String,
        body: Vec<Stmt>,
        decorators: Vec<String>,
    },
    If {
        test: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>,
    },
    For {
        target: Expr,
        iter: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>,
    },
    While {
        test: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>,
    },
    Return(Option<Expr>),
    Pass,
    Break,
    Continue,
    Raise(Option<Expr>),
    With {
        items: Vec<(Expr, Option<Expr>)>, // (context_expr, optional "as" alias)
        body: Vec<Stmt>,
    },
    Try {
        body: Vec<Stmt>,                          // try block
        handlers: Vec<(Option<Expr>, Vec<Stmt>)>, // (exception type, handler body)
        orelse: Vec<Stmt>,                        // else block
        finalbody: Vec<Stmt>,                     // finally block
    },
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub default: Option<Expr>,
    pub kind: ParamKind,
}

#[derive(Debug)]
pub enum ParamKind {
    Positional,
    Star,
    DoubleStar,
}

#[derive(Debug)]
pub enum Expr {
    Name(String),
    Number(String),
    String(String),
    Bool(bool),
    None,
    BinOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    UnaryOp {
        op: String,
        expr: Box<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
}
