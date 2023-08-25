use std::error::Error;

mod grid;

mod bterm;
mod tui;

mod world;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tui::main()?;
    println!("hey");
    bterm::main()
}
