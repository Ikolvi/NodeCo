# NodeCo Language Specification

## 1. Format
- NodeCo uses a custom binary format with the `.kbj` extension.
- The format is not human-readable and is designed for efficient AI parsing and execution.

### File Structure
- **Magic Number**: 3 bytes (`0x4E434F` for "NCO")
- **Version**: 1 byte (e.g., `0x01`)
- **Instructions**: Sequence of instructions, each encoded as:
  - 1 byte: opcode
  - N bytes: operands (length depends on opcode)

### Opcodes
| Opcode | Meaning  | Operand(s)                |
|--------|----------|---------------------------|
| 0x01   | Let      | var_id (1 byte), value (1 byte) |
| 0x02   | Assign   | var_id (1 byte), value (1 byte) |
| 0x03   | Print    | var_id (1 byte)           |
| 0x04   | Add      | dest_id (1 byte), src1_id (1 byte), src2_id (1 byte) |
| 0x10   | Create UI Element | element_type (1 byte), element_id (1 byte), property_count (1 byte), [property_id, value]... |
| 0x11   | Set UI Property   | element_id (1 byte), property_id (1 byte), value (1 byte) |
| 0x12   | On UI Event       | element_id (1 byte), event_type (1 byte), handler_id (1 byte) |
| 0x13   | Show UI           | (no operands) |
| 0x20   | Check for Update  | (no operands) |
| 0x21   | Apply Update      | (no operands) |

#### UI Element Types
- 0x01: Button
- 0x02: Label
- 0x03: Input
- 0x04: Layout (container)

#### UI Property IDs
- 0x01: Text
- 0x02: Width
- 0x03: Height
- 0x04: Color

#### UI Event Types
- 0x01: Click
- 0x02: Change

### Example: Advanced UI Program
- Create a vertical layout with a label, input, and button.
- When the input changes, update the label.
- When the button is clicked, print the input value.

#### Pseudocode
```
create_ui_element(Layout, 0, 0, []) // vertical layout
create_ui_element(Label, 1, 1, [Text, "Hello"])
create_ui_element(Input, 2, 1, [Text, "Type here"])
create_ui_element(Button, 3, 1, [Text, "OK"])
on_ui_event(2, Change, handler_update_label)
on_ui_event(3, Click, handler_print_input)
show_ui()
```

#### Example Hex (simplified, not actual bytes)
```
4E 43 4F 01
10 04 00 00           // Layout (id=0)
10 02 01 01 01 48      // Label (id=1, Text="H")
10 03 01 01 01 54      // Input (id=2, Text="T")
10 01 01 01 01 4F      // Button (id=3, Text="O")
12 02 02 01            // On Change (input 2), handler 1
12 03 01 02            // On Click (button 3), handler 2
13                     // Show UI
```

## 2. Extensibility
- New instructions can be added by defining new opcodes.
- Versioning is handled by the version byte.

## 3. Upgradability
- Programs include a version byte for compatibility.
- The VM and compiler should check this version and support cloud updates.
- Cloud update opcodes (0x20, 0x21) allow the VM to check for and apply updates from a remote server.

---

## NoDCo Text Format (AI-Friendly)

NoDCo now supports a simple, human-readable text format for writing programs. This format is easy to write, read, and share.

### Syntax
- `label <id> "<text>"` — Create a label with the given id and text.
- `show_ui` — Show the UI.
- Lines starting with `#` are comments.

### Example
```
# Hello World in NoDCo Text
label 0 "Hello World"
show_ui
```

### Text to Bytecode Mapping
| Text Command                | Bytecode (Hex)                        |
|-----------------------------|----------------------------------------|
| `label 0 "Hello World"`     | `10 02 00 01 01 48 65 6C ... 64`       |
| `show_ui`                   | `13`                                   |

### Usage
1. Write your program in a `.ndc`, `.nodco`, or `.txt` file.
2. Compile it using the compiler:
   ```
   cargo run --release -- ../path/to/your_program.ndc
   ```
3. The compiler will output a `.kbj` binary file for the VM.

---

## Step-by-Step: Integrate egui (eframe) into NodeCo VM

### 1. Add Dependencies

In your `vm/Cargo.toml`, add:

```toml
[dependencies]
eframe = "0.27"
egui = "0.27"
```

---

### 2. Update the VM to Launch a GUI Window

You’ll need to:
- Store UI elements and their properties in a struct.
- Render them in an egui window.
- Map NodeCo events (like button clicks) to handler logic.

#### Example: Basic egui App Skeleton

In `vm/src/lib.rs`, add:

```rust
pub struct NodeCoGuiApp {
    pub elements: Vec<GuiElement>,
}

pub struct GuiElement {
    pub element_type: u8,
    pub element_id: u8,
    pub properties: Vec<(u8, u8)>,
    pub handler_id: Option<u8>,
}

impl eframe::App for NodeCoGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for el in &self.elements {
            match el.element_type {
                1 => { // Button
                    let text = el.properties.iter().find(|(pid, _)| *pid == 1).map(|(_, v)| format!("{}", v)).unwrap_or("Button".to_string());
                    if egui::Button::new(text).show(ctx).clicked() {
                        if let Some(handler_id) = el.handler_id {
                            println!("[NodeCo] Button {} clicked! Handler: {}", el.element_id, handler_id);
                            // Here you would invoke the NodeCo handler logic
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
```

---

### 3. Parse NodeCo UI Instructions into GUI Elements

In your `execute_program`, instead of just printing, collect UI elements and events:

```rust
pub fn execute_program_gui(program: &Program) {
    let mut elements = Vec::new();
    let mut current_element: Option<GuiElement> = None;

    for instr in &program.instructions {
        match instr {
            Instruction::CreateUI { element_type, element_id, property_count: _, properties } => {
                current_element = Some(GuiElement {
                    element_type: *element_type,
                    element_id: *element_id,
                    properties: properties.clone(),
                    handler_id: None,
                });
            }
            Instruction::OnUIEvent { element_id, event_type, handler_id } => {
                if let Some(ref mut el) = current_element {
                    if el.element_id == *element_id && *event_type == 1 {
                        el.handler_id = Some(*handler_id);
                    }
                }
            }
            Instruction::ShowUI => {
                if let Some(el) = current_element.take() {
                    elements.push(el);
                }
            }
            _ => {}
        }
    }

    let app = NodeCoGuiApp { elements };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("NodeCo GUI", native_options, Box::new(|_cc| Box::new(app))).unwrap();
}
```

---

### 4. Update `main.rs` to Use the GUI Executor

```rust
fn main() {
    let program = vm::parse_kbj_file("../language_spec/examples/gui.kbj");
    match program {
        Ok(prog) => vm::execute_program_gui(&prog),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

---

## Next Steps

- Run `cargo run` in the `vm` directory. You should see a window with a button.
- Clicking the button will print a message to the console (and can later trigger NodeCo bytecode).
