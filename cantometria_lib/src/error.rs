#[cfg(feature = "visualise")]
mod visualise {
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error(transparent)]
    pub enum PlotError {
        Drawing(
            #[from] plotters::prelude::DrawingAreaErrorKind<plotters_bitmap::BitMapBackendError>,
        ),
    }
}
#[cfg(feature = "visualise")]
pub use visualise::PlotError;

use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum RunError {
    OpenMidi(#[from] OpenMidiError),
    OpenWav(#[from] hound::Error),
    NewUnpaddedInputMelody(#[from] NewUnpaddedInputMelodyError),
    NewRawUnpaddedInputMelody(#[from] NewRawUnpaddedTargetMelodyError),
    #[error("input melody is empty")]
    InputMelodyEmpty,
    #[cfg(feature = "visualise")]
    #[error("plotting failed")]
    Plot(#[from] PlotError),
}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum OpenMidiError {
    FileOpen(#[from] std::io::Error),
    MidiFile(#[from] midi_file::Error),
}

#[derive(Error, Debug)]
pub enum NewRawUnpaddedTargetMelodyError {
    #[error("midi file tracks len not equal to one: {}", .0)]
    MidiFileTracksLenNotOne(u32),
    #[error("unknown track key signature")]
    UnknownKeySignature,
    #[error("unsupported time division type (SMPTE)")]
    UnsupportedDivisionType,
}

#[derive(Error, Debug)]
pub enum NewUnpaddedInputMelodyError {
    #[error("error reading sample from WAV file")]
    WavSampleRead(#[from] hound::Error),
}
