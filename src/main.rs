use color_eyre::Result;
use sib::{app::App, config::load_config, context::Context, logging, tui::run_tui};
use tracing::{error, info};

fn main() -> Result<()> {
    color_eyre::install()?;
    logging::init();
    let config = load_config()?;

    info!("Starting TUI...");

    let mut app = App::default();
    let context = Context::new(&config);

    if let Err(e) = run_tui(&mut app, &context) {
        error!("TUI encountered an error: {:?}", e);
    }

    info!("Exiting TUI...");
    Ok(())
}
