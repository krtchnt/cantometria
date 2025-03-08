use std::{io::BufReader, path::Path};

use midi_file::{
    MidiFile,
    core::Message,
    file::{Division, Event, MetaEvent, MicrosecondsPerQuarter},
};

use crate::{
    core::{DynNonUniformNoteTimeSeries, Note, Timed, model::NonUniformNoteTimeSeriesRef},
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

        let mut last_tempo: Option<MicrosecondsPerQuarter> = None;
        let mut note_events = Vec::new();
        let mut time = 0.;

        for event in track.events() {
            time += f64::from(event.delta_time() * last_tempo.unwrap_or_default().get())
                / (f64::from(tpqn.get()) * 1e6);
            match event.event() {
                Event::Midi(midi_event) => {
                    match midi_event {
                        Message::NoteOn(note_on)
                            if note_on.velocity().get() > Note::VELOCITY_THRESHOLD =>
                        {
                            note_events.push(Timed::new(
                                time,
                                Some(crate::core::Note {
                                    note_number: note_on.note_number().get().into(),
                                }),
                            ));
                        }
                        Message::NoteOn(_) | Message::NoteOff(_) => {
                            note_events.push(Timed::new(time, None));
                        }
                        _ => {}
                    };
                }
                Event::Meta(MetaEvent::SetTempo(tempo)) => last_tempo = Some(*tempo),
                _ => {}
            }
        }

        Ok(Self { note_events })
    }

    pub fn note_events(&self) -> NonUniformNoteTimeSeriesRef {
        &self.note_events
    }
}
