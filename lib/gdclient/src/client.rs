use crate::orientation::Orientation;
use crate::payload::{GdFeedbackRequestPayload, GdResponsePayload};
use crate::GdClientError;
use reqwest::Client;

pub struct GdClient(Client);

impl GdClient {
    pub fn new() -> Self {
        Self(Client::new())
    }

    /// Est-ce que `input` est de droite ou de gauche ?
    pub async fn gd(&self, input: &str) -> Result<Orientation, GdClientError> {
        let payload = format!("{{\"input\": \"{input}\"}}");
        const URL: &str = "https://degaucheoudedroite.delemazure.fr/api.php";
        let text = self.0.post(URL).body(payload).send().await?.text().await?;
        match serde_json::from_str::<GdResponsePayload>(&text).unwrap() {
            GdResponsePayload { message, code: 200 } => Ok(message.parse().unwrap()),
            GdResponsePayload { message, code } => Err(GdClientError::Client {
                message: message.to_string(),
                code,
            }),
        }
    }

    async fn gd_feedback(
        &self,
        feedback: &GdFeedbackRequestPayload<'_>,
    ) -> Result<(), GdClientError> {
        const URL: &str = "https://degaucheoudedroite.delemazure.fr/feedback.php";
        let payload = serde_json::to_string(feedback).unwrap();
        let text = self.0.post(URL).body(payload).send().await?.text().await?;
        match text.as_str() {
            "" => Ok(()),
            error => {
                let GdResponsePayload { message, code } = serde_json::from_str(error).unwrap();
                Err(GdClientError::Client {
                    message: message.to_string(),
                    code,
                })
            }
        }
    }

    /// Dit au serveur qu'on est d'accord avec sa réponse.
    pub async fn gd_agree(
        &self,
        input: &str,
        orientation: &Orientation,
    ) -> Result<(), GdClientError> {
        self.gd_feedback(&GdFeedbackRequestPayload {
            val: input,
            res: orientation,
            user: true,
        })
        .await
    }

    /// Dit au serveur qu'on est en désaccord avec sa réponse.
    pub async fn gd_disagree(
        &self,
        input: &str,
        orientation: &Orientation,
    ) -> Result<(), GdClientError> {
        self.gd_feedback(&GdFeedbackRequestPayload {
            val: input,
            res: orientation,
            user: false,
        })
        .await
    }
}

impl Default for GdClient {
    fn default() -> Self {
        Self::new()
    }
}
