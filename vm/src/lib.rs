use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    Let { var_id: u8, value: u8 },
    Assign { var_id: u8, value: u8 },
    Print { var_id: u8 },
    Add { dest_id: u8, src1_id: u8, src2_id: u8 },
    CreateUI { element_type: u8, element_id: u8, property_count: u8, properties: Vec<(u8, u8)> },
    SetUIProperty { element_id: u8, property_id: u8, value: u8 },
    OnUIEvent { element_id: u8, event_type: u8, handler_id: u8 },
    ShowUI,
    CheckForUpdate,
    ApplyUpdate,
    Unknown(u8),
}

#[derive(Debug, Clone)]
pub struct GuiElement {
    pub element_type: u8,
    pub element_id: u8,
    pub properties: Vec<(u8, u8)>,
    pub string_properties: HashMap<u8, String>,
    pub handler_id: Option<u8>,
    pub children: Vec<GuiElement>,
    pub value: Option<String>, // for input state
}

pub struct NodeCoGuiApp {
    pub elements: Vec<GuiElement>,
    pub input_states: std::collections::HashMap<u8, String>,
    pub label_states: std::collections::HashMap<u8, String>,
}

impl eframe::App for NodeCoGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let elements = &mut self.elements;
        let input_states = &mut self.input_states;
        let label_states = &mut self.label_states;
        egui::CentralPanel::default().show(ctx, |ui| {
            for el in elements {
                render_element(ui, el, input_states, label_states);
            }
        });
    }
}

fn render_element(
    ui: &mut egui::Ui,
    el: &mut GuiElement,
    input_states: &mut std::collections::HashMap<u8, String>,
    label_states: &mut std::collections::HashMap<u8, String>,
) {
    match el.element_type {
        1 => { // Button
            let text = el.string_properties.get(&1)
                .cloned()
                .unwrap_or_else(|| el.properties.iter().find(|(pid, _)| *pid == 1)
                    .map(|(_, v)| format!("{}", v))
                    .unwrap_or("Button".to_string()));
            if ui.button(text).clicked() {
                if let Some(handler_id) = el.handler_id {
                    println!("[NodeCo] Button {} clicked! Handler: {}", el.element_id, handler_id);
                }
            }
        }
        2 => { // Label
            let text = el.string_properties.get(&1)
                .cloned()
                .unwrap_or_else(|| label_states.get(&el.element_id).cloned().unwrap_or_else(||
                    el.properties.iter().find(|(pid, _)| *pid == 1)
                        .map(|(_, v)| format!("{}", v))
                        .unwrap_or("Label".to_string())
                ));
            ui.label(text);
        }
        3 => { // Input
            let entry = input_states.entry(el.element_id).or_insert_with(|| {
                el.string_properties.get(&1)
                    .cloned()
                    .unwrap_or_else(|| el.properties.iter().find(|(pid, _)| *pid == 1)
                        .map(|(_, v)| format!("{}", v))
                        .unwrap_or(String::new()))
            });
            let response = ui.text_edit_singleline(entry);
            if response.changed() {
                if let Some(handler_id) = el.handler_id {
                    println!("[NodeCo] Input {} changed! Handler: {}", el.element_id, handler_id);
                    label_states.insert(1, entry.clone());
                }
            }
        }
        4 => { // Layout (vertical)
            ui.vertical(|ui| {
                for child in &mut el.children {
                    render_element(ui, child, input_states, label_states);
                }
            });
        }
        _ => {}
    }
}

pub struct Program {
    pub version: u8,
    pub instructions: Vec<Instruction>,
}

pub fn execute_program_gui(program: &Program) {
    // For demo: build a vertical layout with label, input, button
    let mut elements = Vec::new();
    let mut id_to_element: std::collections::HashMap<u8, GuiElement> = std::collections::HashMap::new();

    for instr in &program.instructions {
        match instr {
            Instruction::CreateUI { element_type, element_id, property_count: _, properties } => {
                let el = GuiElement {
                    element_type: *element_type,
                    element_id: *element_id,
                    properties: properties.clone(),
                    string_properties: HashMap::new(),
                    handler_id: None,
                    children: Vec::new(),
                    value: None,
                };
                id_to_element.insert(*element_id, el);
            }
            Instruction::OnUIEvent { element_id, event_type, handler_id } => {
                if let Some(el) = id_to_element.get_mut(element_id) {
                    if *event_type == 1 || *event_type == 2 {
                        el.handler_id = Some(*handler_id);
                    }
                }
            }
            Instruction::ShowUI => {
                // For demo, build a vertical layout (id=0) with children 1,2,3
                if let Some(mut layout) = id_to_element.remove(&0) {
                    for cid in 1..=3 {
                        if let Some(child) = id_to_element.remove(&(cid as u8)) {
                            layout.children.push(child);
                        }
                    }
                    elements.push(layout);
                }
            }
            _ => {}
        }
    }

    let app = NodeCoGuiApp {
        elements,
        input_states: std::collections::HashMap::new(),
        label_states: std::collections::HashMap::new(),
    };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("NodeCo GUI", native_options, Box::new(|_cc| Box::new(app))).unwrap();
}

// --- END GUI SUPPORT ---

pub fn parse_kbj_file(path: &str) -> io::Result<Program> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    if buf.len() < 4 || buf[0] != 0x4E || buf[1] != 0x43 || buf[2] != 0x4F {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic number"));
    }
    let version = buf[3];
    let mut instructions = Vec::new();
    let mut i = 4;
    while i < buf.len() {
        match buf[i] {
            0x01 => {
                if i + 2 < buf.len() {
                    instructions.push(Instruction::Let { var_id: buf[i+1], value: buf[i+2] });
                    i += 3;
                } else { break; }
            }
            0x02 => {
                if i + 2 < buf.len() {
                    instructions.push(Instruction::Assign { var_id: buf[i+1], value: buf[i+2] });
                    i += 3;
                } else { break; }
            }
            0x03 => {
                if i + 1 < buf.len() {
                    instructions.push(Instruction::Print { var_id: buf[i+1] });
                    i += 2;
                } else { break; }
            }
            0x04 => {
                if i + 3 < buf.len() {
                    instructions.push(Instruction::Add { dest_id: buf[i+1], src1_id: buf[i+2], src2_id: buf[i+3] });
                    i += 4;
                } else { break; }
            }
            0x10 => {
                if i + 3 < buf.len() {
                    let element_type = buf[i+1];
                    let element_id = buf[i+2];
                    let property_count = buf[i+3];
                    let mut properties = Vec::new();
                    let mut string_properties = HashMap::new();
                    let mut j = 0;
                    let mut prop_idx = i + 4;
                    while j < property_count as usize && prop_idx < buf.len() {
                        let prop_id = buf[prop_idx];
                        if prop_id == 1 && prop_idx + 1 < buf.len() {
                            let str_len = buf[prop_idx + 1] as usize;
                            if prop_idx + 2 + str_len - 1 < buf.len() {
                                let str_bytes = &buf[prop_idx + 2 .. prop_idx + 2 + str_len];
                                if let Ok(s) = std::str::from_utf8(str_bytes) {
                                    string_properties.insert(prop_id, s.to_string());
                                }
                                prop_idx += 2 + str_len;
                            } else {
                                break;
                            }
                        } else if prop_idx + 1 < buf.len() {
                            properties.push((prop_id, buf[prop_idx + 1]));
                            prop_idx += 2;
                        } else {
                            break;
                        }
                        j += 1;
                    }
                    instructions.push(Instruction::CreateUI { element_type, element_id, property_count, properties: properties.clone() });
                    i = prop_idx;
                } else { break; }
            }
            0x11 => {
                if i + 3 < buf.len() {
                    instructions.push(Instruction::SetUIProperty { element_id: buf[i+1], property_id: buf[i+2], value: buf[i+3] });
                    i += 4;
                } else { break; }
            }
            0x12 => {
                if i + 3 < buf.len() {
                    instructions.push(Instruction::OnUIEvent { element_id: buf[i+1], event_type: buf[i+2], handler_id: buf[i+3] });
                    i += 4;
                } else { break; }
            }
            0x13 => {
                instructions.push(Instruction::ShowUI);
                i += 1;
            }
            0x20 => {
                instructions.push(Instruction::CheckForUpdate);
                i += 1;
            }
            0x21 => {
                instructions.push(Instruction::ApplyUpdate);
                i += 1;
            }
            op => {
                instructions.push(Instruction::Unknown(op));
                i += 1;
            }
        }
    }
    Ok(Program { version, instructions })
}

pub fn execute_program(program: &Program) {
    let mut vars = [0u8; 256]; // 256 variables, indexed by var_id

    for instr in &program.instructions {
        match instr {
            Instruction::Let { var_id, value } => {
                vars[*var_id as usize] = *value;
            }
            Instruction::Assign { var_id, value } => {
                vars[*var_id as usize] = *value;
            }
            Instruction::Add { dest_id, src1_id, src2_id } => {
                vars[*dest_id as usize] = vars[*src1_id as usize].wrapping_add(vars[*src2_id as usize]);
            }
            Instruction::Print { var_id } => {
                println!("{}", vars[*var_id as usize]);
            }
            Instruction::CreateUI { element_type, element_id, property_count, properties } => {
                println!("[UI] Create element type={} id={} with {} properties: {:?}", element_type, element_id, property_count, properties);
            }
            Instruction::SetUIProperty { element_id, property_id, value } => {
                println!("[UI] Set property {} of element {} to {}", property_id, element_id, value);
            }
            Instruction::OnUIEvent { element_id, event_type, handler_id } => {
                println!("[UI] On event {} for element {}: handler {}", event_type, element_id, handler_id);
            }
            Instruction::ShowUI => {
                println!("[UI] Show UI");
            }
            Instruction::CheckForUpdate => {
                println!("[Cloud] Check for update");
            }
            Instruction::ApplyUpdate => {
                println!("[Cloud] Apply update");
            }
            Instruction::Unknown(op) => {
                eprintln!("Unknown opcode: {}", op);
            }
        }
    }
}
