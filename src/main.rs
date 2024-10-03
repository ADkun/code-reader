use code_reader::app::App;

fn main() {
    let app = match App::from_env() {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match app.run() {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}
