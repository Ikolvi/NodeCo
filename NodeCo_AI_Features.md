# NodeCo AI Features & Prompt Guide

## Syntax Guide for AI

NodeCo is a binary, non-human-readable language, but you can describe programs in a structured pseudocode that maps directly to NodeCo opcodes and binary format. Use the following conventions to help AI generate NodeCo code:

### General Structure
- Each program is a sequence of instructions (opcodes and operands).
- For UI, create elements, set properties, assign event handlers, and show the UI.
- For logic, use let/assign/add/print as needed.

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

### Example: Hello World Button
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

### Example: Add and Print
Pseudocode:
```
let 0 = 5
let 1 = 10
add 2 = 0 + 1
print 2
```
Binary (hex):
```
4E 43 4F 01
01 00 05
01 01 0A
04 02 00 01
03 02
```

### Example: UI with Input and Label
Pseudocode:
```
create_ui_element(Layout, 0, 0, [])
create_ui_element(Label, 1, 1, [Text, "Name"])
create_ui_element(Input, 2, 1, [Text, "Type here"])
create_ui_element(Button, 3, 1, [Text, "Submit"])
on_ui_event(2, Change, handler_update_label)
on_ui_event(3, Click, handler_print_input)
show_ui()
```

---

For more, see the [Language Specification](https://github.com/Ikolvi/NodeCo/blob/main/language_spec/ai_lang_spec.md) and [Example Programs](https://github.com/Ikolvi/NodeCo/tree/main/language_spec/examples). 