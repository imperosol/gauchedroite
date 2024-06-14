use crate::orientation::Orientation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct GdResponsePayload<'a> {
    #[serde(rename = "data")]
    pub(crate) message: &'a str,
    #[serde(rename = "success")]
    pub(crate) code: u32,
}

#[derive(Debug, Serialize)]
pub(crate) struct GdFeedbackRequestPayload<'a> {
    pub(crate) val: &'a str,
    pub(crate) res: &'a Orientation,
    pub(crate) user: bool,
}
