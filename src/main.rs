use color_eyre::Result;
use sib::{app::App, logging, tui::run_tui};
use tracing::{error, info};

fn main() -> Result<()> {
    color_eyre::install()?;
    logging::init();

    info!("Starting TUI...");

    let mut app = App::new();

    if let Err(e) = run_tui(&mut app) {
        error!("TUI encountered an error: {:?}", e);
    }

    info!("Exiting TUI...");
    Ok(())
}
