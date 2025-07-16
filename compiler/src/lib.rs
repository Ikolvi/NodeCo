use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
pub enum Instruction {
    Let { var_id: u8, value: u8 },
    Assign { var_id: u8, value: u8 },
    Print { var_id: u8 },
    Add { dest_id: u8, src1_id: u8, src2_id: u8 },
    Unknown(u8),
}

#[derive(Debug)]
pub struct Program {
    pub version: u8,
    pub instructions: Vec<Instruction>,
}

pub fn parse_kbj_file(path: &str) -> io::Result<Program> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    // Check magic number
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
            op => {
                instructions.push(Instruction::Unknown(op));
                i += 1;
            }
        }
    }
    Ok(Program { version, instructions })
}
