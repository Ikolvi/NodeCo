# NodeCo AI Features & Prompt Guide

## AI-Friendly Text Syntax (Recommended)

NodeCo now supports a simple, human-readable text format for writing programs. This is the preferred way for AI and users to generate NodeCo code.

### General Structure
- Each program is a sequence of instructions in plain text.
- For UI, create elements with key-value properties, assign event handlers, and show the UI.
- For logic, use let/assign/add/print as needed (coming soon to text format).

### Text Syntax
- `label <id> text="<text>"` — Create a label with the given id and text.
- `button <id> text="<text>"` — Create a button with the given id and text.
- `input <id> text="<text>"` — Create an input with the given id and placeholder text.
- `show_ui` — Show the UI.
- Lines starting with `#` are comments.

#### Example: Hello World Label
```
label 0 text="Hello World"
show_ui
```

#### Example: UI with Input and Button
```
label 0 text="Name:"
input 1 text="Type here"
button 2 text="Submit"
show_ui
```

### Property Encoding
String properties are encoded as:
- `[property_count][property_id][string_length][string_bytes]...`
- For example, `label 0 text="Hello"` encodes as:
  - `0x10 0x01 0x00 0x01 0x01 0x05 48 65 6C 6C 6F` (opcode, element_type, id, property_count, property_id, length, bytes)

### Usage
1. Write your program in a `.ndc`, `.nodco`, or `.txt` file.
2. Compile it using the compiler:
   ```
   cargo run --release -- ../path/to/your_program.ndc
   ```
3. The compiler will output a `.kbj` binary file for the VM.
4. Run the VM with the `.kbj` file to see your UI.

---

## Legacy: Binary & Pseudocode (for reference)

NodeCo is a binary, non-human-readable language, but you can describe programs in a structured pseudocode that maps directly to NodeCo opcodes and binary format. This is now considered legacy; use the text format above for new code.

### Pseudocode Syntax
- `let <var_id> = <value>` → `0x01 <var_id> <value>`
- `assign <var_id> = <value>` → `0x02 <var_id> <value>`
- `add <dest_id> = <src1_id> + <src2_id>` → `0x04 <dest_id> <src1_id> <src2_id>`
- `print <var_id>` → `0x03 <var_id>`
- `create_ui_element(<type>, <id>, <property_count>, [<property_id>, <value>]...)` → `0x10 <type> <id> <property_count> ...`
- `set_ui_property(<id>, <property_id>, <value>)` → `0x11 <id> <property_id> <value>`
- `on_ui_event(<id>, <event_type>, <handler_id>)` → `0x12 <id> <event_type> <handler_id>`
- `show_ui()` → `0x13`

### UI Types
- Button: 1
- Label: 2
- Input: 3
- Layout: 4

### Property IDs
- Text: 1
- Width: 2
- Height: 3
- Color: 4

### Event Types
- Click: 1
- Change: 2

### Example: Hello World Button (Legacy)
Pseudocode:
```
create_ui_element(Button, 0, 1, [Text, "Hello World"])
on_ui_event(0, Click, handler_print_hello)
show_ui()
```
Binary (hex):
```
4E 43 4F 01
10 01 00 01 01 48   // Create Button (id=0), 1 property: Text="H"
12 00 01 01          // On Click (element 0), handler 1
13                   // Show UI
```

---

For more, see the [Language Specification](language_spec/ai_lang_spec.md) and [Example Programs](language_spec/examples). 