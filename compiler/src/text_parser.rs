use std::fs;
use std::io::{self, BufRead};

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
            // label <id> "<text>"
            let mut parts = rest.splitn(2, ' ');
            let id_str = parts.next().unwrap_or("");
            let text_part = parts.next().unwrap_or("").trim();
            let id: u8 = id_str.parse().unwrap_or(0);
            let text = if text_part.starts_with('"') && text_part.ends_with('"') {
                &text_part[1..text_part.len()-1]
            } else {
                text_part
            };
            // Opcode for label: 0x10
            bytecode.push(0x10);
            // Length: id (1) + num_props (1) + text (len)
            let len = 1 + 1 + text.len() as u16;
            bytecode.extend_from_slice(&len.to_le_bytes());
            bytecode.push(id);
            bytecode.push(1); // num_props
            bytecode.extend_from_slice(text.as_bytes());
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