use core::usize_to_f64;

mod align;
mod core;
mod error;
mod grade;
mod interpolate;
mod pad;
#[cfg(feature = "visualise")]
mod visualise;

fn main() {
    let midi = core::open_midi("midi/test.mid").expect("opening midi file failed");
    let target_unpadded_raw =
        core::RawUnpaddedTargetMelody::new(&midi).expect("creating target melody failed");
    let wav = core::open_wav("test/test.wav").expect("opening wav file failed");
    let input_unpadded = core::UnpaddedInputMelody::new(wav).expect("creating input melody failed");

    let target_unpadded = target_unpadded_raw.zero_order_hold(&input_unpadded);
    let (target_unaligned, input_series) =
        pad::zero_pad_shorter_series(target_unpadded, input_unpadded);

    let mut target = target_unaligned.notes().samples().clone();
    let input = input_series.notes().samples();
    let time_shift = align::compute_time_shift(&target, input);
    align::apply_time_shift(&mut target, time_shift);
    let note_shift = align::compute_note_shift(&target, input);
    align::apply_note_shift(&mut target, note_shift);

    let mut misses = 0usize;
    let mut intersection = Vec::new();
    for (t, i) in target.iter().zip(input) {
        match (t, i) {
            (Some(x), Some(y)) => intersection.push((x.note_number - y.note_number).abs()),
            (Some(_), None) => misses += 1,
            _ => {}
        }
    }
    dbg!(
        intersection,
        time_shift,
        note_shift,
        usize_to_f64(misses) / usize_to_f64(target.len())
    );
    #[cfg(feature = "visualise")]
    visualise::plot(&target, input_series.notes()).expect("plotting time series failed");
}
