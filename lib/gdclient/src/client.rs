use reqwest::Client;
use crate::{FetchError, Orientation};
use crate::payload::ResponsePayload;

pub struct GdClient(Client);

impl GdClient {
    pub fn new() -> Self {
        Self(Client::new())
    }

    pub async fn gd(&self, input: &str) -> Result<Orientation, FetchError> {
        const URL: &str = "https://degaucheoudedroite.delemazure.fr/api.php";
        let payload = format!("{{\"input\": \"{input}\"}}");
        let text = self.0.post(URL).body(payload).send().await?.text().await?;
        match serde_json::from_str::<ResponsePayload>(&text).unwrap() {
            ResponsePayload { message, code: 200 } => Ok(message.parse().unwrap()),
            ResponsePayload { message, code } => Err(FetchError::Client { message, code }),
        }
    }
}
