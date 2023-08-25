use std::error::Error;

mod grid;

mod bterm;

mod world;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    bterm::main()
}
