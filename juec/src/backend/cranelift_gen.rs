// backend/cranelift_gen.rs
use crate::middle::mir::{LiteralValue, Mir, NodeId, NodeKind};
use anyhow::Result;

use cranelift::prelude::*;
use cranelift_codegen::ir::types;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{FuncId, Linkage, Module as CrModule};
use cranelift_object::{ObjectBuilder, ObjectModule};
/// Cranelift code generator
pub struct CraneliftCodeGen {
    builder_ctx: FunctionBuilderContext,
    module: ObjectModule,
    pub function_ids: Vec<FuncId>,
}

impl CraneliftCodeGen {
    pub fn new(module_name: &str) -> Result<Self> {
        use cranelift::prelude::*;
        use cranelift_codegen::settings;
        use cranelift_module::default_libcall_names;
        //use cranelift_object::{ObjectBuilder, ObjectModule};

        // Create ISA builder for the host machine
        let isa_builder =
            cranelift_native::builder().map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let flags = settings::Flags::new(settings::builder());
        let isa = isa_builder.finish(flags)?; // Arc<dyn TargetIsa>

        // Create the ObjectBuilder
        let obj_builder = ObjectBuilder::new(isa, module_name, default_libcall_names())?;
        let module = ObjectModule::new(obj_builder);

        Ok(Self {
            builder_ctx: FunctionBuilderContext::new(),
            module,
            function_ids: vec![],
        })
    }

    /// Top-level MIR generation
    pub fn generate(&mut self, mir: &Mir) -> Result<()> {
        for node in &mir.nodes {
            if let NodeKind::FunctionDef { name, body, .. } = &node.kind {
                self.gen_function(mir, *name, *body)?;
            }
        }
        Ok(())
    }

    fn gen_function(&mut self, mir: &Mir, name_sym: usize, body_id: NodeId) -> Result<()> {
        let name = mir.symbol_table.lookup(name_sym).unwrap_or("<anon>");

        // Create a function signature
        let mut sig = self.module.make_signature();
        sig.returns.push(AbiParam::new(types::I64)); // placeholder return type

        // Declare the function
        let func_id = self.module.declare_function(name, Linkage::Export, &sig)?;
        self.function_ids.push(func_id);

        // Make context for building the function
        let mut ctx = self.module.make_context();
        ctx.func.signature = sig;

        // Build IR - create builder first
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut self.builder_ctx);
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // Generate code for body statements - move this to a separate method that doesn't need &mut self
        gen_block_helper(mir, &mut builder, body_id)?;

        // Return from function (placeholder)
        builder.ins().return_(&[]);
        builder.finalize();

        // Define the function
        self.module.define_function(func_id, &mut ctx)?;

        Ok(())
    }

    fn gen_block(
        &mut self,
        mir: &Mir,
        builder: &mut FunctionBuilder,
        block_id: NodeId,
    ) -> Result<()> {
        if let Some(node) = mir.get(block_id) {
            if let NodeKind::Block { stmts } = &node.kind {
                for stmt_id in stmts {
                    self.gen_stmt(mir, builder, *stmt_id)?;
                }
            }
        }
        Ok(())
    }

    fn gen_stmt(
        &mut self,
        mir: &Mir,
        builder: &mut FunctionBuilder,
        node_id: NodeId,
    ) -> Result<()> {
        if let Some(node) = mir.get(node_id) {
            match &node.kind {
                NodeKind::Assign { targets: _, value } => {
                    // evaluate value, ignore storage for now
                    self.gen_expr(mir, builder, *value)?;
                }
                NodeKind::Return { value } => {
                    if let Some(v) = value {
                        self.gen_expr(mir, builder, *v)?;
                    }
                }
                NodeKind::ExprStmt { expr } => {
                    self.gen_expr(mir, builder, *expr)?;
                }
                _ => {
                    // Placeholder for other statements
                }
            }
        }
        Ok(())
    }

    fn gen_expr(
        &mut self,
        mir: &Mir,
        builder: &mut FunctionBuilder,
        node_id: NodeId,
    ) -> Result<Value> {
        if let Some(node) = mir.get(node_id) {
            match &node.kind {
                NodeKind::Literal(lit) => match lit {
                    LiteralValue::Int(i) => Ok(builder.ins().iconst(types::I64, *i)),
                    LiteralValue::Bool(b) => {
                        Ok(builder.ins().iconst(types::I64, if *b { 1 } else { 0 }))
                    }
                    _ => Ok(builder.ins().iconst(types::I64, 0)), // fallback
                },
                NodeKind::BinaryOp { lhs, rhs, op } => {
                    let l = self.gen_expr(mir, builder, *lhs)?;
                    let r = self.gen_expr(mir, builder, *rhs)?;
                    let val = match op.as_str() {
                        "+" => builder.ins().iadd(l, r),
                        "-" => builder.ins().isub(l, r),
                        "*" => builder.ins().imul(l, r),
                        "/" => builder.ins().sdiv(l, r),
                        _ => builder.ins().iconst(types::I64, 0),
                    };
                    Ok(val)
                }
                _ => Ok(builder.ins().iconst(types::I64, 0)),
            }
        } else {
            Ok(builder.ins().iconst(types::I64, 0))
        }
    }

    /// Print IR for all declared functions
    pub fn print_ir(&self) {
        for fid in &self.function_ids {
            println!("Function ID: {:?}", fid);
            // Note: Cranelift doesn't provide a direct 'print IR' API outside debug context
        }
    }
}

// Helper function that doesn't require &mut self
fn gen_block_helper(mir: &Mir, builder: &mut FunctionBuilder, block_id: NodeId) -> Result<()> {
    if let Some(node) = mir.get(block_id) {
        if let NodeKind::Block { stmts } = &node.kind {
            for stmt_id in stmts {
                gen_stmt_helper(mir, builder, *stmt_id)?;
            }
        }
    }
    Ok(())
}

fn gen_stmt_helper(mir: &Mir, builder: &mut FunctionBuilder, node_id: NodeId) -> Result<()> {
    if let Some(node) = mir.get(node_id) {
        match &node.kind {
            NodeKind::Assign { targets: _, value } => {
                // evaluate value, ignore storage for now
                gen_expr_helper(mir, builder, *value)?;
            }
            NodeKind::Return { value } => {
                if let Some(v) = value {
                    gen_expr_helper(mir, builder, *v)?;
                }
            }
            NodeKind::ExprStmt { expr } => {
                gen_expr_helper(mir, builder, *expr)?;
            }
            _ => {
                // Placeholder for other statements
            }
        }
    }
    Ok(())
}

fn gen_expr_helper(mir: &Mir, builder: &mut FunctionBuilder, node_id: NodeId) -> Result<Value> {
    if let Some(node) = mir.get(node_id) {
        match &node.kind {
            NodeKind::Literal(lit) => match lit {
                LiteralValue::Int(i) => Ok(builder.ins().iconst(types::I64, *i)),
                LiteralValue::Bool(b) => {
                    Ok(builder.ins().iconst(types::I64, if *b { 1 } else { 0 }))
                }
                _ => Ok(builder.ins().iconst(types::I64, 0)), // fallback
            },
            NodeKind::BinaryOp { lhs, rhs, op } => {
                let l = gen_expr_helper(mir, builder, *lhs)?;
                let r = gen_expr_helper(mir, builder, *rhs)?;
                let val = match op.as_str() {
                    "+" => builder.ins().iadd(l, r),
                    "-" => builder.ins().isub(l, r),
                    "*" => builder.ins().imul(l, r),
                    "/" => builder.ins().sdiv(l, r),
                    _ => builder.ins().iconst(types::I64, 0),
                };
                Ok(val)
            }
            _ => Ok(builder.ins().iconst(types::I64, 0)),
        }
    } else {
        Ok(builder.ins().iconst(types::I64, 0))
    }
}
