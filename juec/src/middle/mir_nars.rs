// src/mir_nars.rs
//This emits a tiny set of Narselike sentences (string templates). Later you can convert to proper Narsese.

use crate::mir::{Mir, NodeId, NodeKind};
use std::collections::HashMap;

/// A tiny Narsese-like statement representation (string-based for now).
#[derive(Debug, Clone)]
pub struct NarsStatement {
    pub term: String,
    pub confidence: f64,
}

pub fn mir_to_nars_terms(mir: &Mir) -> Vec<NarsStatement> {
    let mut out = Vec::new();

    // Walk top-level module
    for node in &mir.nodes {
        match &node.kind {
            NodeKind::FunctionDef {
                name, params, body, ..
            } => {
                let fname = mir
                    .symbol_table
                    .lookup(*name)
                    .unwrap_or("<anon>")
                    .to_string();
                // function existence belief
                out.push(NarsStatement {
                    term: format!("(function {fname})"),
                    confidence: 0.9,
                });
                // parameters
                for p in params {
                    if let Some(pname) = mir.symbol_table.lookup(*p) {
                        out.push(NarsStatement {
                            term: format!("(has_param {fname} {pname})"),
                            confidence: 0.9,
                        });
                    }
                }
                // calls inside body -> collect call edges
                collect_calls_in_node(mir, *body, &mut out, 0.7);
            }
            _ => {}
        }
    }

    out
}

fn collect_calls_in_node(mir: &Mir, id: NodeId, out: &mut Vec<NarsStatement>, conf: f64) {
    if let Some(node) = mir.get(id) {
        match &node.kind {
            NodeKind::Call { func, args } => {
                let callee = if let Some(fnode) = mir.get(*func) {
                    match &fnode.kind {
                        NodeKind::Identifier(sid) => {
                            mir.symbol_table.lookup(*sid).unwrap_or("<unk>").to_string()
                        }
                        _ => "<complex>".to_string(),
                    }
                } else {
                    "<missing>".to_string()
                };
                out.push(NarsStatement {
                    term: format!("(calls <node{}> {})", id, callee),
                    confidence: conf,
                });
                for a in args {
                    collect_calls_in_node(mir, *a, out, conf * 0.8);
                }
            }
            NodeKind::Block { stmts } => {
                for s in stmts {
                    collect_calls_in_node(mir, *s, out, conf);
                }
            }
            // descend into many node kinds
            NodeKind::If { test, body, orelse } => {
                collect_calls_in_node(mir, *test, out, conf);
                collect_calls_in_node(mir, *body, out, conf);
                if let Some(o) = orelse {
                    collect_calls_in_node(mir, *o, out, conf);
                }
            }
            _ => {}
        }
    }
}
