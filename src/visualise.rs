use std::path::Path;

use ordered_float::FloatCore;

use crate::{
    core::{NoteSeries, NoteTimeSeries, Time, usize_to_f64},
    error::PlotError,
};
pub fn plot<P: AsRef<Path>>(
    target_series: &NoteSeries,
    input_time_series: &NoteTimeSeries,
    file: P,
) -> Result<(), PlotError> {
    use plotters::{
        chart::ChartBuilder,
        prelude::{BitMapBackend, Circle, IntoDrawingArea},
        style::{BLACK, BLUE, Color, RED, WHITE},
    };

    const CIRCLE_SIZE: f64 = 2.;

    let dt = input_time_series.interval();
    let target_data = target_series
        .iter()
        .enumerate()
        .filter_map(|(i, ts)| ts.as_ref().map(|ns| (dt * usize_to_f64(i), ns.note_number)))
        .collect::<Box<_>>();

    let input_data = input_time_series
        .iter()
        .enumerate()
        .filter_map(|(i, ts)| ts.as_ref().map(|ns| (dt * usize_to_f64(i), ns.note_number)))
        .collect::<Box<_>>();

    let all_x = target_data
        .iter()
        .map(|(x, _)| *x)
        .chain(input_data.iter().map(|(x, _)| *x))
        .collect::<Box<_>>();
    let x_min = all_x.iter().copied().fold(Time::infinity(), Ord::min);
    let x_max = all_x.iter().copied().fold(Time::neg_infinity(), Ord::max);

    let all_y: Vec<f64> = target_data
        .iter()
        .map(|(_, y)| *y)
        .chain(input_data.iter().map(|(_, y)| *y))
        .collect();
    let y_min = all_y.iter().copied().fold(f64::INFINITY, f64::min);
    let y_max = all_y.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    let target_fp = Path::new("target").join(file);
    let root_area = BitMapBackend::new(&target_fp, (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Target vs Input Melodies", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(*x_min..*x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(
            target_data
                .iter()
                .map(|(x, y)| Circle::new((**x, *y), CIRCLE_SIZE, RED.filled())),
        )?
        .label("Target")
        .legend(|(x, y)| Circle::new((x + 10, y), CIRCLE_SIZE, RED.filled()));

    chart
        .draw_series(
            input_data
                .iter()
                .map(|(x, y)| Circle::new((**x, *y), CIRCLE_SIZE, BLUE.filled())),
        )?
        .label("Input")
        .legend(|(x, y)| Circle::new((x + 10, y), CIRCLE_SIZE, BLUE.filled()));

    chart.configure_series_labels().border_style(BLACK).draw()?;

    Ok(())
}
