# NodeCo

NodeCo is an AI-friendly, multiplatform, binary programming language designed for seamless AI parsing, code generation, and execution. It supports basic programming constructs, advanced UI creation, event handling, and is easily extensible for future features like cloud updates and mobile support.

## Features
- **AI-first binary format** (.kbj files)
- **Cross-platform VM and compiler** (Windows, with plans for more)
- **Rich UI creation** (buttons, labels, inputs, layouts)
- **Event handling** (click, change, etc.)
- **Styling and layout** (width, height, color, containers)
- **Cloud update and extensibility**
- **Easy integration with AI tools and prompt-based code generation**

## Logo & Icon
The NodeCo logo and application icon are located in the `assets/` folder:
- ![NodeCo Logo](assets/NoDCoLogo.png)
- ![NodeCo Icon](assets/nodeco.ico)

## Documentation
- [Language Specification](https://github.com/yourusername/yourrepo/blob/main/language_spec/ai_lang_spec.md)
- [AI Features & Prompt Guide](https://github.com/yourusername/yourrepo/blob/main/NodeCo_AI_Features.md)
- [Example Programs](https://github.com/yourusername/yourrepo/tree/main/language_spec/examples)

> **Tip:** AI tools can use these HTTP URLs to access up-to-date documentation and examples for NodeCo.

## Quick Start
1. Build the VM and compiler:
   ```sh
   cd vm && cargo build --release
   cd ../compiler && cargo build --release
   ```
2. Run a NodeCo program:
   ```sh
   vm.exe path\to\yourfile.kbj
   ```

## Contributing
See the documentation above for details on extending NodeCo or creating new features. 