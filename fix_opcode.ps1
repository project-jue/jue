# Fix missing Div opcode in bytecode parser
$content = Get-Content "physics_world/src/vm/opcodes/make_closure_fixed.rs"
$newContent = $content -replace '"SetLocal\(1\)" => OpCode::SetLocal\(1\),', '"SetLocal(1)" => OpCode::SetLocal(1),
            "Div" => OpCode::Div, // FIXED: Added Div opcode'
$newContent | Set-Content "physics_world/src/vm/opcodes/make_closure_fixed.rs"

# Add missing symbols in physics compiler
$content2 = Get-Content "jue_world/src/integration/physics.rs"
$newContent2 = $content2 -replace '"lt" => OpCode::Lt,', '"lt" => OpCode::Lt,
            "<" => OpCode::Lt, // Add literal < symbol
            "gt" => OpCode::Gt,'
$newContent2 | Set-Content "jue_world/src/integration/physics.rs"

Write-Host "Fixed missing Div opcode and < symbol"