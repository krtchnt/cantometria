use std::ops::Deref;

use midi_file::file::MicrosecondsPerQuarter;
use ordered_float::OrderedFloat;

type NoteSample = Option<Note>;
type TimedNote = Timed<NoteSample>;
type NonUniformNoteTimeSeriesInner = [TimedNote];

#[derive(Clone)]
pub struct UniformNoteTimeSeries<A>
where
    A: IntoIterator<Item = NoteSample> + Deref<Target = [NoteSample]>,
{
    samples: A,
    interval: Time,
}

impl<A> UniformNoteTimeSeries<A>
where
    A: IntoIterator<Item = NoteSample> + Deref<Target = [NoteSample]>,
{
    pub const fn new(samples: A, interval: Time) -> Self {
        Self { samples, interval }
    }

    pub const fn samples(&self) -> &A {
        &self.samples
    }

    pub const fn interval(&self) -> OrderedFloat<f64> {
        self.interval
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    #[cfg(feature = "visualise")]
    pub fn iter(&self) -> std::slice::Iter<'_, NoteSample> {
        self.samples.iter()
    }
}

impl UniformNoteTimeSeries<Vec<NoteSample>> {
    pub fn push(&mut self, value: NoteSample) {
        self.samples.push(value);
    }
}

pub type DynNoteTimeSeries = UniformNoteTimeSeries<Vec<NoteSample>>;
pub type DynNonUniformNoteTimeSeries = Vec<TimedNote>;
pub type NoteSeries = [NoteSample];
pub type NoteTimeSeries = UniformNoteTimeSeries<Box<NoteSeries>>;

impl From<DynNoteTimeSeries> for NoteTimeSeries {
    fn from(value: DynNoteTimeSeries) -> Self {
        Self {
            samples: value.samples.into(),
            interval: value.interval,
        }
    }
}

pub type NonUniformNoteTimeSeriesRef<'a> = &'a NonUniformNoteTimeSeriesInner;
pub type Time = OrderedFloat<f64>;

#[derive(Debug, Clone)]
pub struct Timed<T: Clone> {
    pub time: Time,
    pub value: T,
}

impl<T: Clone> From<(Time, T)> for Timed<T> {
    fn from(value: (Time, T)) -> Self {
        Self {
            time: value.0,
            value: value.1,
        }
    }
}

impl<T: Clone> Timed<T> {
    pub const fn new(time: f64, value: T) -> Self {
        Self {
            time: OrderedFloat(time),
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tempo(pub MicrosecondsPerQuarter);

impl Tempo {
    pub const DEFAULT: Self = Self(MicrosecondsPerQuarter::new(500_000));
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Note {
    pub note_number: f64,
}

impl Note {
    pub const VELOCITY_THRESHOLD: u8 = 0;
    pub const fn new(note_number: f64) -> Self {
        Self { note_number }
    }
}
