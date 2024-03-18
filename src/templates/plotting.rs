use std::collections::HashMap;

use serde::Serialize;
use time::macros::format_description;

use crate::database::{
    power_generation::{Fields, PowerGeneration},
    Entity,
};

#[derive(Serialize)]
pub struct PlottingTemplateDataSet {
    pub label: &'static str,
    pub data: Vec<PlottingTemplateDataSetData>,
    pub unit: String,
}

#[derive(Serialize)]
pub struct PlottingTemplateDataSetData {
    pub x: String,
    pub y: Option<f64>,
}

impl Fields {
    fn chart_display_name(&self) -> &'static str {
        match self {
            Fields::Biomass => "Biomass",
            Fields::HardCoal => "Hard Coal",
            Fields::Hydro => "Hydro",
            Fields::Lignite => "Lignite",
            Fields::NaturalGas => "Natural Gas",
            Fields::Nuclear => "Nuclear",
            Fields::Other => "Other",
            Fields::PumpedStorageGeneration => "Pumped Storage Generation",
            Fields::Solar => "Solar",
            Fields::TotalConventionalPowerPlant => "Total Conventional Power Plant",
            Fields::WindOffshore => "Wind Offshore",
            Fields::WindOnshore => "Wind Onshore",
        }
    }
}

impl PowerGeneration {
    fn get_by_kind(&self, kind: &Fields) -> Option<f64> {
        match kind {
            Fields::Biomass => self.biomass,
            Fields::HardCoal => self.hard_coal,
            Fields::Hydro => self.hydro,
            Fields::Lignite => self.lignite,
            Fields::NaturalGas => self.natural_gas,
            Fields::Nuclear => self.nuclear,
            Fields::Other => self.other,
            Fields::PumpedStorageGeneration => self.pumped_storage_generation,
            Fields::Solar => self.solar,
            Fields::TotalConventionalPowerPlant => self.total_conventional_power_plant,
            Fields::WindOffshore => self.wind_offshore,
            Fields::WindOnshore => self.wind_onshore,
        }
    }
}

pub fn to_data_sets(data: Vec<PowerGeneration>) -> Vec<PlottingTemplateDataSet> {
    let mut result_map: HashMap<Fields, PlottingTemplateDataSet> = HashMap::new();

    for item in data {
        for kind in PowerGeneration::all_fields() {
            let new_data = PlottingTemplateDataSetData {
                x: item
                    .date_id
                    .format(format_description!("[day].[month].[year] [hour]:[minute]"))
                    .unwrap(),
                y: item.get_by_kind(&kind),
            };

            if let Some(existing_data) = result_map.get_mut(&kind) {
                existing_data.data.push(new_data)
            } else {
                let new_data_set = PlottingTemplateDataSet {
                    label: kind.chart_display_name(),
                    data: vec![new_data],
                    unit: PowerGeneration::unit(&kind),
                };
                result_map.insert(kind, new_data_set);
            }
        }
    }

    result_map.into_values().collect()
}
