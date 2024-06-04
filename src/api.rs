use serde::{Deserialize, Serialize};

pub enum Orientation {
    Gauche,
    Droite,
}

#[derive(Debug, Serialize)]
pub struct RequestPayload {
    input: String
}

#[derive(Debug, Deserialize)]
pub struct ResponsePayload {
    data: String,
    success: u32
}

pub async fn get_api_result(input: &str) -> Orientation {
    todo!()
}
