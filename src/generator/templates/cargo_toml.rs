pub static TEXT: &'static str = "[profile.dev]
codegen-units = 4

[package]
name = \"hangar\"
version = \"0.0.1\"
authors = [\"{{authors_name_email}}\"]

[lib]
name = \"hangar\"
path = \"src/lib.rs\"

[[bin]]
name = \"hangar_runner\"
path = \"src/main.rs\"

[[test]]
name = \"hangar_test\"
path = \"tests/lib.rs\"

[features]
default = [\"email\", \"{{cargo_db}}\", \"multipart_form\"]
diesel_default = [\"diesel/chrono\", \"diesel/serde_json\", \"r2d2\", \"r2d2-diesel\"]
diesel_postgres = [\"diesel_default\", \"diesel/postgres\", \"diesel_derives/postgres\"]
diesel_mysql = [\"diesel_default\", \"diesel/mysql\", \"diesel_derives/mysql\"]
diesel_sqlite = [\"diesel_default\", \"diesel/sqlite\", \"diesel_derives/sqlite\"]
multipart_form = [\"multipart\"]
email = [\"lettre\", \"email-format\", \"native-tls\"]

[dependencies]
# rocket
rocket = { git = \"https://github.com/SergioBenitez/Rocket.git\", branch = \"master\" }
rocket_codegen = { git = \"https://github.com/SergioBenitez/Rocket.git\", branch = \"master\" }
rocket_contrib = { git = \"https://github.com/SergioBenitez/Rocket.git\", branch = \"master\", default-features = false, features = [\"json\", \"tera_templates\"]}
# database
# valid features are : \"postgresql\", \"mysql\", \"sqlite\"
diesel = { version = \"1.1.1\", optional = true }
diesel_derives = { version = \"1.1.0\", optional = true }
r2d2 = { version = \"0.8.2\", optional = true }
r2d2-diesel = { version = \"1.0.0\", optional = true }

# serialization
serde = \"1\"
serde_json = \"1\"
serde_derive = \"1\"

uuid = { version = \"0.6.1\", features = [\"use_std\", \"serde\", \"v4\"] }
chrono = { version = \"0.4\", features = [\"serde\"] }

# password encryption + self salting
argon2rs = \"0.2\"

validator = \"0.6\"
validator_derive = \"0.6\"
cookie = \"0.10.1\"
ring = \"0.13.0-alpha\"
rand = \"0.4\"
clippy = {version = \"*\", optional = true}
tera = \"0.11\"

# Logging
slog = \"2.1.1\"
sloggers = \"0.2\"

# Error handling
error-chain = \"0.11.0\"

# html sanitizer
ammonia = \"1.1\"

# multipart upload
multipart = { version = \"0.14\", features = [\"server\", \"nightly\"], optional = true }

# email
lettre = { version = \"0.7\", optional = true }
email-format = { version = \"0.6\", features = [\"lettre\"], optional = true }
native-tls = { version = \"0.1.5\", optional = true }

[dev-dependencies]
quickcheck = \"0.5\"
stainless = { git = \"https://github.com/reem/stainless.git\" }
parking_lot = {version = \"0.5\", features = [\"nightly\"]}
";
