use serde::Serialize;

#[derive(Serialize)]
pub struct Link {
    description: String,
    uri: String,
}

impl Link {
    pub fn new(description: String, uri: String) -> Self {
        Self { description, uri }
    }
}
