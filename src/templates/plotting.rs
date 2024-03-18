use std::collections::HashMap;

use serde::Serialize;
use time::macros::format_description;

use crate::database::Entity;

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

pub fn to_data_sets<D, F>(data: Vec<D>) -> Vec<PlottingTemplateDataSet>
where
    D: Entity<F> + std::fmt::Debug,
    F: std::hash::Hash + std::cmp::Eq,
{
    let mut result_map: HashMap<F, PlottingTemplateDataSet> = HashMap::new();

    for item in data {
        for kind in D::all_fields() {
            let new_data = PlottingTemplateDataSetData {
                x: item
                    .id()
                    .format(format_description!("[day].[month].[year] [hour]:[minute]"))
                    .unwrap(),
                y: item.get_by_field(&kind),
            };

            if let Some(existing_data) = result_map.get_mut(&kind) {
                existing_data.data.push(new_data)
            } else {
                let new_data_set = PlottingTemplateDataSet {
                    label: D::chart_display_name(&kind),
                    data: vec![new_data],
                    unit: D::unit(&kind),
                };
                result_map.insert(kind, new_data_set);
            }
        }
    }

    result_map.into_values().collect()
}
