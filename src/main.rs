use clap::Parser;
use code_reader::app::App;

fn main() {
    let app = App::parse();

    match app.run() {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}
