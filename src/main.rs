#[macro_use]
mod macros;

use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cmd!(utf - 8);
    cmd!(line);

    Ok(())
}
