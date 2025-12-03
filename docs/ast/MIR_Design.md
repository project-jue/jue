Design decisions (short)

Arena-based MIR: Vec<Node> addressed by NodeId = usize. Cheap inserts/updates.

SymbolId: small integer handle for interned identifiers.

Node contains NodeKind + Meta (provenance: source span, frontend id, pretty hints).

EditEvent log: append-only list of ops (Insert, Replace, Delete) with timestamp and optional actor/evidence id.

API: Mir::insert_node, replace_node, delete_node — each produces an EditEvent.

Lowering: lower_frontend_to_mir(frontend::ast::Module) -> Mir stub.

Pretty-printer & Narsese translator: stubs that show how to extract readable code and NARS terms.