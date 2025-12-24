# Fix the compile_symbol function by removing duplicate patterns

# Read the file
$content = Get-Content "jue_world/src/integration/physics.rs" -Raw

# Fix the compile_symbol function - remove duplicate patterns and fix the logic
$oldFunction = @'
    /// Compile symbol to bytecode - FIXED: Type-aware symbol resolution
    fn compile_symbol(&mut self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
        let opcode = match name {
            // Integer arithmetic operations
            "add" => OpCode::Add,
            "sub" => OpCode::Sub,
            "mul" => OpCode::Mul,
            "div" => OpCode::Div,
            "mod" => OpCode::Mod,
            // Float arithmetic operations - TESTS EXPECT THESE FOR FLOAT OPS
            "fadd" => OpCode::FAdd,
            "fsub" => OpCode::FSub,
            "fmul" => OpCode::FMul,
            "fdiv" => OpCode::FDiv,
            // Comparison operations
            "eq" => OpCode::Eq,
            "=" => OpCode::Eq, // Add = symbol for equality
            "lt" => OpCode::Lt,
            "<" => OpCode::Lt, // Add literal < symbol
            "gt" => OpCode::Gt,
            "<" => OpCode::Lt, // Add literal < symbol
            "gt" => OpCode::Gt,
            "gt" => OpCode::Gt,
            "lte" => OpCode::Lte,
            "<=" => OpCode::Lte, // Add literal <= symbol
            "gte" => OpCode::Gte,
            ">=" => OpCode::Gte, // Add literal >= symbol
            "ne" => OpCode::Ne,
            "!=" => OpCode::Ne, // Add literal != symbol
            // Arithmetic operators (symbol form)
            "+" => OpCode::Add,
            "-" => OpCode::Sub,
            "*" => OpCode::Mul,
            "/" => OpCode::Div,
            "%" => OpCode::Mod,
            // Additional arithmetic symbols
            "mod" => OpCode::Mod,
            // String operations
            "str-concat" => OpCode::StrConcat,
            "str-len" => OpCode::StrLen,
            "str-index" => OpCode::StrIndex,
            // Variable and stack operations
            "swap" => OpCode::Swap,
            "dup" => OpCode::Dup,
            "pop" => OpCode::Pop,
            // Control flow
            "call" => OpCode::Call(0), // Will be overridden with arg count in actual calls
            "tail-call" => OpCode::TailCall(0), // Will be overridden with arg count
            "ret" => OpCode::Ret,
            "jmp" => OpCode::Jmp(0), // Will be overridden with jump target
            "jmp-if-false" => OpCode::JmpIfFalse(0), // Will be overridden with jump target
            // Actor operations
            "yield" => OpCode::Yield,
            "send" => OpCode::Send,
            // Closure operations
            "make-closure" => OpCode::MakeClosure(0, 0), // Will be overridden
            // List operations
            "cons" => OpCode::Cons,
            "car" => OpCode::Car,
            "cdr" => OpCode::Cdr,
            // Resource management
            "check-step-limit" => OpCode::CheckStepLimit,
            // Sandbox operations
            "init-sandbox" => OpCode::InitSandbox,
            "isolate-capabilities" => OpCode::IsolateCapabilities,
            "set-error-handler" => OpCode::SetErrorHandler(0), // Will be overridden
            "log-sandbox-violation" => OpCode::LogSandboxViolation,
            "cleanup-sandbox" => OpCode::CleanupSandbox,
            _ => {
                return Err(CompilationError::InternalError(format!(
                    "Unknown symbol '{}' for Physics-World compilation",
                    name
                )))
            }
        };

        Ok(vec![opcode])
    }'@

$newFunction = @'
    /// Compile symbol to bytecode - FIXED: Type-aware symbol resolution
    fn compile_symbol(&mut self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
        let opcode = match name {
            // Integer arithmetic operations
            "add" => OpCode::Add,
            "sub" => OpCode::Sub,
            "mul" => OpCode::Mul,
            "div" => OpCode::Div,
            "mod" => OpCode::Mod,
            // Float arithmetic operations - TESTS EXPECT THESE FOR FLOAT OPS
            "fadd" => OpCode::FAdd,
            "fsub" => OpCode::FSub,
            "fmul" => OpCode::FMul,
            "fdiv" => OpCode::FDiv,
            // Comparison operations
            "eq" => OpCode::Eq,
            "=" => OpCode::Eq, // Add = symbol for equality
            "lt" => OpCode::Lt,
            "<" => OpCode::Lt, // Add literal < symbol
            "gt" => OpCode::Gt,
            ">" => OpCode::Gt, // Add literal > symbol
            "lte" => OpCode::Lte,
            "<=" => OpCode::Lte, // Add literal <= symbol
            "gte" => OpCode::Gte,
            ">=" => OpCode::Gte, // Add literal >= symbol
            "ne" => OpCode::Ne,
            "!=" => OpCode::Ne, // Add literal != symbol
            // Arithmetic operators (symbol form)
            "+" => OpCode::Add,
            "-" => OpCode::Sub,
            "*" => OpCode::Mul,
            "/" => OpCode::Div,
            "%" => OpCode::Mod,
            // String operations
            "str-concat" => OpCode::StrConcat,
            "str-len" => OpCode::StrLen,
            "str-index" => OpCode::StrIndex,
            // Variable and stack operations
            "swap" => OpCode::Swap,
            "dup" => OpCode::Dup,
            "pop" => OpCode::Pop,
            // Control flow
            "call" => OpCode::Call(0), // Will be overridden with arg count in actual calls
            "tail-call" => OpCode::TailCall(0), // Will be overridden with arg count
            "ret" => OpCode::Ret,
            "jmp" => OpCode::Jmp(0), // Will be overridden with jump target
            "jmp-if-false" => OpCode::JmpIfFalse(0), // Will be overridden with jump target
            // Actor operations
            "yield" => OpCode::Yield,
            "send" => OpCode::Send,
            // Closure operations
            "make-closure" => OpCode::MakeClosure(0, 0), // Will be overridden
            // List operations
            "cons" => OpCode::Cons,
            "car" => OpCode::Car,
            "cdr" => OpCode::Cdr,
            // Resource management
            "check-step-limit" => OpCode::CheckStepLimit,
            // Sandbox operations
            "init-sandbox" => OpCode::InitSandbox,
            "isolate-capabilities" => OpCode::IsolateCapabilities,
            "set-error-handler" => OpCode::SetErrorHandler(0), // Will be overridden
            "log-sandbox-violation" => OpCode::LogSandboxViolation,
            "cleanup-sandbox" => OpCode::CleanupSandbox,
            _ => {
                return Err(CompilationError::InternalError(format!(
                    "Unknown symbol '{}' for Physics-World compilation",
                    name
                )))
            }
        };

        Ok(vec![opcode])
    }'@

# Replace the function
$content = $content.Replace($oldFunction, $newFunction)

# Write back the file
Set-Content "jue_world/src/integration/physics.rs" $content

Write-Host "Fixed duplicate patterns in compile_symbol function"