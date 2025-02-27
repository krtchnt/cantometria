use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum OpenMidiError {
    FileOpen(#[from] std::io::Error),
    MidiFile(#[from] midi_file::Error),
}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum NewRawUnpaddedTargetMelodyError {
    #[error("midi file tracks len not equal to one: {}", .0)]
    MidiFileTracksLenNotOne(u32),
    #[error("unknown track key signature")]
    UnknownKeySignature,
    #[error("unsupported time division type (SMPTE)")]
    UnsupportedDivisionType,
}

#[derive(Error, Debug)]
pub enum NewInputMelodyError {
    #[error("error reading sample from WAV file")]
    WavSampleRead(#[from] hound::Error),
}
