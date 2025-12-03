// src/mir.rs
// Arena-based MIR (minimal starter)

use mir_events::EditEvent;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::middle::mir_events;
pub type NodeId = usize;
pub type SymbolId = usize;

/// Optional provenance: original file/span, frontend node id, pretty-print hint.
#[derive(Debug, Clone)]
pub struct Provenance {
    pub file: Option<String>,
    pub span: Option<(usize, usize)>, // byte offsets
    pub frontend_id: Option<usize>,
    pub pretty_hint: Option<String>,
}

/// Meta for each node
#[derive(Debug, Clone)]
pub struct Meta {
    pub prov: Option<Provenance>,
    pub created_at: u64,
    // other metadata fields (owner/agent id, confidence, tags)
}

impl Meta {
    pub fn now() -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Meta {
            prov: None,
            created_at: ts,
        }
    }
}

impl Default for Meta {
    fn default() -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Meta {
            prov: None,
            created_at: ts,
        }
    }
}

/// The MIR node kinds: statements, expressions, and meta ops.
#[derive(Debug, Clone)]
pub enum NodeKind {
    // Module/decls
    Module {
        name: Option<SymbolId>,
        body: Vec<NodeId>,
    },

    // Declarations
    FunctionDef {
        name: SymbolId,
        params: Vec<SymbolId>,
        body: NodeId,            // Block node id
        decorators: Vec<NodeId>, // decorator expressions (node ids)
    },
    ClassDef {
        name: SymbolId,
        bases: Vec<NodeId>,
        body: NodeId,
        decorators: Vec<NodeId>,
    },
    //mirrors Python/Jue semantics more clearly: expressions can appear as statements.
    ExprStmt {
        expr: NodeId,
    },
    // Statements
    Block {
        stmts: Vec<NodeId>,
    },
    Assign {
        targets: Vec<NodeId>,
        value: NodeId,
    },
    AugAssign {
        target: NodeId,
        op: String,
        value: NodeId,
    },
    Return {
        value: Option<NodeId>,
    },
    If {
        test: NodeId,
        body: NodeId,
        orelse: Option<NodeId>,
    },
    For {
        target: NodeId,
        iter: NodeId,
        body: NodeId,
        orelse: Option<NodeId>,
    },
    While {
        test: NodeId,
        body: NodeId,
        orelse: Option<NodeId>,
    },
    Pass,
    Break,
    Continue,
    Raise {
        value: Option<NodeId>,
    },
    With {
        items: Vec<(NodeId, Option<NodeId>)>,
        body: NodeId,
    },
    Try {
        body: NodeId,
        handlers: Vec<(Option<NodeId>, NodeId)>, // (exception type, handler body)
        orelse: NodeId,
        finalbody: NodeId,
    },

    // Expressions
    Identifier(SymbolId),
    Literal(LiteralValue),
    BinaryOp {
        op: String,
        lhs: NodeId,
        rhs: NodeId,
    },
    UnaryOp {
        op: String,
        expr: NodeId,
    },
    Call {
        func: NodeId,
        args: Vec<NodeId>,
    },
    Lambda {
        params: Vec<SymbolId>,
        body: NodeId,
    },
    Attr {
        object: NodeId,
        attr: SymbolId,
    },
    Index {
        object: NodeId,
        index: NodeId,
    },
    ListLiteral {
        elts: Vec<NodeId>,
    },
    DictLiteral {
        entries: Vec<(NodeId, NodeId)>,
    },
    TupleLiteral {
        elts: Vec<NodeId>,
    },

    // Meta / homoiconic
    QuoteSyntax {
        node: NodeId,
    },
    QuoteValue {
        node: NodeId,
    },
    SpliceSyntax {
        node: NodeId,
    },
    SpliceValue {
        node: NodeId,
    },
    Eval {
        node: NodeId,
    },

    // Macros
    MacroDef {
        name: SymbolId,
        args: Vec<SymbolId>,
        body: NodeId,
    },
    MacroCall {
        name: SymbolId,
        args: Vec<NodeId>,
    },

    // Placeholder for unknown/experimental nodes
    Unknown,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

/// A node in the MIR arena
#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub meta: Meta,
}

impl Node {
    pub fn new(id: NodeId, kind: NodeKind, meta: Meta) -> Self {
        Self { id, kind, meta }
    }
}

/// The MIR arena container
#[derive(Debug)]
pub struct Mir {
    pub nodes: Vec<Node>,
    pub symbol_table: SymbolTable,
    pub edit_log: Vec<EditEvent>, // requires mir_events module
}

impl Mir {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            symbol_table: SymbolTable::new(),
            edit_log: Vec::new(),
        }
    }

    pub fn alloc(&mut self, kind: NodeKind, meta: Meta) -> NodeId {
        let id = self.nodes.len();
        let node = Node::new(id, kind, meta);
        self.nodes.push(node);
        id
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    // Basic mutation APIs — these append events to the edit_log and return the NodeId(s).

    pub fn insert_node(
        &mut self,
        parent: Option<NodeId>,
        kind: NodeKind,
        meta: Meta,
        position: Option<usize>,
        actor: Option<String>,
    ) -> NodeId {
        let id = self.alloc(kind.clone(), meta.clone());
        // If parent is a Block or Module, insert into its children vector at position.
        if let Some(pid) = parent {
            if let Some(parent_node) = self.get_mut(pid) {
                match &mut parent_node.kind {
                    NodeKind::Module { body, .. } => {
                        let pos = position.unwrap_or(body.len());
                        body.insert(pos, id);
                    }
                    NodeKind::Block { stmts } => {
                        let pos = position.unwrap_or(stmts.len());
                        stmts.insert(pos, id);
                    }
                    _ => {
                        // For other parents, caller must manually attach nodes.
                    }
                }
            }
        }
        let ev = EditEvent::insert(id, parent, position, actor);
        self.edit_log.push(ev);
        id
    }

    pub fn replace_node(
        &mut self,
        id: NodeId,
        new_kind: NodeKind,
        actor: Option<String>,
    ) -> Result<(), String> {
        if let Some(node) = self.get_mut(id) {
            let old_kind = std::mem::replace(&mut node.kind, new_kind.clone());
            let ev = EditEvent::replace(id, old_kind, new_kind, actor);
            self.edit_log.push(ev);
            Ok(())
        } else {
            Err(format!("No such node {}", id))
        }
    }

    pub fn delete_node(&mut self, id: NodeId, actor: Option<String>) -> Result<(), String> {
        if id >= self.nodes.len() {
            return Err(format!("No such node {}", id));
        }
        // NB: We do not physically remove the node from the arena (keep stable NodeIds).
        // Instead we mark it Unknown — this preserves NodeId references for history.
        if let Some(node) = self.get_mut(id) {
            let old_kind = std::mem::replace(&mut node.kind, NodeKind::Unknown);
            let ev = EditEvent::delete(id, old_kind, actor);
            self.edit_log.push(ev);
            Ok(())
        } else {
            Err(format!("No such node {}", id))
        }
    }
}

/// Small symbol table
#[derive(Debug)]
pub struct SymbolTable {
    pub symbols: Vec<String>,
    pub map: HashMap<String, SymbolId>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> SymbolId {
        if let Some(&id) = self.map.get(s) {
            return id;
        }
        let id = self.symbols.len();
        self.symbols.push(s.to_string());
        self.map.insert(s.to_string(), id);
        id
    }

    pub fn lookup(&self, id: SymbolId) -> Option<&str> {
        self.symbols.get(id).map(|s| s.as_str())
    }
}
