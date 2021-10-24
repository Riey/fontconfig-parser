fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example dump -- <conf file path>");
        return;
    }

    let doc =
        fontconfig_parser::parse_document_from_str(&std::fs::read_to_string(&args[1]).unwrap())
            .unwrap();

    println!("{:#?}", doc);
}
