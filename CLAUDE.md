# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## DER Language Understanding

DER (Dynamic Execution Representation) is the world's first AI-Native programming language designed specifically for AI as the primary programmer. Key concepts:

- **Binary computational graphs** instead of text-based code
- **AI generates programs** from natural language intent
- **Humans interact through visualization** tools
- **Proof-carrying code** with formal correctness guarantees

## Project Structure

```
src/
├── core/           # Binary format, serialization/deserialization
├── runtime/        # Execution engine (executor, memory, async)
├── compiler/       # AI-to-DER translation (intent parser, AI translator)
├── types/          # Type system and inference
├── verification/   # Formal verification and proof generation
├── visualization/  # Human-readable rendering (text, graph)
└── tests/          # Comprehensive test suite
```

## Essential Commands

### Build and Test
```bash
# Build the project
cargo build --release

# Run all tests
cargo test

# Run specific test module
cargo test --lib runtime_tests

# Run examples
cargo run --example hello_world
cargo run --example comprehensive_demo
```

### DER Program Operations
```bash
# Create DER program from natural language
cargo run --bin der compile "add 10 and 20"

# Visualize DER program structure
cargo run --bin der visualize program.der

# Execute DER program
cargo run --bin der execute program.der
```

## Core Architecture

### Binary Format
- **16-byte node structures** with opcode, flags, timestamp, arguments
- **Chunks**: META (metadata), IMPL (nodes), CNST (constants), PROF (proofs)
- **Direct memory mapping** for execution (no parsing phase)

### Execution Model
1. Load binary program
2. Start at entry point node
3. Recursively evaluate dependencies
4. Cache results for efficiency
5. Return final value

### Key Components
- **Node**: Basic computation unit with opcode and arguments
- **Program**: Collection of nodes with entry point
- **Executor**: Graph traversal engine
- **Constants**: Shared values pool

## OpCode Categories

### Value Operations
- `ConstInt`, `ConstFloat`, `ConstString`, `ConstBool`
- `Add`, `Sub`, `Mul`, `Div`, `Mod`
- `Eq`, `Ne`, `Lt`, `Le`, `Gt`, `Ge`

### Memory Operations
- `Alloc`, `Free`, `Load`, `Store`
- `ArrayNew`, `ArrayGet`, `ArraySet`
- `MapNew`, `MapGet`, `MapSet`

### Control Flow
- `If`, `Loop`, `Call`, `Return`
- `AsyncBegin`, `AsyncComplete`, `AsyncAwait`

### I/O Operations
- `Print`, `Read`, `FileOpen`, `FileRead`, `FileWrite`

## Type System

DER uses automatic type inference with these types:
- **Primitives**: `int`, `float`, `string`, `bool`
- **Composites**: `array`, `map`
- **Special**: `memory`, `async`, `nil`

## Verification System

Programs include formal proofs with:
- **Traits**: Program properties (IsPure, IsTerminating, IsSafe)
- **Preconditions**: Input requirements
- **Postconditions**: Output guarantees
- **Invariants**: Maintained throughout execution

## Working with DER Programs

### Understanding Binary Files
- `.der` files contain the binary computational graph
- `.ders` files contain semantic annotations (when present)
- Use visualization tools to understand program structure

### Creating Programs
1. **Manual construction**: Build nodes programmatically
2. **AI generation**: Use natural language compilation
3. **Hybrid approach**: Generate base, then modify

### Debugging
- Use `cargo run --bin der visualize` to see program structure
- Check execution trace with debug flags
- Verify correctness with formal verification tools

## Development Guidelines

### Code Style
- Follow Rust conventions
- Use descriptive variable names
- Include comprehensive error handling
- Add documentation for public APIs

### Testing
- Write unit tests for each module
- Include integration tests for end-to-end workflows
- Test both success and error cases
- Use property-based testing for complex logic

### Performance
- DER programs are designed for efficiency
- Node execution is O(1) for most operations
- Graph traversal is O(V + E)
- Memory usage is O(n) for results

## Common Patterns

### Error Handling
```rust
match executor.execute() {
    Ok(value) => println!("Result: {}", value),
    Err(e) => eprintln!("Execution error: {}", e),
}
```

### Program Construction
```rust
let mut program = Program::new();
let node1 = Node::new(OpCode::ConstInt, 1).with_args(&[10]);
let node2 = Node::new(OpCode::Print, 2).with_args(&[1]);
program.add_node(node1);
let entry = program.add_node(node2);
program.set_entry_point(entry);
```

### Visualization
```rust
let renderer = TextRenderer::new();
renderer.render_program(&program);
```

## Extension Points

### Adding New OpCodes
1. Add to `OpCode` enum in `core/binary_format.rs`
2. Implement execution logic in `runtime/executor.rs`
3. Add type rules in `types/type_checker.rs`
4. Update visualization in `visualization/text_renderer.rs`

### Custom Verification Traits
1. Define trait in `verification/traits.rs`
2. Implement proof generation in `verification/proof.rs`
3. Add verification logic in `verification/verifier.rs`

Remember: DER is AI-Native. Think in terms of computational graphs, not imperative code. The binary format is the source of truth, and visualization helps humans understand the AI's intent.