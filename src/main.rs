use color_eyre::Result;
use sib::{
    app::{run::run_tui, App},
    config::load_config,
    context::Context,
    logging,
};
use tracing::{error, info};

fn main() -> Result<()> {
    color_eyre::install()?;
    logging::init();
    let config = load_config()?;

    info!("Starting TUI...");

    let context = Context::new(&config);
    let notes = context.parser.collect_notes();
    let app = App::new(notes);

    if let Err(e) = run_tui(app, context) {
        error!("TUI encountered an error: {:?}", e);
    }

    info!("Exiting TUI...");
    Ok(())
}
