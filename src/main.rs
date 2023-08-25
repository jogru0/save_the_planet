use std::error::Error;

mod grid;

mod bterm;
mod tui;

mod world;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if false {
        tui::main()
    } else {
        bterm::main()
    }
}
