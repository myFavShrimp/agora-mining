use crate::database::{agora_entities::AgoraEntities, Average};
use askama::Template;
use time::{macros::format_description, Date};
pub mod plotting;

pub static REFRESH_BUTTON_ID: &str = "refreshButton";

#[derive(Template)]
#[template(path = "landingpage/landingpage.html")]
pub struct LandingPageTemplate;

#[derive(Template)]
#[template(path = "about/about.html")]
pub struct AboutTemplate;

#[derive(Template)]
#[template(path = "plotting/plotting.html")]
pub struct PlottingTemplate {
    pub data_sets: Vec<plotting::PlottingTemplateDataSet>,
    pub from: Date,
    pub to: Date,
    pub used_data_sets: Vec<AgoraEntities>,
}

fn format_date(date: &Date) -> Result<String, time::error::Format> {
    let format = format_description!("[year]-[month]-[day]");
    date.format(&format)
}

impl PlottingTemplate {
    fn formatted_to(&self) -> Result<String, time::error::Format> {
        format_date(&self.to)
    }

    fn formatted_from(&self) -> Result<String, time::error::Format> {
        format_date(&self.from)
    }
}
