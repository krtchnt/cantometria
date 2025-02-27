use rustfft::FftPlanner;
use rustfft::num_complex::Complex;

use crate::core::{NoteSeries, usize_to_f64, usize_to_isize};

pub fn compute_time_shift(target: &NoteSeries, input: &NoteSeries) -> isize {
    let n = target.len();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    let ifft = planner.plan_fft_inverse(n);

    let target_values = target
        .iter()
        .map(|v| v.as_ref().map_or(0., |x| x.note_number))
        .collect::<Box<_>>();
    let input_values = input
        .iter()
        .map(|v| v.as_ref().map_or(0., |x| x.note_number))
        .collect::<Box<_>>();

    let mut target_fft = target_values
        .iter()
        .map(|&x| Complex::new(x, 0.))
        .collect::<Box<_>>();
    let mut input_fft = input_values
        .iter()
        .map(|&x| Complex::new(x, 0.))
        .collect::<Box<_>>();

    fft.process(&mut target_fft);
    fft.process(&mut input_fft);

    for i in 0..n {
        input_fft[i] = input_fft[i].conj() * target_fft[i];
    }

    ifft.process(&mut input_fft);

    let (max_shift, _) = input_fft
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.re.total_cmp(&b.re))
        .expect("input_fft must not be empty");

    if max_shift < n / 2 {
        usize_to_isize(max_shift)
    } else {
        usize_to_isize(max_shift) - usize_to_isize(n)
    }
}

pub fn apply_time_shift(target: &mut NoteSeries, shift: isize) {
    match shift.cmp(&0) {
        std::cmp::Ordering::Greater => target.rotate_left(shift.unsigned_abs()),
        std::cmp::Ordering::Less => target.rotate_right(shift.unsigned_abs()),
        std::cmp::Ordering::Equal => {}
    }
}

pub fn compute_note_shift(target: &NoteSeries, input: &NoteSeries) -> f64 {
    assert_eq!(
        target.len(),
        input.len(),
        "Signals must have the same length"
    );

    let mut sum_diff = 0.;
    let mut count = 0usize;

    for (t, i) in target.iter().zip(input.iter()) {
        if let (Some(t_val), Some(i_val)) = (t, i) {
            sum_diff += i_val.note_number - t_val.note_number;
            count += 1;
        }
    }

    assert!((count != 0), "No valid samples to compute amplitude shift");
    sum_diff / usize_to_f64(count)
}

pub fn apply_note_shift(target: &mut NoteSeries, shift: f64) {
    for sample in target {
        if let Some(note_number) = sample.as_mut() {
            note_number.note_number += shift;
        }
    }
}
