use cranelift::codegen::isa::CallConv;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cranelift_const_function() {
        // Create JIT module for testing
        let builder = JITBuilder::new(cranelift_module::default_libcall_names())
            .expect("Failed to create JIT builder");
        let mut module = JITModule::new(builder);

        // Build function signature: fn() -> i64
        let mut sig = Signature::new(CallConv::SystemV);
        sig.returns.push(AbiParam::new(types::I64));

        // Declare the function
        let func_id = module
            .declare_function("const42", Linkage::Export, &sig)
            .expect("Failed to declare function");

        // Create function context
        let mut ctx = module.make_context();
        ctx.func.signature = sig;
        ctx.func.name = cranelift::codegen::ir::UserFuncName::user(0, 0);

        // Build function body
        let mut builder_ctx = FunctionBuilderContext::new();
        let mut fb = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);

        let bb = fb.create_block();
        fb.append_block_params_for_function_params(bb);
        fb.switch_to_block(bb);
        fb.seal_block(bb);

        let forty_two = fb.ins().iconst(types::I64, 42);
        fb.ins().return_(&[forty_two]);
        fb.finalize();

        // Define the function
        module
            .define_function(func_id, &mut ctx)
            .expect("Failed to define function");

        let _ = module.finalize_definitions();

        // Get pointer, cast, call
        let code_ptr = module.get_finalized_function(func_id);
        let myfn = unsafe { std::mem::transmute::<_, fn() -> i64>(code_ptr) };
        assert_eq!(myfn(), 42);
    }
}
