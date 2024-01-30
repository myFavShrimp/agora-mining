use plotters::prelude::*;
use time::macros::offset;
use crate::database::power_generation_and_consumption::PowerGenerationAndConsumption;

pub fn create_plot(data: Vec<PowerGenerationAndConsumption>, lowest_value: f64, highest_value: f64) -> String {
    let mut svg_path = String::new();
    let earliest_date = data.first().unwrap().date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64();
    let latest_date = data.last().unwrap().date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64();

    let grid_emission_factor = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.grid_emission_factor.unwrap()));
    let biomass = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.biomass.unwrap()));
    let hard_coal = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.hard_coal.unwrap()));
    let hydro = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.hydro.unwrap()));
    let lignite = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.lignite.unwrap()));
    let natural_gas = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.natural_gas.unwrap()));
    let nuclear = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.nuclear.unwrap()));
    let other = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.other.unwrap()));
    let pumped_storage_generation = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.pumped_storage_generation.unwrap()));
    let solar = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.solar.unwrap()));
    let total_conventional_power_plant = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.total_conventional_power_plant.unwrap()));
    let total_electricity_demand = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.total_electricity_demand.unwrap()));
    let total_grid_emissions = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.total_grid_emissions.unwrap()));
    let wind_offshore = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.wind_offshore.unwrap()));
    let wind_onshore = data.iter().map(|data| (data.date_id.assume_offset(offset!(UTC)).unix_timestamp().as_f64(), data.wind_onshore.unwrap()));

    {
        let root = SVGBackend::with_string(&mut svg_path, (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .margin(5)
            .x_label_area_size(25)
            .y_label_area_size(50)
            .build_cartesian_2d(earliest_date..latest_date,
                                (lowest_value..highest_value),
            ).unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                grid_emission_factor,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                biomass,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                hard_coal,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                hydro,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                lignite,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                natural_gas,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                nuclear,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                other,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                pumped_storage_generation,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                solar,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                total_conventional_power_plant,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                total_electricity_demand,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                total_grid_emissions,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                wind_offshore,
                &RED,
            )).unwrap();

        chart
            .draw_series(LineSeries::new(
                wind_onshore,
                &RED,
            )).unwrap();


        root.present().unwrap();
    };

    svg_path
}