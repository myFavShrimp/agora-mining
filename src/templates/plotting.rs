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

pub fn to_data_sets(data: Vec<PowerGenerationAndConsumption>) -> Vec<PlottingTemplateDataSet> {
    let mut result_map: HashMap<GenerationKind, PlottingTemplateDataSet> = HashMap::new();

    for item in data {
        for kind in GenerationKind::all() {
            match &kind {
                GenerationKind::Biomass => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.biomass,
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
                GenerationKind::GridEmissionFactor => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.grid_emission_factor,
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
                GenerationKind::HardCoal => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.hard_coal,
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
                GenerationKind::Hydro => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.hydro,
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
                GenerationKind::Lignite => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.lignite,
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
                GenerationKind::NaturalGas => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.natural_gas,
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
                GenerationKind::Nuclear => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.nuclear,
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
                GenerationKind::Other => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.other,
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
                GenerationKind::PumpedStorageGeneration => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.pumped_storage_generation,
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
                GenerationKind::Solar => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.solar,
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
                GenerationKind::TotalConventionalPowerPlant => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.total_conventional_power_plant,
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
                GenerationKind::TotalElectricityDemand => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.total_electricity_demand,
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
                GenerationKind::TotalGridEmissions => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.total_grid_emissions,
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
                GenerationKind::WindOffshore => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.wind_offshore,
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
                GenerationKind::WindOnshore => {
                    let new_data = PlottingTemplateDataSetData {
                        x: item.date_id,
                        y: item.wind_onshore,
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
        }
    }

    result_map.into_values().collect()
}
