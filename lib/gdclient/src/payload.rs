use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ResponsePayload {
    #[serde(rename = "data")]
    pub(crate) message: String,
    #[serde(rename = "success")]
    pub(crate) code: u32,
}
