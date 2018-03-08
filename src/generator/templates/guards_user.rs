pub static TEXT: &'static str = "use validator::Validate;

use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome, Request};

use libs::lib_const::CURRENT_USER_STR;

#[derive(Validate, Serialize, Deserialize, FromForm)]
pub struct Registration {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = \"8\"))]
    pub password: String,
    #[validate(must_match = \"password\")]
    pub password_confirmation: String,
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct CurrentUser(i32);

impl<'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<CurrentUser, ()> {
        request
            .cookies()
            .get_private(CURRENT_USER_STR)
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| CurrentUser(id))
            .or_forward(())
    }
}

impl CurrentUser {
    pub fn id(&self) -> i32 {
        self.0
    }
}
";
