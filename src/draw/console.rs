use super::pixels::{PixelState, TextDrawingBackend};


use plotters::prelude::*;
use plotters::style::{CYAN};
use plotters_backend::{DrawingBackend};
use std::ops::Range;
use std::error::Error;

// Modified version of:
// https://github.com/plotters-rs/plotters/blob/master/plotters/examples/console.rs

pub fn chart(data: Vec<(f64, f64)>, xmax: f64, ymax: f64) -> Result<(), Box<dyn Error>> {

    let caption = String::from("Market Price");
    let xrange = 0.0..xmax;
    let yrange = 0.0..ymax;

    let buffer = vec![PixelState::Empty; 5000];
    let b = TextDrawingBackend(buffer).into_drawing_area();

    let mut chart = ChartBuilder::on(&b)
        .margin(1)
        .caption(caption, ("sans-serif", (10).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (10i32).percent_width())
        .set_label_area_size(LabelAreaPosition::Bottom, (10i32).percent_height())
        .build_cartesian_2d(xrange, yrange)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(LineSeries::new(
        data.into_iter(),
        &CYAN,
    ))?;

    b.present()?;

    Ok(())
}