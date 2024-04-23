fn main() {
    if let Err(e) = rsdukto::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
