pub static TEXT: &'static str = "use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use validator::Validate;

use rocket::http::{Cookie, Cookies};
use rocket::request::{Form, LenientForm};
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_contrib::Template;

use guards::user::{CurrentUser, Login, Registration};
use libs::lib_const::CURRENT_USER_STR;
use libs::db::ConnPool;
use libs::settings::Settings;
use models::user::{NewUser, User};
use schema::users::dsl::{email, users};
use view_models::users::{Signin, Signup};
use view_models::*;

// Sample of showing user information using GET
#[get(\"/<id>\", format = \"text/html\")]
pub fn show(
    id: i32,
    db: ConnPool,
    _current_user: CurrentUser,
) -> Result<Template, Flash<Redirect>> {
    let user_query = users.find(&id).first::<User>(&**db).optional();

    if let Ok(Some(user)) = user_query {
        let header = DefaultHeader {
            title: String::from(\"User\"),
        };
        let context = ViewModel::new(&header, &user);
        Ok(Template::render(\"users/show\", &context))
    } else {
        Err(Flash::error(Redirect::to(\"/\"), \"Unauthorized.\"))
    }
}

// Sample of showing user information using GET
#[get(\"/<_id>\", rank = 2, format = \"text/html\")]
pub fn failed_show(_id: i32) -> Redirect {
    Redirect::to(\"users/signin\")
}

#[get(\"/signin\", format = \"text/html\")]
pub fn signin() -> Template {
    render_signin()
}

#[get(\"/signup\", format = \"text/html\")]
pub fn signup() -> Template {
    render_signup(None)
}

#[post(\"/login\", data = \"<user>\", format = \"application/x-www-form-urlencoded\")]
pub fn login(
    mut cookies: Cookies,
    user: Form<Login>,
    db: ConnPool,
    settings: State<Settings>,
) -> Result<Flash<Redirect>, Template> {
    let user_data = user.get();

    let user_query = users
        .filter(email.eq(&user_data.email))
        .first::<User>(&**db)
        .optional();

    if let Ok(Some(user_result)) = user_query {
        let salt = &settings.application_security.password_salt;
        if !user_result.verify_password(&user_data.password, salt) {
            return Err(render_signin());
        };

        cookies.add_private(Cookie::new(
            CURRENT_USER_STR,
            user_result.id.to_string(),
        ));
        Ok(Flash::success(Redirect::to(\"/\"), \"Successfully login.\"))
    } else {
        Err(render_signin())
    }
}

#[get(\"/logout\", format = \"text/html\")]
pub fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named(CURRENT_USER_STR));
    Flash::success(Redirect::to(\"/\"), \"Successfully logout.\")
}

#[post(\"/register\", data = \"<user>\", format = \"application/x-www-form-urlencoded\")]
pub fn register(
    user: LenientForm<Registration>,
    db: ConnPool,
    settings: State<Settings>,
) -> Result<Flash<Redirect>, Template> {
    let user_data = user.into_inner();
    if user_data.validate().is_err() {
        return Err(render_signup(user_data.email));
    };

    let salt = &settings.application_security.password_salt;

    let new_user = NewUser::new_from_user_data(&user_data, salt, &db);
    if let Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) = new_user {
        return Err(render_signup(None));
    }

    Ok(Flash::success(
        Redirect::to(\"/\"),
        \"Successfully create user.\",
    ))
}

fn render_signup(registration_email: Option<String>) -> Template {
    let header = DefaultHeader {
        title: String::from(\"Signup\"),
    };
    let user_signup = Signup {
        email: registration_email,
    };
    let context = ViewModel::new(&header, &user_signup);
    Template::render(\"users/signup\", &context)
}

fn render_signin() -> Template {
    let header = DefaultHeader {
        title: String::from(\"Signin\"),
    };
    let user_signin = Signin {};
    let context = ViewModel::new(&header, &user_signin);
    Template::render(\"users/signin\", &context)
}
";