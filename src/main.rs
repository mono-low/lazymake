mod parser;
mod tui;
mod executor;
mod app;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let makefile_path = std::env::current_dir()?;
    
    let mut app = app::App::new(makefile_path)?;
    tui::run(&mut app).await?;
    
    Ok(())
}
