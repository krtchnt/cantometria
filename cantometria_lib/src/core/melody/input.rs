use hound::WavReader;
use pitch_detection::detector::PitchDetector;
use pitch_detection::detector::yin::YINDetector;
use std::{fs::File, io::BufReader, path::Path};

use crate::{
    core::{DynNoteTimeSeries, model::Note, usize_to_f64},
    error::NewUnpaddedInputMelodyError,
};

fn frequency_to_note_number(frequency: f64) -> Option<f64> {
    const A4_FREQUENCY: f64 = 440.0;
    (frequency > 0.0).then(|| {
        let semitones_from_a4 = 12.0 * (frequency / A4_FREQUENCY).log2();
        69.0 + semitones_from_a4
    })
}

type WavFile = WavReader<BufReader<File>>;

pub fn open_wav<P: AsRef<Path>>(path: P) -> Result<WavFile, hound::Error> {
    WavReader::open(path)
}

pub struct UnpaddedInputMelody {
    pub notes: DynNoteTimeSeries,
}

impl UnpaddedInputMelody {
    pub fn new(mut wav: WavFile) -> Result<Self, NewUnpaddedInputMelodyError> {
        const SIZE: usize = 1024;
        const PADDING: usize = SIZE / 2;
        const POWER_THRESHOLD: f64 = 1.0;
        const CLARITY_THRESHOLD: f64 = 0.9;

        let spec = wav.spec();
        let sample_rate = spec.sample_rate as usize;
        let mut detector = YINDetector::new(SIZE, PADDING);
        let mut samples_f64: Vec<f64> = Vec::with_capacity(SIZE);
        let mut notes = Vec::new();

        let num_channels = f64::from(spec.channels);
        let chunk_duration_seconds =
            (usize_to_f64(SIZE) / num_channels) / usize_to_f64(sample_rate);

        wav.samples::<i16>().try_for_each(|s| {
            let sample = f64::from(s?) / f64::from(i16::MAX);
            samples_f64.push(sample);

            if samples_f64.len() >= SIZE {
                let pitch_result = detector.get_pitch(
                    &samples_f64,
                    sample_rate,
                    POWER_THRESHOLD,
                    CLARITY_THRESHOLD,
                );

                notes.push(
                    pitch_result
                        .and_then(|p| Some(Note::new(frequency_to_note_number(p.frequency)?))),
                );
                samples_f64.clear();
            }
            Ok::<_, NewUnpaddedInputMelodyError>(())
        })?;

        Ok(Self {
            notes: DynNoteTimeSeries::new(notes, chunk_duration_seconds.into()),
        })
    }
}
