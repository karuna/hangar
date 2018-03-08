pub static TEXT: &'static str = "use std::default::Default;
use serde::Serialize;
use serde_json::Map;
use serde_json::value::Value;

pub mod users;

#[derive(Debug, Serialize)]
pub struct DefaultBody {
    pub content: String,
}

impl Default for DefaultBody {
    fn default() -> DefaultBody {
        DefaultBody {
            content: String::from(\"Insert content here...\"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DefaultHeader {
    pub title: String,
}

impl Default for DefaultHeader {
    fn default() -> DefaultHeader {
        DefaultHeader {
            title: String::from(\"Hangar\"),
        }
    }
}

/// This is the main placeholder for all view items to be used for rendering the templates.
/// There are 2 parts, Header and Body
#[derive(Debug, Serialize)]
pub struct ViewModel<H, B>
where
    H: Serialize,
    B: Serialize,
{
    pub header: H,
    pub body: B,
    pub payload: Map<String, Value>,
}

impl<H, B> ViewModel<H, B>
where
    H: Serialize,
    B: Serialize,
{
    pub fn new(header: H, body: B) -> Self {
        let payload: Map<String, Value> = Map::new();
        ViewModel { header, body, payload, }
    }
}
";
