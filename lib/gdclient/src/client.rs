use crate::cache::{History, Query};
use crate::orientation::Orientation;
use crate::payload::{GdFeedbackRequestPayload, GdResponsePayload};
use crate::GdClientError;
use reqwest::Client;

#[derive(Debug)]
pub struct GdClient<H> {
    inner: Client,
    cache: H,
}

impl<H> GdClient<H> {
    /// Dit au serveur qu'on est d'accord avec sa réponse.
    pub async fn gd_agree(
        &self,
        input: &str,
        orientation: &Orientation,
    ) -> Result<(), GdClientError> {
        feedback_req(
            &self.inner,
            &GdFeedbackRequestPayload {
                val: input,
                res: orientation,
                user: true,
            },
        )
        .await
    }

    /// Dit au serveur qu'on est en désaccord avec sa réponse.
    pub async fn gd_disagree(
        &self,
        input: &str,
        orientation: &Orientation,
    ) -> Result<(), GdClientError> {
        feedback_req(
            &self.inner,
            &GdFeedbackRequestPayload {
                val: input,
                res: orientation,
                user: false,
            },
        )
        .await
    }
}

#[derive(Debug)]
struct NoCache;

impl GdClient<NoCache> {
    pub fn uncached() -> Self {
        Self {
            inner: Client::new(),
            cache: NoCache,
        }
    }

    /// Est-ce que `input` est de droite ou de gauche ?
    pub async fn gd(&self, input: &str) -> Result<Orientation, GdClientError> {
        orientation_req(&self.inner, input).await
    }
}

impl<H: History> GdClient<H> {
    pub fn with_cache(cache: H) -> Self {
        Self {
            inner: Client::new(),
            cache,
        }
    }

    pub async fn gd(&mut self, input: &str) -> Result<Orientation, GdClientError> {
        let res = match self.cache.last_input(input) {
            Ok(Some(cached)) => Ok(cached.orientation),
            _ => orientation_req(&self.inner, input).await,
        };
        if let Ok(orientation) = res {
            let _ = self.cache.push(Query {
                input: input.to_string(),
                orientation,
            });
        }
        res
    }
}

async fn orientation_req(client: &Client, input: &str) -> Result<Orientation, GdClientError> {
    let payload = format!("{{\"input\": \"{input}\"}}");
    const URL: &str = "https://degaucheoudedroite.delemazure.fr/api.php";
    let text = client.post(URL).body(payload).send().await?.text().await?;
    match serde_json::from_str::<GdResponsePayload>(&text).unwrap() {
        GdResponsePayload { message, code: 200 } => Ok(message.parse().unwrap()),
        GdResponsePayload { message, code } => Err(GdClientError::Client {
            message: message.to_string(),
            code,
        }),
    }
}

async fn feedback_req(
    client: &Client,
    feedback: &GdFeedbackRequestPayload<'_>,
) -> Result<(), GdClientError> {
    const URL: &str = "https://degaucheoudedroite.delemazure.fr/feedback.php";
    let payload = serde_json::to_string(feedback).unwrap();
    let text = client.post(URL).body(payload).send().await?.text().await?;
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
