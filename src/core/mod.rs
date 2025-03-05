mod melody;
mod model;

pub use melody::{RawUnpaddedTargetMelody, UnpaddedInputMelody, open_midi, open_wav};
#[cfg(feature = "visualise")]
pub use model::Time;
pub use model::{
    DynNonUniformNoteTimeSeries, DynNoteTimeSeries, Note, NoteSeries, NoteTimeSeries, Timed,
};

#[allow(clippy::cast_precision_loss)]
pub const fn usize_to_f64(value: usize) -> f64 {
    value as f64
}

#[allow(clippy::cast_precision_loss)]
pub const fn isize_to_f64(value: isize) -> f64 {
    value as f64
}

#[allow(clippy::cast_possible_wrap)]
pub const fn usize_to_isize(value: usize) -> isize {
    value as isize
}

pub const fn median(arr: &[f64]) -> f64 {
    if arr.len() % 2 == 1 {
        return arr[arr.len() / 2];
    }
    (arr[arr.len() / 2 - 1] + arr[arr.len() / 2]) / 2.
}
