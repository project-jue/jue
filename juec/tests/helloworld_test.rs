#[cfg(test)]
mod tests {

    use jue_common::internal_ast::{JueAST, LiteralValue};
    use juec::backend::cranelift_gen::CraneliftCodeGen;
    use juec::frontend::semantic_analyzer::SemanticAnalyzer;

    #[test]
    fn hello_world_test() {
        let ast = JueAST::Module {
            name: "main".to_string(),
            body: vec![JueAST::Call {
                func: Box::new(JueAST::Identifier("print".to_string())),
                args: vec![JueAST::Literal(LiteralValue::String(
                    "Hello, World!".to_string(),
                ))],
            }],
        };

        // Semantic analysis
        SemanticAnalyzer::analyze(&ast).expect("Semantic error detected");

        // Cranelift IR generation
        let mut codegen = CraneliftCodeGen::new("main_module");
        let _ = codegen.generate(&ast);

        // Print Cranelift IR
        codegen.print_ir();
    }
}
