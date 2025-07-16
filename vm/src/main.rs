fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "../language_spec/examples/gui.kbj"
    };
    let program = vm::parse_kbj_file(file_path);
    match program {
        Ok(prog) => vm::execute_program_gui(&prog),
        Err(e) => eprintln!("Error: {}", e),
    }
} 