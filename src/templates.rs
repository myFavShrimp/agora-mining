use askama::Template;

static REFRESH_BUTTON_ID: &str = "refreshButton";

#[derive(Template)]
#[template(path = "landingpage/landingpage.html")]
pub struct HelloAgoraTemplate<'a> {
    pub text: &'a str,
}

#[derive(Template)]
#[template(path = "about/about.html")]
pub struct AboutTemplate;
