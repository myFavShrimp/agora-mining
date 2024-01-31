use askama::Template;
use serde::Serialize;
use time::PrimitiveDateTime;

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
    pub data_sets: Vec<PlottingTemplateDataSet>,
    //pub labels: Vec<>,
}

#[derive(Serialize)]
pub struct PlottingTemplateDataSet {
    pub data: Vec<PlottingTemplateDataSetData>,
}
#[derive(Serialize)]
pub struct PlottingTemplateDataSetData {
    pub x: PrimitiveDateTime,
    pub y: f64,
}


