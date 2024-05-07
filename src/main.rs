// https://www.rustadventure.dev/introducing-clap/clap-v4/initializing-a-new-rust-cli-project
fn main() {
    if let Err(e) = rsdukto::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
