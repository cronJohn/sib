use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn init() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_timer(fmt::time::ChronoLocal::new(
            "%Y-%m-%d %I:%M:%S%p".to_owned(),
        ))
        .with_target(false) // hide crate/module path
        .with_line_number(true)
        .with_file(true)
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT) // log span enter/exit
        .with_writer(std::io::stderr)
        .init();
}
