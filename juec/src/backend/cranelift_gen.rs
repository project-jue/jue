//! Cranelift code generation backend
use cranelift::codegen::ir::UserFuncName;
use cranelift::codegen::isa::CallConv;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
pub(crate) use frontend::internal_ast::{JueAST, LiteralValue, Operator};

pub struct CraneliftCodeGen {
    name: String,
    module: JITModule,
    functions: std::collections::HashMap<String, cranelift_module::FuncId>,
}

impl CraneliftCodeGen {
    pub fn new(name: &str) -> Self {
        let builder = JITBuilder::new(cranelift_module::default_libcall_names())
            .expect("Failed to create JIT builder");
        let module = JITModule::new(builder);

        Self {
            name: name.to_string(),
            module,
            functions: std::collections::HashMap::new(),
        }
    }

    pub fn generate(&mut self, ast: &JueAST) -> Result<(), String> {
        match self.generate_module(ast) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Code generation failed: {}", e)),
        }
    }

    fn generate_module(&mut self, ast: &JueAST) -> Result<(), String> {
        match ast {
            JueAST::Module { name: _, body } => {
                // For now, just compile the body as a simple main function
                self.compile_statements(body)?;
                Ok(())
            }
            _ => Err("Expected module AST".to_string()),
        }
    }

    fn compile_statements(&mut self, statements: &[JueAST]) -> Result<(), String> {
        for stmt in statements {
            self.compile_statement(stmt)?;
        }
        Ok(())
    }

    fn compile_statement(&mut self, stmt: &JueAST) -> Result<(), String> {
        match stmt {
            JueAST::Call { func, args } => self.compile_call(func, args),
            _ => Err(format!("Unsupported statement: {:?}", stmt)),
        }
    }

    fn compile_call(&mut self, func: &JueAST, args: &[JueAST]) -> Result<(), String> {
        match func {
            JueAST::Identifier(name) if name == "print" => {
                self.compile_print_call(args)?;
                Ok(())
            }
            _ => Err(format!("Unsupported function call: {:?}", func)),
        }
    }

    fn compile_print_call(&self, _args: &[JueAST]) -> Result<(), String> {
        // For now, just return an empty function since we're using JIT compilation
        // In a real implementation, this would generate calls to println! or similar
        Ok(())
    }

    pub fn print_ir(&self) {
        println!("Cranelift IR generated for module: {}", self.name);
        println!("Functions compiled: {}", self.functions.len());

        // Note: In a full implementation, you would print the actual IR here
        // Cranelift doesn't have a direct equivalent to LLVM's print_to_string(),
        // but you could implement custom IR printing if needed
    }

    // Compiles a simple expression to a function
    pub fn compile_expression(&mut self, expr: &JueAST) -> Result<(), String> {
        match expr {
            JueAST::Literal(LiteralValue::Int(val)) => {
                // Generate a function that returns this integer
                self.generate_simple_int_function(*val)
            }
            JueAST::BinaryOp { op, lhs, rhs } => self.compile_binary_op(op, lhs, rhs),
            _ => Err(format!("Unsupported expression: {:?}", expr)),
        }
    }

    fn generate_simple_int_function(&mut self, value: i64) -> Result<(), String> {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.returns.push(AbiParam::new(types::I64));

        let function_id = self
            .module
            .declare_function("simple_int", Linkage::Export, &sig)
            .map_err(|e| format!("Failed to declare function: {}", e))?;

        let mut ctx = self.module.make_context();
        ctx.func.signature = sig;
        // Set external name
        ctx.func.name = UserFuncName::user(0, 0);

        let mut builder_ctx = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);

        let block = builder.create_block();
        builder.switch_to_block(block);

        let const_val = builder.ins().iconst(types::I64, value);
        builder.ins().return_(&[const_val]);
        builder.seal_block(block);
        builder.finalize();

        self.module
            .define_function(function_id, &mut ctx)
            .map_err(|e| format!("Failed to define function: {}", e))?;

        let _ = self.module.finalize_definitions();
        self.functions.insert("simple_int".to_string(), function_id);
        Ok(())
    }

    fn compile_binary_op(
        &mut self,
        op: &Operator,
        lhs: &JueAST,
        rhs: &JueAST,
    ) -> Result<(), String> {
        // For now, just compile both sides as integers and apply the operation
        match (lhs, rhs) {
            (
                JueAST::Literal(LiteralValue::Int(lval)),
                JueAST::Literal(LiteralValue::Int(rval)),
            ) => {
                let result = match op {
                    Operator::Add => lval + rval,
                    Operator::Subtract => lval - rval,
                    Operator::Multiply => lval * rval,
                    Operator::Divide => lval / rval,
                };
                self.generate_simple_int_function(result)
            }
            _ => Err("Only literal integer operations supported for now".to_string()),
        }
    }
}
