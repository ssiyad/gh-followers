use askama::Template;
use askama_web::WebTemplate;
use ghf::Followee;

#[derive(Template, WebTemplate)]
#[template(path = "table_ghosts.html")]
pub struct TableGhosts {
    pub ghosts: Vec<Followee>,
}
