use std::path::PathBuf;

use clap::Parser;
use termtab::app::App;

#[derive(Parser)]
pub struct Cli {
    path: PathBuf,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let mut terminal = ratatui::init();
    let mut app = App::new(cli.path);
    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}
