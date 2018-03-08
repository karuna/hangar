pub static TEXT: &'static str = "# Hangar
Rust web framework, using [Rocket](https://rocket.rs), [Diesel](https://diesel.rs), and [Yew](https://github.com/DenisKolodin/yew)

## Installation
- `git clone` this project
- copy `.env.local` to `.env`
- modify `.env`
- install diesel-cli `cargo install diesel-cli`
- install cargo-web `cargo install cargo-web`
- `rustup target add wasm32-unknown-emscripten`
- create database and run migrations `diesel setup`

## Running
### Server side
- copy `Rocket.toml.example` to `Rocket.toml`
- modify `Rocket.toml`
- in server folder `cargo run`

### Client side
";
