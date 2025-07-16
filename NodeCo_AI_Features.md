# NodeCo AI Features & Prompt Guide

## What is NodeCo?
NodeCo is an AI-friendly, binary, multiplatform programming language designed for easy parsing and code generation by AI. It supports basic programming constructs, UI creation, event handling, and is easily extensible.

## Core Features
- Binary, non-human-readable format (.kbj)
- Variables, arithmetic, assignment, print
- UI creation: buttons, labels, inputs, layouts
- Event handling: click, change
- Styling: width, height, color (extensible)
- Cloud update and extensibility support

## Supported Instructions (Opcodes)
| Opcode | Name                | Description/Operands |
|--------|---------------------|---------------------|
| 0x01   | Let                 | var_id, value       |
| 0x02   | Assign              | var_id, value       |
| 0x03   | Print               | var_id              |
| 0x04   | Add                 | dest_id, src1_id, src2_id |
| 0x10   | Create UI Element   | element_type, element_id, property_count, [property_id, value]... |
| 0x11   | Set UI Property     | element_id, property_id, value |
| 0x12   | On UI Event         | element_id, event_type, handler_id |
| 0x13   | Show UI             | (none)              |
| 0x20   | Check for Update    | (none)              |
| 0x21   | Apply Update        | (none)              |

### UI Element Types
- 0x01: Button
- 0x02: Label
- 0x03: Input
- 0x04: Layout (vertical container)

### UI Property IDs
- 0x01: Text
- 0x02: Width
- 0x03: Height
- 0x04: Color

### UI Event Types
- 0x01: Click
- 0x02: Change

## Example Prompts & Outputs

### Prompt: Write a hello world program in NodeCo
**AI Output:**
- Create a button with text "Hello World"
- On button click, print 42
- Show the UI

**NodeCo Pseudocode:**
```
create_ui_element(Button, 0, 1, [Text, "Hello World"])
on_ui_event(0, Click, handler_print_hello)
show_ui()
```

**NodeCo .kbj Hex (example):**
```
4E 43 4F 01
10 01 00 01 01 48   // Create Button (id=0), 1 property: Text="H"
12 00 01 01          // On Click (element 0), handler 1
13                   // Show UI
```

### Prompt: Create a form with a label, input, and submit button in NodeCo
**AI Output:**
- Create a vertical layout with a label, input, and button
- On input change, update the label
- On button click, print the input value
- Show the UI

**NodeCo Pseudocode:**
```
create_ui_element(Layout, 0, 0, [])
create_ui_element(Label, 1, 1, [Text, "Name"])
create_ui_element(Input, 2, 1, [Text, "Type here"])
create_ui_element(Button, 3, 1, [Text, "Submit"])
on_ui_event(2, Change, handler_update_label)
on_ui_event(3, Click, handler_print_input)
show_ui()
```

## How to Extend
- Add new opcodes for more UI elements, events, or logic
- Use the version byte for compatibility
- See `language_spec/ai_lang_spec.md` for full details 