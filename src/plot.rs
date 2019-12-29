use plotters::coord::Shift;
use plotters::prelude::*;

fn draw_chart<B: DrawingBackend>(
    root: &DrawingArea<B, Shift>,
    arr: Vec<(f64, f64)>,
) -> DrawResult<(), B> {
    let size = arr.len();

    let (x, y): (Vec<_>, Vec<_>) = arr.iter().cloned().unzip();

    let x_offset = (x[size - 1] - x[0]) * 0.05;

    let x_max = x[size - 1] + x_offset;
    let x_min = x[0] - x_offset;

    let y_max = y
        .iter()
        .fold(std::f64::NEG_INFINITY, |acc, x| f64::max(acc, *x));
    let y_min = y
        .iter()
        .fold(std::f64::INFINITY, |acc, x| f64::min(acc, *x));

    let y_offset = (y_max - y_min) * 0.05;
    let y_max = y_max + y_offset;
    let y_min = y_min - y_offset;

    let mut chart = ChartBuilder::on(root)
        .caption("FFTExample", ("sans-serif", (5).percent_height()))
        .x_label_area_size((10).percent_height())
        .y_label_area_size((10).percent_width())
        .margin(15)
        .build_ranged(x_min..x_max, y_min..y_max)?;

    chart
        .configure_mesh()
        .label_style(("sans-serif", (3).percent_height()))
        .draw()?;

    chart.draw_series(LineSeries::new(arr, &RED))?;
    Ok(())
}
