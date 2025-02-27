mod melody;
mod model;

pub use melody::{RawUnpaddedTargetMelody, UnpaddedInputMelody, open_midi, open_wav};
#[cfg(feature = "visualise")]
pub use model::Time;
pub use model::{
    DynNonUniformNoteTimeSeries, DynNoteTimeSeries, Note, NoteSeries, NoteTimeSeries, Tempo, Timed,
};

#[allow(clippy::cast_precision_loss)]
pub const fn usize_to_f64(value: usize) -> f64 {
    value as f64
}

#[allow(clippy::cast_possible_wrap)]
pub const fn usize_to_isize(value: usize) -> isize {
    value as isize
}
