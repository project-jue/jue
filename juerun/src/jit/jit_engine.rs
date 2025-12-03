// placeholder for future
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};

pub struct JitEngine {
    module: JITModule,
}

impl JitEngine {
    pub fn new() -> Self {
        let mut builder = JITBuilder::new(cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);
        Self { module }
    }

    pub fn compile_and_run<F: FnOnce(&mut JITModule) -> FuncId>(
        &mut self,
        compile_fn: F,
    ) -> *const u8 {
        let func_id = compile_fn(&mut self.module);
        self.module.finalize_definitions();
        unsafe { self.module.get_finalized_function(func_id) }
    }
}
