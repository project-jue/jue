// juec/src/bin/demo.rs
use juec::middle::mir::*;

fn main() {
    // --- Setup MIR ---
    let mut mir = Mir::new();

    // --- Symbol IDs for demo ---
    let demo_module_sym: usize = 1;
    let add_one_sym: usize = 2;
    let x_sym: usize = 3;
    let deco_sym: usize = 4;
    // --- Insert Module ---
    let module_id = mir.insert_node(
        None,
        NodeKind::Module {
            name: Some(demo_module_sym),
            body: vec![],
        },
        Meta::default(),
        None,
        Some("demo".to_string()),
    );
    // Create decorator node
    let deco_id = mir.insert_node(
        Some(module_id),                // or function node, depending on context
        NodeKind::Identifier(deco_sym), // or call if decorator has arguments
        Meta::default(),
        None,
        Some("demo".to_string()),
    );
    // --- Insert Function: add_one(x) ---
    let fn_id = mir.insert_node(
        Some(module_id),
        NodeKind::FunctionDef {
            name: add_one_sym,
            params: vec![x_sym],
            body: NodeId::MAX, // placeholder; we'll create a block next
            decorators: vec![deco_id],
        },
        Meta::default(),
        None,
        Some("demo".to_string()),
    );

    // --- Create Function Body Block ---
    let fn_block_id = mir.insert_node(
        Some(fn_id),
        NodeKind::Block { stmts: vec![] },
        Meta::default(),
        None,
        Some("demo".to_string()),
    );

    // Update function body to point to the block
    if let Some(fn_node) = mir.get_mut(fn_id) {
        if let NodeKind::FunctionDef { body, .. } = &mut fn_node.kind {
            *body = fn_block_id;
        }
    }

    // --- Insert Return Statement: return x + 1 ---
    // 1. Literal node for `1`
    let one_id = mir.insert_node(
        Some(fn_block_id),
        NodeKind::Literal(LiteralValue::Int(1)),
        Meta::default(),
        None,
        Some("demo".to_string()),
    );

    // 2. Identifier `x`
    let x_id = mir.insert_node(
        Some(fn_block_id),
        NodeKind::Identifier(x_sym),
        Meta::default(),
        None,
        Some("demo".to_string()),
    );

    // 3. BinaryOp `x + 1`
    let sum_id = mir.insert_node(
        Some(fn_block_id),
        NodeKind::BinaryOp {
            lhs: x_id,
            rhs: one_id,
            op: "Operator::Add".to_string(),
        },
        Meta::default(),
        None,
        Some("demo".to_string()),
    );

    // 4. Return statement
    let _return_id = mir.insert_node(
        Some(fn_block_id),
        NodeKind::Return {
            value: Some(sum_id),
        },
        Meta::default(),
        None,
        Some("demo".to_string()),
    );

    // --- Debug print ---
    println!("=== MIR Dump ===");
    debug_print(&mir);
}

/// Simple recursive debug printer for MIR nodes
fn debug_print(mir: &Mir) {
    for node in &mir.nodes {
        println!("{:?}", node);
    }
}
// run with `cargo run --bin demo`
