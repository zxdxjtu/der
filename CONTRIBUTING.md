# Contributing to DER

Thank you for your interest in contributing to DER! This document provides guidelines for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/yourusername/der.git
   cd der
   ```
3. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
4. **Build the project**:
   ```bash
   cargo build --release
   ```
5. **Run tests**:
   ```bash
   cargo test
   ```

## Development Workflow

### Making Changes

1. **Create a new branch** for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the coding guidelines below

3. **Test your changes**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

4. **Commit your changes** with descriptive commit messages:
   ```bash
   git commit -m "Add feature: description of what you added"
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request** on GitHub

### Coding Guidelines

- **Follow Rust conventions**: Use `cargo fmt` and `cargo clippy`
- **Write tests**: All new functionality should include comprehensive tests
- **Document public APIs**: Use Rust doc comments (`///`) for public functions
- **Keep commits atomic**: Each commit should represent one logical change
- **Write descriptive commit messages**: Explain what and why, not just what

### Code Structure

- `src/core/`: Binary format and serialization
- `src/runtime/`: Execution engine and memory management
- `src/compiler/`: AI-to-DER translation
- `src/types/`: Type system and inference
- `src/verification/`: Formal verification
- `src/visualization/`: Human-readable rendering
- `src/tests/`: Test modules

## Types of Contributions

### Bug Reports

When filing a bug report, please include:
- DER version
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Minimal example if possible

### Feature Requests

For new features:
- Describe the use case
- Explain why it would be valuable
- Consider implementation complexity
- Be open to alternative solutions

### Code Contributions

Areas where contributions are welcome:
- **Performance optimizations**: Executor efficiency, memory usage
- **New opcodes**: Additional computational operations
- **Visualization improvements**: Better rendering, interactive tools
- **Documentation**: Examples, tutorials, API docs
- **Platform support**: Additional architectures
- **Testing**: Edge cases, integration tests

## Pull Request Process

1. **Ensure tests pass**: `cargo test` should succeed
2. **Update documentation**: If you change public APIs
3. **Add examples**: For new features
4. **Squash commits**: If you have many small commits
5. **Write a clear PR description**: Explain what and why

## Questions and Discussion

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Discord**: [Link to be added] for real-time chat

## License

By contributing to DER, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be recognized in the project's credits and changelog. Thank you for helping make DER better!