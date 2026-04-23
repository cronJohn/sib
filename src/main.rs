use color_eyre::Result;
use sib::{app::App, config::load_config, context::Context, logging};
use tracing::{error, info};

fn main() -> Result<()> {
    color_eyre::install()?;
    logging::init();
    let config = load_config()?;

    info!("Starting TUI...");

    let context = Context::new(&config);
    let notes = context.parser.collect_notes();
    let mut app = App::new(notes, config);

    if let Err(e) = app.run(context) {
        error!("TUI encountered an error: {:?}", e);
    }

    info!("Exiting TUI...");
    Ok(())
}
