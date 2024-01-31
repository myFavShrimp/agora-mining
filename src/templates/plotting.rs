use serde::Serialize;
use time::PrimitiveDateTime;
use crate::templates::PlottingTemplate;

#[derive(Serialize)]
pub struct PlottingTemplateDataSet {
    pub data: Vec<PlottingTemplateDataSetData>,
}

#[derive(Serialize)]
pub struct PlottingTemplateDataSetData {
    pub x: PrimitiveDateTime,
    pub y: f64,
}

impl PlottingTemplate {
    pub fn labels(&self) -> serde_json::Value {
        let labels_unserialized: Vec<String> = self.data_sets.iter().map(|data| data.label.clone()).collect();
        serde_json::to_value(labels_unserialized).unwrap()
    }
}
