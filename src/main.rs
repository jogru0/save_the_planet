use std::error::Error;

mod grid;

mod bterm;
mod tui;

mod world;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if true {
        tui::main()
    } else {
        bterm::main()
    }
}
