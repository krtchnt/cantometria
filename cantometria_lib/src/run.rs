use std::path::Path;

use crate::align;
use crate::core;
use crate::error::RunError;
use crate::grade::Accuracy;

/// # Errors
/// - opening the midi file failed
/// - opening the wav file failed
/// - creating a new raw unpadded target melody failed
/// - creating a new unpadded target melody failed
/// - input melody is empty
/// - (visualise) plotting failed
pub fn run<P: AsRef<Path>>(midi_file: P, wav_file: P) -> Result<Accuracy, RunError> {
    #[cfg(feature = "visualise")]
    let fp = plot_target_file(&midi_file, &wav_file);
    let midi = core::open_midi(midi_file)?;
    let target_unpadded_raw = core::RawUnpaddedTargetMelody::new(&midi)?;
    let wav = crate::core::open_wav(wav_file)?;
    let input_unpadded = core::UnpaddedInputMelody::new(wav)?;

    let target_unpadded = target_unpadded_raw.zero_order_hold(&input_unpadded);
    let (target_unaligned, input_series) =
        crate::pad::zero_pad_shorter_series(target_unpadded, input_unpadded);

    let mut target = target_unaligned.notes().samples().clone();
    let input = input_series.notes().samples();
    let time_shift = align::compute_time_shift(&target, input).ok_or(RunError::InputMelodyEmpty)?;
    align::apply_time_shift(&mut target, time_shift);
    let note_shift = align::compute_note_shift(&target, input);
    align::apply_note_shift(&mut target, note_shift);
    let time_shift_secs = *input_series.notes().interval() * core::isize_to_f64(time_shift);
    #[cfg(feature = "visualise")]
    crate::visualise::plot(&target, input_series.notes(), fp)?;
    Ok(Accuracy::new(&target, input, time_shift_secs, note_shift))
}

#[cfg(feature = "visualise")]
fn plot_target_file<P: AsRef<Path>>(midi_file: &P, wav_file: &P) -> String {
    let midi_file_stem = midi_file.as_ref().file_stem().expect("no file stem");
    let wav_file_stem = wav_file.as_ref().file_stem().expect("no file stem");
    format!(
        "{}+{}.png",
        midi_file_stem.to_str().expect("utf-8 file stem"),
        wav_file_stem.to_str().expect("utf-8 file stem")
    )
}

#[cfg(test)]
mod test {
    use std::{f64, ops::RangeBounds, path::Path};

    use rstest::rstest;

    use super::run;

    #[rstest]
    #[case("test.mid", "test.wav", 0.8..1.0)]
    #[case("test.mid", "100hz-4s.wav", 0.0..0.2)]
    #[case("bite.mid", "bite-cn.wav", 0.6..0.8)]
    #[case("bite.mid", "bite-cn-delayed-100ms.wav", 0.0..0.2)]
    #[case("bite.mid", "bite-cn-delayed-6.1s.wav", 0.0..0.2)]
    #[case("bite-2.mid", "bite-kr.wav", 0.6..0.8)]
    #[case("bite-2.mid", "bite-jp.wav", 0.0..0.2)]
    #[case("tetris.mid", "tetris.wav", 0.8..1.0)]
    #[case("tetris.mid", "tetris-2.wav", 0.8..1.0)]
    fn test<P: AsRef<Path>, R: RangeBounds<f64>>(
        #[case] midi_file: P,
        #[case] wav_file: P,
        #[case] expected_accuracy: R,
    ) {
        let accuracy = run(midi_file, wav_file).expect("running failed");
        assert!(expected_accuracy.contains(&accuracy.total_accuracy()));
    }
}
