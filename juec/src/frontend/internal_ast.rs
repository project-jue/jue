pub type NodeId = usize;
pub type SymbolId = usize;

pub struct Meta {}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

pub struct Node {
    pub meta: Meta,
    pub kind: NodeKind,
}

#[derive(Debug)]
pub enum NodeKind {
    // === MODULES / DECLS ===
    Module {
        name: SymbolId,
        body: Vec<NodeId>,
    },

    FunctionDef {
        name: SymbolId,
        params: Vec<NodeId>,
        body: NodeId,
    },

    ClassDef {
        name: SymbolId,
        bases: Vec<NodeId>,
        body: NodeId,
    },

    // === STATEMENTS ===
    Block(Vec<NodeId>),

    Assignment {
        target: NodeId,
        value: NodeId,
    },

    AttributeAssign {
        object: NodeId,
        field: SymbolId,
        value: NodeId,
    },

    Return(NodeId),

    If {
        test: NodeId,
        body: NodeId,
        orelse: Option<NodeId>,
    },

    While {
        test: NodeId,
        body: NodeId,
    },

    For {
        target: NodeId,
        iter: NodeId,
        body: NodeId,
    },

    // === EXPRESSIONS ===
    Identifier(SymbolId),

    Literal(LiteralValue),

    BinaryOp {
        op: Operator,
        lhs: NodeId,
        rhs: NodeId,
    },

    Call {
        func: NodeId,
        args: Vec<NodeId>,
    },

    AttributeAccess {
        object: NodeId,
        field: SymbolId,
    },

    Index {
        object: NodeId,
        index: NodeId,
    },

    ListLiteral(Vec<NodeId>),
    DictLiteral(Vec<(NodeId, NodeId)>),
    TupleLiteral(Vec<NodeId>),

    // === HOF / META ===
    Lambda {
        params: Vec<NodeId>,
        body: NodeId,
    },

    Eval(NodeId), // Evaluate code at runtime

    // === HOMOICONICITY ===
    QuoteSyntax(NodeId), // Preserve exact syntactic structure
    QuoteValue(NodeId),  // Structural AST, no syntax sugar

    SpliceSyntax(NodeId), // Insert AST into syntax context
    SpliceValue(NodeId),  // Insert computed runtime value as AST

    // === OPTIONAL: MACROS ===
    MacroDef {
        name: SymbolId,
        args: Vec<SymbolId>,
        body: NodeId,
    },

    MacroCall {
        macro_name: SymbolId,
        args: Vec<NodeId>,
    },
}

#[derive(Debug)]
pub enum JueAST {
    Module {
        name: String,
        body: Vec<JueAST>,
    },
    FunctionDef {
        id: NodeId,
        name: String,
        args: Vec<JueAST>,
        body: Box<JueAST>,
    },
    ClassDef {
        id: NodeId,
        name: String,
        body: Box<JueAST>,
    },
    Block(Vec<JueAST>),
    Return {
        value: Box<JueAST>,
    },
    Assignment {
        target: Box<JueAST>,
        value: Box<JueAST>,
    },
    If {
        test: Box<JueAST>,
        body: Box<JueAST>,
        orelse: Option<Box<JueAST>>,
    },
    BinaryOp {
        op: Operator,
        lhs: Box<JueAST>,
        rhs: Box<JueAST>,
    },
    Call {
        func: Box<JueAST>,
        args: Vec<JueAST>,
    },
    Literal(LiteralValue),
    Identifier(String),
    QuoteBlock {
        id: NodeId,
        quoted_ast: Box<JueAST>,
    },
    Splice(Box<JueAST>),
}
