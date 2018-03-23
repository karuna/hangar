pub static TEXT: &'static str = "use std::fmt;

use argon2rs::argon2i_simple;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use schema::users;
use rand::{OsRng, Rng};

use guards::user::Registration;
use libs::db::ConnPool;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[table_name = \"users\"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub encrypted_password: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub access_token: Option<String>,
    pub last_access: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = \"users\"]
pub struct NewUser {
    pub email: String,
    pub encrypted_password: Vec<u8>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            \"User id: {id}, email: {email}\",
            id = self.id.to_string(),
            email = self.email
        )
    }
}

impl fmt::Display for NewUser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, \"User email: {email}\", email = self.email)
    }
}

impl User {
    // https://tools.ietf.org/html/draft-josefsson-argon2-00#section-1
    pub fn verify_password(&self, password: &str, salt: &str) -> bool {
        let hash = argon2i_simple(password, salt).to_vec();
        self.encrypted_password == hash
    }

    pub fn generate_access_token(&mut self, conn: &ConnPool) -> Result<String, Error> {
        let mut rand_gen = OsRng::new().expect(\"Couldn't make OsRng!\");
        let new_access_token = rand_gen.gen_ascii_chars().take(32).collect::<String>();
        self.access_token = Some(new_access_token.clone());
        self.last_access = Utc::now().naive_utc();
        self.save_changes::<User>(conn)?;
        Ok(new_access_token)
    }

    pub fn has_valid_access_token(&self, access_token_timeout: Duration) -> bool {
        let latest_valid_date = Utc::now() - access_token_timeout;
        if self.access_token.is_some() {
            self.last_access > latest_valid_date.naive_utc()
        } else {
            false
        }
    }

    pub fn from_id(id: i32, db: ConnPool) -> Result<Option<Self>, diesel::result::Error> {
        users::dsl::users
            .find(id)
            .first::<User>(&**db)
            .optional()
    }

    pub fn from_access_token(
        access_token: &str,
        db: ConnPool,
    ) -> Result<Option<Self>, diesel::result::Error> {
        users::dsl::users
            .filter(users::access_token.eq(access_token))
            .first::<User>(&**db)
            .optional()
    }
}

impl NewUser {
    fn make_encrypted_password(password: &str, salt: &str) -> Vec<u8> {
        argon2i_simple(password, salt).to_vec()
    }

    pub fn new_from_user_data(
        user_data: &Registration,
        salt: &str,
        db: &ConnPool,
    ) -> Result<User, diesel::result::Error> {
        let encrypted_password =
            NewUser::make_encrypted_password(user_data.password.as_str(), salt);

        let new_user = NewUser {
            email: user_data.email.clone().unwrap(),
            encrypted_password,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&***db);

        users::dsl::users.filter(users::dsl::email.eq(&new_user.email))
            .first::<User>(&***db)
    }
}
";
