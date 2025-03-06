use crate::core::{NoteSeries, median, usize_to_f64};

const PERFECT_THRESHOLD: f64 = 1.0;

const fn allow_perfection(grade: f64) -> f64 {
    (grade / PERFECT_THRESHOLD).min(1.)
}

fn distance_to_nearest_octave(x: f64) -> f64 {
    let lower = (x / 12.0).floor() * 12.0;
    let upper = (x / 12.0).ceil() * 12.0;
    (x - lower).abs().min((x - upper).abs())
}

/// Evaluates the grading of the note shift.
///
/// For fairness, compensation is made as such:
/// * whole octave shifts will get no penalty.
/// * whole perfect fifth and perfect forth shifts get a moderate compensation (~50%).
/// * other intervals are not compensated.
fn grade_key(d: f64) -> f64 {
    const FRAC_3_LOG6_3_4: f64 = 0.459_860_394_574_093_83;
    const FRAC_3_4_MUL_LN_6: f64 = 0.418_582_969_913_435_47;
    allow_perfection(match d {
        0.0..0.5 => ((-2.0 * d).mul_add(d, 0.5 * d) + 1.).min(1.),
        0.5..3.0 => FRAC_3_4_MUL_LN_6.mul_add(-d.ln(), FRAC_3_LOG6_3_4),
        3.0..4.0 => 0.0,
        4.0..6.0 => (-0.25f64).mul_add((std::f64::consts::PI * d).sin(), 0.25),
        _ => 0.0,
    })
}

fn get_intersection(target: &NoteSeries, input: &NoteSeries) -> (Vec<(f64, f64)>, usize) {
    let mut misses = 0usize;
    let mut intersection = Vec::new();
    for (t, i) in target.iter().zip(input) {
        match (t, i) {
            (Some(x), Some(y)) => {
                intersection.push((x.note_number, y.note_number));
            }
            (Some(_), None) => misses += 1,
            _ => {}
        }
    }
    (intersection, misses)
}

fn grade_coverage(misses: usize, len: usize) -> f64 {
    allow_perfection((1. - usize_to_f64(misses) / usize_to_f64(len)).cbrt())
}

fn grade_timing(time_shift_secs: f64) -> f64 {
    match time_shift_secs.abs() {
        0.0..0.05 => 1.,
        0.05..0.25 => (time_shift_secs.abs() - 0.05) / 0.2,
        _ => 0.0,
    }
}

#[derive(Debug)]
pub struct Accuracy {
    pub coverage: f64,
    pub timing: f64,
    pub pitch: f64,
    pub key: f64,
}

impl Accuracy {
    const PITCH_WEIGHT: f64 = 0.75;

    pub fn total_accuracy(&self) -> f64 {
        self.coverage
            * self.timing
            * self
                .pitch
                .mul_add(Self::PITCH_WEIGHT, self.key * (1. - Self::PITCH_WEIGHT))
    }
}

impl Accuracy {
    pub fn new(
        target: &NoteSeries,
        input: &NoteSeries,
        time_shift_secs: f64,
        note_shift: f64,
    ) -> Self {
        let (intersection, misses) = get_intersection(target, input);
        let mut individual_note_shifts = intersection
            .into_iter()
            .map(|(x, y)| grade_key((x - y).abs()))
            .collect::<Box<_>>();
        individual_note_shifts.sort_unstable_by(f64::total_cmp);
        Self {
            coverage: grade_coverage(misses, target.len()),
            timing: grade_timing(time_shift_secs),
            pitch: median(&individual_note_shifts),
            key: grade_key(distance_to_nearest_octave(note_shift)),
        }
    }
}
