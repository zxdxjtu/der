# DER Frequently Asked Questions

## General Questions

### Q: What is DER?
**A:** DER (Dynamic Execution Representation) is a binary-native programming language designed for AI as the primary programmer. Unlike traditional languages with text syntax, DER programs exist only as binary computational graphs.

### Q: Why are all the files .rs instead of .der?
**A:** Great observation! The `.rs` files are the Rust implementation of the DER runtime, compiler, and tools. The actual DER programs are `.der` binary files. Think of it like:
- `.rs` files = The video player application (written in Rust)
- `.der` files = The video files you play (DER programs)

### Q: Does DER have source code?
**A:** No! This is DER's most radical feature. There is no text syntax, no source files, no grammar. DER programs exist only as binary computational graphs. When you "write" DER, you're either:
1. Using AI to generate the binary from natural language
2. Using APIs to construct the graph programmatically
3. Using visual tools to design the graph

### Q: How do I write a DER program?
**A:** You don't "write" DER in the traditional sense. Instead:
```bash
# Method 1: Natural language
cargo run --bin der compile "print hello world"

# Method 2: Programmatic API (in Rust)
let mut program = Program::new();
// ... build the graph

# Method 3: Visual editor (future tool)
der-visual-editor
```

### Q: Can I see DER source code?
**A:** No, because it doesn't exist! You can:
- View the binary structure with `der disassemble program.der`
- See a visualization with `der visualize program.der`
- Inspect the hex dump with `hexdump -C program.der`

But there's no "source code" to show.

## Technical Questions

### Q: What's in a .der file?
**A:** A DER file contains:
```
- Header (16 bytes): Magic "DER!", version, flags, chunk count
- META chunk: Entry point, capabilities, traits
- IMPL chunk: Computational nodes (the actual program)
- CNST chunk: Constants (strings, numbers, etc.)
- PROF chunk: Formal proofs of correctness
```

### Q: How does execution work?
**A:** DER uses graph traversal:
1. Start at the entry point node
2. Recursively evaluate dependencies
3. Cache results to avoid recomputation
4. Return the final value

### Q: Is DER compiled or interpreted?
**A:** Neither! DER is "binary-native":
- No compilation step (already binary)
- No interpretation of text (no text exists)
- Direct execution of computational graphs

### Q: Can DER interoperate with other languages?
**A:** Yes! Through the Foreign Execution Interface (FXI):
- Call Python, JavaScript, C functions
- Embed DER in other applications
- Compile DER to WASM, LLVM, etc.

## Philosophy Questions

### Q: Why no text syntax?
**A:** Text syntax is for humans. DER is designed for AI:
- No parsing errors
- No syntax debates
- Faster generation
- More precise representation
- Universal (no language barriers)

### Q: How do humans work with DER?
**A:** Through visualization and high-level tools:
- Natural language descriptions
- Visual graph editors
- Automated documentation
- Proof verification

### Q: Is this the future of programming?
**A:** DER represents one possible future where:
- AI does the implementation
- Humans define intent and constraints
- Correctness is mathematically proven
- The gap between idea and execution vanishes

## Practical Questions

### Q: How do I debug DER programs?
**A:** DER provides several debugging tools:
```bash
# Visualize the graph
der visualize program.der

# Trace execution
DER_TRACE=1 der run program.der

# Verify correctness
der verify program.der

# Inspect binary structure
der disassemble program.der
```

### Q: Can I convert existing code to DER?
**A:** Not directly, because DER has no text format to convert to. Instead:
1. Describe the program's intent to AI
2. Use transpilers (future feature)
3. Manually rebuild using DER APIs

### Q: What can I build with DER?
**A:** Anything! DER is Turing-complete:
- Web services (compile to WASM)
- Data processing (native performance)
- AI models (graph-native)
- System tools (via FXI)
- Games (with UI opcodes)

### Q: How do I contribute?
**A:** Many ways to help:
- Build visualization tools
- Create AI translators
- Write optimizers
- Develop debugging tools
- Design new opcodes
- Improve documentation

## Common Misconceptions

### ❌ "DER is just bytecode"
✅ DER is a complete language with its own execution model, type system, and verification engine.

### ❌ "You need to know Rust to use DER"
✅ You only need Rust to modify the DER system itself. Using DER requires no Rust knowledge.

### ❌ "DER is compiled from some source language"
✅ DER is binary-native. There's no source language to compile from.

### ❌ "This is just an academic experiment"
✅ DER is designed for practical AI-driven development with real-world applications.

## Getting Help

- **Documentation**: Read the docs/ directory
- **Examples**: Study examples/ directory
- **Community**: Join our Discord/Forum
- **Issues**: Report on GitHub

Remember: DER challenges everything you know about programming languages. Embrace the paradigm shift!