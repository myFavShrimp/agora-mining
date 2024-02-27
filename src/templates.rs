use askama::Template;

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
}
