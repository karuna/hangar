# Hangar

[https://hangar-project.org/](https://hangar-project.org/)

Rust web framework, using [Rocket](https://rocket.rs), [Diesel](https://diesel.rs), and [stdweb](https://github.com/koute/stdweb)

## Installation
- Make sure you have rust installed. Use [rustup](https://www.rustup.rs/) if you don't have rust installed.
- Install rust nightly
  `rustup install nightly`
  `rustup default nightly`
- Install diesel for database access
  `cargo install diesel_cli`
- Install this crate
  `cargo install hangar`
- Create new web application
  `hangar new --name new_app --database sqlite --database-url db.sqlite`
- Inside the generated web application folder do initial migration
  `hangar db setup`
  `hangar db print-schema > src/schema.rs`
- Run the application
  `cargo run`
- In other terminal start assets packager
  `npm run start-js`
  `nmp run start-css`

