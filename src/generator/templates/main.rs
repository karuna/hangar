pub static TEXT: &'static str = "extern crate hangar;

fn main() {
    let rocket = hangar::rocket_factory().unwrap();
    rocket.launch();
}
";
