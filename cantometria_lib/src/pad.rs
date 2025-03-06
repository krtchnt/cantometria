use crate::{
    core::{NoteTimeSeries, UnpaddedInputMelody},
    interpolate::UnpaddedTargetMelody,
};

pub struct TargetMelody {
    notes: NoteTimeSeries,
}

impl TargetMelody {
    pub const fn notes(&self) -> &NoteTimeSeries {
        &self.notes
    }
}
pub struct InputMelody {
    notes: NoteTimeSeries,
}

impl InputMelody {
    pub const fn notes(&self) -> &NoteTimeSeries {
        &self.notes
    }
}

pub fn zero_pad_shorter_series(
    mut target: UnpaddedTargetMelody,
    mut input: UnpaddedInputMelody,
) -> (TargetMelody, InputMelody) {
    let dt = target.notes.interval();
    let (shorter, longer) = if target.notes.len() < input.notes.len() {
        (&mut target.notes, &input.notes)
    } else {
        (&mut input.notes, &target.notes)
    };

    let mut t = dt * crate::core::usize_to_f64(shorter.len());
    while shorter.len() < longer.len() {
        shorter.push(None);
        t += dt;
    }
    debug_assert_eq!(shorter.len(), longer.len());

    let target_melody = TargetMelody {
        notes: target.notes.into(),
    };
    let input_melody = InputMelody {
        notes: input.notes.into(),
    };
    (target_melody, input_melody)
}
