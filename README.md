# NodeCo

<p align="center">
  <img src="assets/logo.png" alt="NodeCo" style="width:100%;max-width:320px;"/>
</p>

NodeCo is an AI-friendly, multiplatform, binary programming language designed for seamless AI parsing, code generation, and execution. It supports basic programming constructs, advanced UI creation, event handling, and is easily extensible for future features like cloud updates and mobile support.

## Features
- **AI-first binary format** (.kbj files)
- **Cross-platform VM and compiler** (Windows, with plans for more)
- **Rich UI creation** (buttons, labels, inputs, layouts)
- **Event handling** (click, change, etc.)
- **Styling and layout** (width, height, color, containers)
- **Cloud update and extensibility**
- **Easy integration with AI tools and prompt-based code generation**

## Documentation
- [Language Specification](https://github.com/Ikolvi/NodeCo/blob/main/language_spec/ai_lang_spec.md)
- [AI Features & Prompt Guide](https://github.com/Ikolvi/NodeCo/blob/main/NodeCo_AI_Features.md)
- [Example Programs](https://github.com/Ikolvi/NodeCo/tree/main/language_spec/examples)

> **Tip:** AI tools can use these HTTP URLs to access up-to-date documentation and examples for NodeCo.

## Quick Start
1. Build the VM and compiler:
   ```sh
   cd vm && cargo build --release
   cd ../compiler && cargo build --release
   ```
2. Run a NodeCo program:
   ```sh
   ./vm [path/to/yourfile.kbj]
   ```
   or
   ```sh
   ./compiler [path/to/yourfile.kbj]
   ```
   (Use `./vm` or `./compiler` on Unix/macOS, or `vm.exe`/`compiler.exe` on Windows.)

## Contributing
We welcome contributions from everyone! To get started:

- Fork the repository and create your branch from `main`.
- If you’ve added code, add tests.
- If you’ve changed APIs, update the documentation.
- Ensure your code builds and passes all tests.
- Open a pull request with a clear description of your changes.

### Pull Requests
- Please keep pull requests focused and atomic.
- Reference related issues in your PR description.
- Use clear, descriptive commit messages.
- Be respectful and constructive in code reviews and discussions.

For more details, see the [CONTRIBUTING.md](https://github.com/Ikolvi/NodeCo/blob/main/CONTRIBUTING.md) if available, or open an issue to ask questions. 

# NoDCo Text Format (AI-Friendly)

You can now write NoDCo programs in a simple, human-readable text format and compile them to binary for the VM.

## Example

```
# Hello World in NoDCo Text
label 0 text="Hello World"
show_ui
```

Save this as `hello.ndc`, `hello.nodco`, or `hello.txt`.

## Compiling

Run the compiler with your text file:

```
cd compiler
cargo run --release -- ../path/to/hello.ndc
```

This will produce `hello.kbj` in the same directory.

## Supported Commands
- `label <id> text="<text>"` — Create a label with the given id and text.
- `show_ui` — Show the UI.

## String Properties
You can specify string properties for UI elements using key-value syntax, e.g., `text="Hello World"`. The compiler and VM now support string properties for labels and other UI elements.

More commands and properties will be added as the language evolves! 