// src/mir_lower.rs
use crate::frontend::ast::{
    Expr as FrontExpr, Module as FrontModule, Param as FrontParam, Stmt as FrontStmt,
};
use crate::mir::{LiteralValue, Meta, Mir, NodeId, NodeKind};

/// Lower a frontend Module into a MIR arena. Returns the root NodeId (module).
pub fn lower_frontend_module(front: &FrontModule) -> Mir {
    let mut mir = Mir::new();
    let meta = Meta::now();
    let module_id = mir.alloc(
        NodeKind::Module {
            name: None,
            body: Vec::new(),
        },
        meta,
    );
    if let Some(body) = Some(&front.body) {
        for stmt in body {
            lower_stmt_into(&mut mir, module_id, stmt);
        }
    }
    mir
}

fn lower_stmt_into(mir: &mut Mir, parent_module_id: NodeId, stmt: &FrontStmt) -> Option<NodeId> {
    match stmt {
        FrontStmt::Expr(e) => {
            let expr_id = lower_expr(mir, e);
            mir.insert_node(
                Some(parent_module_id),
                NodeKind::Block {
                    stmts: vec![expr_id],
                },
                Meta::now(),
                None,
                None,
            );
            None
        }
        FrontStmt::Assign { targets, value } => {
            let mut t_ids = Vec::new();
            for t in targets {
                t_ids.push(lower_expr(mir, t));
            }
            let v_id = lower_expr(mir, value);
            let assign_id = mir.insert_node(
                Some(parent_module_id),
                NodeKind::Assign {
                    targets: t_ids,
                    value: v_id,
                },
                Meta::now(),
                None,
                None,
            );
            Some(assign_id)
        }
        FrontStmt::FuncDef {
            name,
            params,
            body,
            decorators,
        } => {
            let func_name = mir.symbol_table.intern(name);
            let mut param_ids = Vec::new();
            for p in params {
                param_ids.push(mir.symbol_table.intern(&p.name));
            }
            // Lower body as a Block node
            let mut stmt_ids = Vec::new();
            for s in body {
                if let Some(nid) = lower_stmt_into(mir, parent_module_id, s) {
                    stmt_ids.push(nid);
                }
            }
            let block_id = mir.alloc(NodeKind::Block { stmts: stmt_ids }, Meta::now());
            let deco_ids = decorators
                .iter()
                .map(|d| {
                    // treat decorator string as identifier node for now
                    let sid = mir.symbol_table.intern(d);
                    mir.alloc(NodeKind::Identifier(sid), Meta::now())
                })
                .collect();
            let func_id = mir.insert_node(
                Some(parent_module_id),
                NodeKind::FunctionDef {
                    name: func_name,
                    params: param_ids,
                    body: block_id,
                    decorators: deco_ids,
                },
                Meta::now(),
                None,
                None,
            );
            Some(func_id)
        }
        // add lowering for other statements here...
        _ => None,
    }
}

fn lower_expr(mir: &mut Mir, expr: &FrontExpr) -> NodeId {
    match expr {
        FrontExpr::Name(s) => {
            let sid = mir.symbol_table.intern(s);
            mir.alloc(NodeKind::Identifier(sid), Meta::now())
        }
        FrontExpr::Number(n) => {
            // parse number string heuristically; keep as float/int fallback
            if let Ok(i) = n.parse::<i64>() {
                mir.alloc(NodeKind::Literal(LiteralValue::Int(i)), Meta::now())
            } else if let Ok(f) = n.parse::<f64>() {
                mir.alloc(NodeKind::Literal(LiteralValue::Float(f)), Meta::now())
            } else {
                mir.alloc(
                    NodeKind::Literal(LiteralValue::String(n.clone())),
                    Meta::now(),
                )
            }
        }
        FrontExpr::String(s) => mir.alloc(
            NodeKind::Literal(LiteralValue::String(s.clone())),
            Meta::now(),
        ),
        FrontExpr::Bool(b) => mir.alloc(NodeKind::Literal(LiteralValue::Bool(*b)), Meta::now()),
        FrontExpr::None => mir.alloc(NodeKind::Literal(LiteralValue::None), Meta::now()),
        FrontExpr::BinOp { left, op, right } => {
            let l = lower_expr(mir, left);
            let r = lower_expr(mir, right);
            mir.alloc(
                NodeKind::BinaryOp {
                    op: op.clone(),
                    lhs: l,
                    rhs: r,
                },
                Meta::now(),
            )
        }
        FrontExpr::UnaryOp { op, expr } => {
            let inner = lower_expr(mir, expr);
            mir.alloc(
                NodeKind::UnaryOp {
                    op: op.clone(),
                    expr: inner,
                },
                Meta::now(),
            )
        }
        FrontExpr::Call { func, args } => {
            let f = lower_expr(mir, func);
            let mut arg_ids = Vec::new();
            for a in args {
                arg_ids.push(lower_expr(mir, a));
            }
            mir.alloc(
                NodeKind::Call {
                    func: f,
                    args: arg_ids,
                },
                Meta::now(),
            )
        }
        FrontExpr::Lambda { params, body } => {
            let mut param_sids = Vec::new();
            for p in params {
                param_sids.push(mir.symbol_table.intern(p));
            }
            let body_id = lower_expr(mir, body);
            mir.alloc(
                NodeKind::Lambda {
                    params: param_sids,
                    body: body_id,
                },
                Meta::now(),
            )
        }
        // Add Attr/Index/List/Tuple lowering as you expand frontend
        _ => mir.alloc(NodeKind::Unknown, Meta::now()),
    }
}
