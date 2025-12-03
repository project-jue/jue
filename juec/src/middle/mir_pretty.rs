// src/mir_pretty.rs
use crate::mir::{LiteralValue, Mir, NodeId, NodeKind};

pub fn pretty_print_mir(mir: &Mir) -> String {
    if mir.nodes.is_empty() {
        return String::new();
    }
    // find root Module (node 0 by our constructor convention)
    let mut out = String::new();
    for node in &mir.nodes {
        if let NodeKind::Module { name, body } = &node.kind {
            out.push_str(&format!("# module {:?}\n", name));
            for stmt_id in body {
                out.push_str(&pretty_print_node(mir, *stmt_id, 0));
            }
            break;
        }
    }
    out
}

fn pretty_print_node(mir: &Mir, id: NodeId, indent: usize) -> String {
    let padding = "  ".repeat(indent);
    let mut out = String::new();
    if let Some(node) = mir.get(id) {
        match &node.kind {
            NodeKind::Block { stmts } => {
                for s in stmts {
                    out.push_str(&pretty_print_node(mir, *s, indent));
                }
            }
            NodeKind::Assign { targets, value } => {
                let t: Vec<_> = targets
                    .iter()
                    .filter_map(|tid| node_to_string(mir, *tid))
                    .collect();
                let v = node_to_string(mir, *value).unwrap_or_else(|| "<expr>".to_string());
                out.push_str(&format!("{}{} = {}\n", padding, t.join(", "), v));
            }
            NodeKind::FunctionDef {
                name, params, body, ..
            } => {
                let fname = mir.symbol_table.lookup(*name).unwrap_or("<anon>");
                let ps: Vec<_> = params
                    .iter()
                    .filter_map(|sid| mir.symbol_table.lookup(*sid).map(|s| s.to_string()))
                    .collect();
                out.push_str(&format!("{}def {}({}) :\n", padding, fname, ps.join(", ")));
                out.push_str(&pretty_print_node(mir, *body, indent + 1));
            }
            NodeKind::Call { func, args } => {
                let f = node_to_string(mir, *func).unwrap_or_else(|| "<call>".to_string());
                let as_: Vec<_> = args
                    .iter()
                    .filter_map(|a| node_to_string(mir, *a))
                    .collect();
                out.push_str(&format!("{}{}({})\n", padding, f, as_.join(", ")));
            }
            NodeKind::Literal(l) => {
                out.push_str(&format!("{}{:?}\n", padding, l));
            }
            NodeKind::Identifier(sid) => {
                let s = mir.symbol_table.lookup(*sid).unwrap_or("<id>");
                out.push_str(&format!("{}{}\n", padding, s));
            }
            _ => {
                out.push_str(&format!("{}/* {:?} */\n", padding, node.kind));
            }
        }
    }
    out
}

fn node_to_string(mir: &Mir, id: NodeId) -> Option<String> {
    mir.get(id).and_then(|n| match &n.kind {
        NodeKind::Identifier(sid) => mir.symbol_table.lookup(*sid).map(|s| s.to_string()),
        NodeKind::Literal(l) => Some(match l {
            LiteralValue::Int(i) => i.to_string(),
            LiteralValue::Float(f) => f.to_string(),
            LiteralValue::String(s) => format!("{:?}", s),
            LiteralValue::Bool(b) => b.to_string(),
            LiteralValue::None => "None".to_string(),
        }),
        _ => None,
    })
}
