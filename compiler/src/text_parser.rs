use std::fs;
use std::io::{self, BufRead};

fn parse_properties(props: &str) -> Vec<u8> {
    let mut out = Vec::new();
    let mut count = 0u8;
    let mut prop_bytes = Vec::new();
    for prop in props.split_whitespace() {
        if let Some((key, value)) = prop.split_once('=') {
            let property_id = match key {
                "text" => 1u8,
                _ => continue, // skip unknown properties for now
            };
            let value = value.trim();
            let value = value.strip_prefix('"').unwrap_or(value);
            let value = value.strip_suffix('"').unwrap_or(value);
            let value_bytes = value.as_bytes();
            prop_bytes.push(property_id);
            prop_bytes.push(value_bytes.len() as u8);
            prop_bytes.extend_from_slice(value_bytes);
            count += 1;
        }
    }
    out.push(count);
    out.extend(prop_bytes);
    out
}

pub fn parse_text_file(path: &str) -> io::Result<Vec<u8>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut bytecode = Vec::new();

    // Header: NCO + version 1
    bytecode.extend_from_slice(&[0x4E, 0x43, 0x4F, 0x01]);

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("label ") {
            // label <id> [props]
            let mut parts = rest.splitn(2, ' ');
            let id_str = parts.next().unwrap_or("");
            let props = parts.next().unwrap_or("");
            let id: u8 = id_str.parse().unwrap_or(0);
            // Opcode for label: 0x10, element_type=1 (label)
            bytecode.push(0x10);
            bytecode.push(1); // element_type: 1 = label
            bytecode.push(id); // element_id
            let prop_bytes = parse_properties(props);
            bytecode.extend_from_slice(&prop_bytes);
        } else if line == "show_ui" {
            // Opcode for show_ui: 0x13
            bytecode.push(0x13);
        } else {
            // Unknown command, ignore or error
            // For now, ignore
        }
    }
    Ok(bytecode)
} 