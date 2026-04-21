use crate::domain::note::Note;

/// Side-effect requests emitted during `update` and executed by the runtime.
///
/// Used for operations outside pure state updates (e.g. IO, subprocesses).
#[derive(Debug)]
pub enum Effect {
    OpenEditor(Note),
}
