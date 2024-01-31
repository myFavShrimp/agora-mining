use std::collections::HashMap;

use serde::Serialize;
use time::PrimitiveDateTime;

use crate::{
    agora::GenerationKind,
    database::power_generation_and_consumption::PowerGenerationAndConsumption,
};

#[derive(Serialize)]
pub struct PlottingTemplateDataSet {
    pub label: &'static str,
    pub data: Vec<PlottingTemplateDataSetData>,
}

#[derive(Serialize)]
pub struct PlottingTemplateDataSetData {
    pub x: PrimitiveDateTime,
    pub y: Option<f64>,
}

impl GenerationKind {
    fn chart_display_name(&self) -> &'static str {
        match self {
            GenerationKind::Biomass => "Biomass",
            GenerationKind::GridEmissionFactor => "Grid Emission Factor",
            GenerationKind::HardCoal => "Hard Coal",
            GenerationKind::Hydro => "Hydro",
            GenerationKind::Lignite => "Lignite",
            GenerationKind::NaturalGas => "Natural Gas",
            GenerationKind::Nuclear => "Nuclear",
            GenerationKind::Other => "Other",
            GenerationKind::PumpedStorageGeneration => "Pumped Storage Generation",
            GenerationKind::Solar => "Solar",
            GenerationKind::TotalConventionalPowerPlant => "Total Conventional Power Plant",
            GenerationKind::TotalElectricityDemand => "Total Electricity Demand",
            GenerationKind::TotalGridEmissions => "Total Grid Emissions",
            GenerationKind::WindOffshore => "Wind Offshore",
            GenerationKind::WindOnshore => "Wind Onshore",
        }
    }
}

impl PowerGenerationAndConsumption {
    fn get_by_kind(&self, kind: &GenerationKind) -> Option<f64> {
        match kind {
            GenerationKind::Biomass => self.biomass,
            GenerationKind::GridEmissionFactor => self.grid_emission_factor,
            GenerationKind::HardCoal => self.hard_coal,
            GenerationKind::Hydro => self.hydro,
            GenerationKind::Lignite => self.lignite,
            GenerationKind::NaturalGas => self.natural_gas,
            GenerationKind::Nuclear => self.nuclear,
            GenerationKind::Other => self.other,
            GenerationKind::PumpedStorageGeneration => self.pumped_storage_generation,
            GenerationKind::Solar => self.solar,
            GenerationKind::TotalConventionalPowerPlant => self.total_conventional_power_plant,
            GenerationKind::TotalElectricityDemand => self.total_electricity_demand,
            GenerationKind::TotalGridEmissions => self.total_grid_emissions,
            GenerationKind::WindOffshore => self.wind_offshore,
            GenerationKind::WindOnshore => self.wind_onshore,
        }
    }
}

pub fn to_data_sets(data: Vec<PowerGenerationAndConsumption>) -> Vec<PlottingTemplateDataSet> {
    let mut result_map: HashMap<GenerationKind, PlottingTemplateDataSet> = HashMap::new();

    for item in data {
        for kind in GenerationKind::all() {
            let new_data = PlottingTemplateDataSetData {
                x: item.date_id,
                y: item.get_by_kind(&kind),
            };

            if let Some(existing_data) = result_map.get_mut(&kind) {
                existing_data.data.push(new_data)
            } else {
                let new_data_set = PlottingTemplateDataSet {
                    label: kind.chart_display_name(),
                    data: vec![new_data],
                };
                result_map.insert(kind, new_data_set);
            }
        }
    }

    result_map.into_values().collect()
}
