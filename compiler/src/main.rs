fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "../language_spec/examples/hello.kbj"
    };
    let program = compiler::parse_kbj_file(file_path);
    match program {
        Ok(prog) => println!("{:#?}", prog),
        Err(e) => eprintln!("Error: {}", e),
    }
}