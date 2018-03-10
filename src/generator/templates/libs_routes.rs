pub static TEXT: &'static str = "use rocket::{Rocket, Route};
use controllers::{pages_controller, users_controller};

pub fn routes(attached_rocket: Rocket) -> Rocket {
    attached_rocket
        .mount(\"/\", root_urls())
        .mount(\"/users\", user_urls())
}

fn root_urls() -> Vec<Route> {
    routes![
        pages_controller::index,
    ]
}

fn user_urls() -> Vec<Route> {
    routes![
        users_controller::failed_show,
        users_controller::show,
        users_controller::signin,
        users_controller::signup,
        users_controller::login,
        users_controller::logout,
        users_controller::register,
    ]
}
";
