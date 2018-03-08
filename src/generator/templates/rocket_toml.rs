pub static TEXT: &'static str = "# Except for the secret key, none of these are actually needed; Rocket has sane
# defaults. We show all of them here explicitly for demonstrative purposes.

[global.limits]
forms = 32768
json = 1048576 # this is an extra used by the json contrib module
msgpack = 1048576 # this is an extra used by the msgpack contrib module

[development]
address = \"localhost\"
port = 8000
workers = 4
log = \"normal\"
# don't use this key! generate your own and keep it private!
# generate using `openssl rand -base64 32`
secret_key = \"Jd1wnIKl0nRIV7TUJWNHcbn3HqtcaBe3uiVTTfIcGDg=\"
template_dir = \"src/views/\"
# Application_security
access_token_timeout_days = 30
# Don't use this key! generate your own and keep it private!
password_salt = \"zT0X/fdPU62zSJy3+vvnZg==\"
# Assets
assets_dir = \"src/assets\"
assets_host = \"localhost:8000\"
serve_assets = true
# Database
database_url = \"{{database_url}}\"
database_pool = 8
# Logging
terminal_logger = true
file_logger = true
file_logger_dir_path = \"../log/\"
log_level = \"debug\" # https://docs.rs/slog/2.1.1/slog/enum.Level.html
mailer = { enabled = true, transport = \"file\", mail_send_dir = \"/tmp\" }
";
