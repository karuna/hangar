pub static TEXT: &'static str = "use rocket::{Rocket, Route};
use controllers::{pages_controller};

pub fn routes(attached_rocket: Rocket) -> Rocket {
    attached_rocket
        .mount(\"/\", root_urls())
}

fn root_urls() -> Vec<Route> {
    routes![
        pages_controller::index,
    ]
}
";
