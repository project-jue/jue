state.rs is getting large, see if we can split it up.
Maybe each op handling can be put in it's own file.

For splitting a large Rust match statement across multiple files while maintaining access to `self`, here are several good approaches:

## 1. **Method Dispatch Pattern** (Most Recommended)

Create a trait for opcode handlers and implement it in separate modules:

```rust
// vm.rs
mod opcode_handlers;

impl VM {
    pub fn execute(&mut self, opcode: Opcode) -> Result<()> {
        match opcode {
            Opcode::Add => self.handle_add(),
            Opcode::Sub => self.handle_sub(),
            // ... other opcodes
        }
    }
    
    // Optionally, delegate to module functions
    fn handle_add(&mut self) -> Result<()> {
        opcode_handlers::add::execute(self)
    }
}
```

```rust
// opcode_handlers/add.rs
use crate::VM;

pub fn execute(vm: &mut VM) -> Result<()> {
    // Implementation here with access to vm
    vm.registers[0] = vm.registers[1] + vm.registers[2];
    Ok(())
}
```

```rust
// opcode_handlers/mod.rs
pub mod add;
pub mod sub;
// ... other opcodes
```

## 2. **Function Table Pattern**

Create a mapping from opcodes to handler functions:

```rust
// vm.rs
mod opcode_handlers;

impl VM {
    pub fn execute(&mut self, opcode: Opcode) -> Result<()> {
        let handler = HANDLERS.get(&opcode).ok_or(Error::UnknownOpcode)?;
        handler(self)
    }
}

type HandlerFn = fn(&mut VM) -> Result<()>;

lazy_static! {
    static ref HANDLERS: HashMap<Opcode, HandlerFn> = {
        let mut m = HashMap::new();
        m.insert(Opcode::Add, opcode_handlers::add::execute);
        m.insert(Opcode::Sub, opcode_handlers::sub::execute);
        // ...
        m
    };
}
```

## 3. **Macro-Based Approach** (If you need compile-time dispatch)

```rust
// vm.rs
macro_rules! dispatch_opcode {
    ($self:expr, $opcode:expr) => {
        match $opcode {
            Opcode::Add => include!("opcodes/add.rs")($self),
            Opcode::Sub => include!("opcodes/sub.rs")($self),
            // ...
        }
    };
}

impl VM {
    pub fn execute(&mut self, opcode: Opcode) -> Result<()> {
        dispatch_opcode!(self, opcode)
    }
}
```

```rust
// opcodes/add.rs
|vm: &mut VM| -> Result<()> {
    // Implementation
    Ok(())
}
```

## 4. **Visitor Pattern** (Good for complex opcode hierarchies)

```rust
// opcode_visitor.rs
pub trait OpcodeVisitor<T> {
    fn visit_add(&mut self, vm: &mut VM) -> T;
    fn visit_sub(&mut self, vm: &mut VM) -> T;
    // ...
}

// In separate files:
// add_visitor.rs
pub struct AddVisitor;
impl OpcodeVisitor<Result<()>> for AddVisitor {
    fn visit_add(&mut self, vm: &mut VM) -> Result<()> {
        // Implementation
        Ok(())
    }
    // ... other methods with default implementations or panic
}
```

## 5. **Simple Modular Function Approach** (Easiest to implement)

```rust
// vm.rs
mod opcodes;

impl VM {
    pub fn execute(&mut self, opcode: Opcode) -> Result<()> {
        match opcode {
            Opcode::Add => opcodes::add(self),
            Opcode::Sub => opcodes::sub(self),
            // ...
        }
    }
}
```

```rust
// opcodes/add.rs
use crate::VM;

pub fn add(vm: &mut VM) -> Result<()> {
    // Direct implementation with access to vm
    Ok(())
}
```

## **Recommendation**

For your use case (LLM-friendly, small isolated files), I recommend **Option 5** (Simple Modular Function Approach) because:

1. **Each opcode gets its own file** - perfect for LLM editing
2. **Simple to understand and maintain**
3. **Direct access to `&mut VM`** in each handler
4. **No complex abstractions** - just plain functions
5. **Easy to revert changes** if LLM makes mistakes

## **Project Structure Example:**

```
src/
├── vm.rs          # Main VM struct and execute method
├── opcodes/
│   ├── mod.rs     # Exports all opcode handlers
│   ├── add.rs     # fn add(vm: &mut VM) -> Result<()>
│   ├── sub.rs     # fn sub(vm: &mut VM) -> Result<()>
│   ├── mul.rs
│   └── ... (dozens more)
└── main.rs
```

## **Automating the Migration**

Since you have thousands of lines, write a script to:
1. Extract each match arm into its own file
2. Generate the `mod.rs` exports
3. Update the main match to call the functions

This keeps the logic identical but organized into smaller, LLM-friendly files.