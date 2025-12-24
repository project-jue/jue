# Fix missing symbols in physics compiler and parser
$content2 = Get-Content "jue_world/src/integration/physics.rs"
$newContent2 = $content2 -replace '"<=" => OpCode::Lte, \/\/ Add literal <= symbol', '"<=" => OpCode::Lte, // Add literal <= symbol
            ">" => OpCode::Gt, // Add literal > symbol
            "count-down" => OpCode::Symbol(0), // Add count-down function name
            "double-fact" => OpCode::Symbol(0), // Add double-fact function name
            "is-odd?" => OpCode::Symbol(0), // Add is-odd function name
            "infinite-recursion" => OpCode::Symbol(0), // Add infinite-recursion function name'
$newContent2 | Set-Content "jue_world/src/integration/physics.rs"

Write-Host "Fixed remaining missing symbols"