use std::{io::BufReader, path::Path};

use midi_file::{
    MidiFile,
    core::Message,
    file::{Division, Event, MetaEvent},
};

use crate::{
    core::{
        DynNonUniformNoteTimeSeries, Note, Timed,
        model::{NonUniformNoteTimeSeriesRef, Tempo},
    },
    error::{NewRawUnpaddedTargetMelodyError, OpenMidiError},
};

pub fn open_midi<P: AsRef<Path>>(path: P) -> Result<MidiFile, OpenMidiError> {
    Ok(MidiFile::read(BufReader::new(std::fs::File::open(path)?))?)
}

pub struct RawUnpaddedTargetMelody {
    note_events: DynNonUniformNoteTimeSeries,
}

impl RawUnpaddedTargetMelody {
    pub fn new(midi: &MidiFile) -> Result<Self, NewRawUnpaddedTargetMelodyError> {
        let tracks_len = midi.tracks_len();
        let Division::QuarterNote(tpqn) = midi.header().division() else {
            return Err(NewRawUnpaddedTargetMelodyError::UnsupportedDivisionType);
        };
        if tracks_len != 1 {
            return Err(NewRawUnpaddedTargetMelodyError::MidiFileTracksLenNotOne(
                tracks_len,
            ));
        }
        // SAFETY: at this point, midi.tracks_len() == 1, so this is safe
        let track = unsafe { midi.tracks().next().unwrap_unchecked() };

        let mut key_signatures = Vec::new();
        let mut tempos: Vec<Timed<Tempo>> = Vec::new();
        let mut note_events = Vec::new();
        let mut time = 0.;

        for event in track.events() {
            let last_tempo = tempos
                .last()
                .map_or_else(|| &Tempo::DEFAULT, |sample| &sample.value);
            time +=
                f64::from(event.delta_time() * last_tempo.0.get()) / (f64::from(tpqn.get()) * 1e6);
            #[allow(clippy::match_wildcard_for_single_variants)]
            match event.event() /* TODO: make this #[non_exhaustive] to remove the above lint */ {
                Event::Midi(midi_event) => {
                    match midi_event {
                        Message::NoteOn(note_on)
                            if note_on.velocity().get() > Note::VELOCITY_THRESHOLD =>
                        {
                            note_events.push(Timed::new(
                                time,
                                Some(crate::core::Note {
                                    note_number: note_on.note_number().get().into()
                                }),
                            ));
                        }
                        Message::NoteOn(_) | Message::NoteOff(_) => {
                            note_events.push(Timed::new(time, None));
                        }
                        _ => {}
                    };
                }
                Event::Meta(meta_event) => match meta_event {
                    MetaEvent::KeySignature(signature) => {
                        key_signatures.push(Timed::new(time, *signature));
                    }
                    MetaEvent::SetTempo(tempo) => tempos.push(Timed::new(time, Tempo(*tempo))),
                    _ => {}
                },
                _ => {}
            }
        }

        if key_signatures.is_empty() {
            return Err(NewRawUnpaddedTargetMelodyError::UnknownKeySignature);
        };

        //dbg!(key_signatures);

        Ok(Self { note_events })
    }

    pub fn note_events(&self) -> NonUniformNoteTimeSeriesRef {
        &self.note_events
    }
}
