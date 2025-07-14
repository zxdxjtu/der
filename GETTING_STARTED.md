# Getting Started with DER

## Overview

DER (Dynamic Execution Representation) is a groundbreaking programming language where:
- **AI is the primary programmer** - generating binary computational graphs
- **Humans interact through visualization** - no text syntax to learn
- **Programs are mathematical proofs** - correctness is built-in

## System Requirements

- Rust 1.70 or higher
- 8GB RAM recommended
- Any OS (Windows, macOS, Linux)

## Installation

```bash
# 1. Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clone the DER repository
git clone https://github.com/your-org/der.git
cd der

# 3. Build the project
cargo build --release

# 4. Run tests to verify installation
cargo test
```

## Creating Your First Program

### Example 1: Hello World

Here's how DER's Hello World works internally:

```
Program Structure:
┌─────────────────┐
│ Constants Pool  │
│ [0] "Hello,     │
│      World!"    │
└─────────────────┘
        ↓
┌─────────────────┐
│ Node 1          │
│ OpCode: ConstStr│
│ Args: [0]       │
│ Result: str_ref │
└─────────────────┘
        ↓
┌─────────────────┐
│ Node 2          │
│ OpCode: Print   │
│ Args: [1]       │
│ Result: nil     │
└─────────────────┘
```

To create this programmatically:

```rust
use der::core::*;
use der::runtime::*;

// Create program
let mut program = Program::new();

// Add string to constants
let idx = program.constants.add_string("Hello, World!".to_string());

// Create nodes
let str_node = Node::new(OpCode::ConstString, 1).with_args(&[idx]);
let print_node = Node::new(OpCode::Print, 2).with_args(&[1]);

// Build program
program.add_node(str_node);
let entry = program.add_node(print_node);
program.set_entry_point(entry);

// Execute
let mut executor = Executor::new(program);
executor.execute().unwrap(); // Prints: Hello, World!
```

### Example 2: Basic Arithmetic

Let's create a program that calculates: (10 + 20) * 3

```
Program Structure:
┌─────────────────┐
│ Constants       │
│ [0] 10          │
│ [1] 20          │
│ [2] 3           │
└─────────────────┘
    ↓       ↓
┌─────┐ ┌─────┐
│ N1  │ │ N2  │
│ 10  │ │ 20  │
└─────┘ └─────┘
    ↓     ↓
┌─────────────┐
│ N3: Add     │
│ Args: [1,2] │
│ Result: 30  │
└─────────────┘
        ↓
    ┌─────┐
    │ N4  │
    │  3  │
    └─────┘
        ↓
┌─────────────┐
│ N5: Mul     │
│ Args: [3,4] │
│ Result: 90  │
└─────────────┘
```

### Example 3: Using AI to Generate Programs

DER's killer feature is AI generation:

```bash
# Natural language to DER
cargo run --bin der compile "add 10 and 20"

# More complex examples
cargo run --bin der compile "create an array with values 1, 2, 3"
cargo run --bin der compile "multiply 5 by 8 and print the result"
```

## Understanding DER Programs

### Binary Format

DER programs are stored as binary files with:
- **Header**: Magic number "DER!", version, chunk count
- **Chunks**: META (metadata), IMPL (nodes), CNST (constants), PROF (proofs)
- **Nodes**: 16-byte structures with opcode, flags, timestamp, arguments

### Execution Model

1. **Graph Traversal**: Start at entry point, recursively evaluate dependencies
2. **Value Caching**: Each node's result is computed once and cached
3. **Type Safety**: Runtime type checking ensures correctness
4. **Memory Safety**: Reference counting prevents leaks

### Visualization

View any DER program's structure:

```bash
cargo run --bin der visualize program.der
```

Outputs:
- Text tree representation
- DOT file for Graphviz
- Program statistics

## Advanced Features

### Memory Management

```rust
// Allocate 100 bytes
let size = Node::new(OpCode::ConstInt, 1).with_args(&[size_100]);
let alloc = Node::new(OpCode::Alloc, 2).with_args(&[1]);

// Store value
let store = Node::new(OpCode::Store, 3).with_args(&[2, value]);

// Load value
let load = Node::new(OpCode::Load, 4).with_args(&[2]);

// Free memory (automatic with ref counting)
let free = Node::new(OpCode::Free, 5).with_args(&[2]);
```

### Async Operations

```rust
// Start async task
let async_op = Node::new(OpCode::AsyncBegin, 1);

// Do work...

// Complete async
let complete = Node::new(OpCode::AsyncComplete, 2).with_args(&[1, result]);

// Await result
let await_node = Node::new(OpCode::AsyncAwait, 3).with_args(&[1]);
```

### Formal Verification

```rust
// Add program traits
program.metadata.traits.push(Trait {
    name: "IsPure".to_string(),
    preconditions: vec!["No side effects".to_string()],
    postconditions: vec!["Deterministic result".to_string()],
});

// Verify program
let verifier = Verifier::new(program);
let result = verifier.verify_program();
assert!(result.is_valid);
```

## Best Practices

### For AI Systems

1. **Generate Proofs**: Always include correctness proofs with generated code
2. **Use Type Information**: Leverage DER's type system for safety
3. **Minimize Graph Size**: Reuse nodes when possible
4. **Add Metadata**: Include traits and capabilities

### For Human Developers

1. **Think in Graphs**: Visualize data flow, not control flow
2. **Use Tools**: Leverage visualizers and verifiers
3. **Start Simple**: Build complex programs from simple components
4. **Test Thoroughly**: Use DER's verification engine

## Common Patterns

### Map-Reduce
```rust
// Map: double each element
let doubled = array.iter().map(|x| x * 2);

// Reduce: sum all elements  
let sum = doubled.fold(0, |acc, x| acc + x);
```

### Error Handling
```rust
// DER uses Result types
match executor.execute() {
    Ok(value) => println!("Success: {}", value),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Resource Management
```rust
// RAII pattern with automatic cleanup
{
    let mem = allocate(100);
    // Use memory...
} // Automatically freed
```

## Troubleshooting

### Common Errors

1. **"Invalid opcode"**: Check OpCode enum for valid operations
2. **"Type mismatch"**: Ensure operation arguments have correct types  
3. **"Node not found"**: Verify all referenced nodes exist
4. **"Cycle detected"**: DER doesn't allow circular dependencies

### Performance Tips

- Pre-compute constants
- Use async for I/O operations
- Minimize memory allocations
- Cache frequently used values

## Next Steps

1. **Explore Examples**: Study programs in `examples/`
2. **Read Architecture**: Understand DER's design in `docs/`
3. **Build Tools**: Create custom visualizers or optimizers
4. **Contribute**: Improve DER's ecosystem

## Resources

- [Design Philosophy](docs/philosophy.md)
- [Binary Format Spec](docs/der-binary-spec.md)
- [API Reference](docs/api-reference.md)
- [Contributing Guide](CONTRIBUTING.md)

Happy coding with DER! 🚀