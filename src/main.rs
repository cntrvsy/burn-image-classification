// region:    --- Modules

mod error;
mod model;

pub use error::{Error, Result};

// endregion: --- Modules

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}
