pub static TEXT: &'static str = "use rocket_contrib::Template;
use view_models::*;

#[get(\"/\")]
pub fn index() -> Template {
    let context = ViewModel::new(DefaultHeader::default(), DefaultBody::default());
    Template::render(\"pages/index\", &context)
}
";