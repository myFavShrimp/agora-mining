use askama::Template;
use serde::Serialize;
use time::PrimitiveDateTime;
use crate::templates::plotting::PlottingTemplateDataSet;

mod plotting;

pub static REFRESH_BUTTON_ID: &str = "refreshButton";

#[derive(Template)]
#[template(path = "landingpage/landingpage.html")]
pub struct HelloAgoraTemplate<'a> {
    pub text: &'a str,
}

#[derive(Template)]
#[template(path = "about/about.html")]
pub struct AboutTemplate;

#[derive(Template)]
#[template(path = "plotting/plotting.html")]
pub struct PlottingTemplate {
    pub data_sets: Vec<PlottingTemplateRawData>,
    //pub labels: Vec<>,
}

#[derive(Serialize)]
pub struct PlottingTemplateRawData {
    pub label: String,
    pub data: Vec<(PrimitiveDateTime, f64)>
}