#[macro_use]
mod macros;
mod config;
mod html;
mod json;
mod tui;

use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cmd!(utf - 8);
    cmd!(line);
    html::sub();
    json::sub();
    tui::sub();

    config::write();
    config::read();
    Ok(())
}
