pub static TEXT: &'static str = "#[derive(Serialize)]
pub struct Signin {}
#[derive(Serialize)]
pub struct Signup {
    pub email: Option<String>,
}
";
