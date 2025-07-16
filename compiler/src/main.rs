mod text_parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "../language_spec/examples/hello.kbj"
    };

    let is_text = file_path.ends_with(".ndc") || file_path.ends_with(".nodco") || file_path.ends_with(".txt");
    if is_text {
        match text_parser::parse_text_file(file_path) {
            Ok(bytecode) => {
                let out_path = if file_path.ends_with(".txt") {
                    file_path.trim_end_matches(".txt").to_owned() + ".kbj"
                } else if file_path.ends_with(".ndc") {
                    file_path.trim_end_matches(".ndc").to_owned() + ".kbj"
                } else if file_path.ends_with(".nodco") {
                    file_path.trim_end_matches(".nodco").to_owned() + ".kbj"
                } else {
                    String::from("out.kbj")
                };
                std::fs::write(&out_path, &bytecode).expect("Failed to write output file");
                println!("Compiled {} to {} ({} bytes)", file_path, out_path, bytecode.len());
            }
            Err(e) => eprintln!("Text parse error: {}", e),
        }
    } else {
        let program = compiler::parse_kbj_file(file_path);
        match program {
            Ok(prog) => println!("{:#?}", prog),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}