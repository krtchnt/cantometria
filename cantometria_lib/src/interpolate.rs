use ordered_float::OrderedFloat;

use crate::core::{DynNoteTimeSeries, Timed, UnpaddedInputMelody};

pub struct UnpaddedTargetMelody {
    pub notes: DynNoteTimeSeries,
}

impl crate::core::RawUnpaddedTargetMelody {
    pub fn zero_order_hold(self, input: &UnpaddedInputMelody) -> UnpaddedTargetMelody {
        let mut target = Vec::new();
        let mut last_event = None;
        let mut t = OrderedFloat(0.);
        let dt = input.notes.interval();
        for event in self.note_events() {
            while t < event.time {
                target.push(last_event.clone().and_then(|o: Timed<_>| o.value));
                t += dt;
            }
            last_event = Some(event.clone());
        }

        UnpaddedTargetMelody {
            notes: DynNoteTimeSeries::new(target, dt),
        }
    }
}
