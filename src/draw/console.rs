use super::pixels::{PixelState, TextDrawingBackend};


use plotters::prelude::*;
use plotters::style::{CYAN};
use std::error::Error;

// Modified version of:
// https://github.com/plotters-rs/plotters/blob/master/plotters/examples/console.rs

pub fn chart(data: Vec<(f64, f64)>, steps: usize) -> Result<(), Box<dyn Error>> {

    // get last period, or all data if period is not full
    let last_period: Vec<(f64, f64)> = if data.len() > steps {
        data[data.len() - steps..].to_vec()
    } else {
        data.clone()
    };

    let (tmin, _) = last_period.first().unwrap();
    let (tmax, _) = last_period.last().unwrap();
    let (_, pmax) = last_period.iter().cloned().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();

    // set xmax to period length or highest time
    let xmax = if *tmax > steps as f64 {
        *tmax
    } else {
        steps as f64
    };

    // set ymax to base or highest price
    let ymax = if pmax > 250.0 {
        pmax
    } else {
        250.0
    };

    let xrange = *tmin..xmax;
    let yrange = 0.0..ymax;

    let buffer = vec![PixelState::Empty; 5000];
    let b = TextDrawingBackend(buffer).into_drawing_area();

    let caption = String::from("Market Price");

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
        last_period.into_iter(),
        &CYAN,
    ))?;

    b.present()?;

    Ok(())
}