use serde_json::json;
use crate::model::*;
use crate::PrimerClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ResumePaymentPaymentsIdResumePostRequest<'a> {
    pub(crate) client: &'a PrimerClient,
    pub id: String,
    pub resume_token: String,
}
impl<'a> ResumePaymentPaymentsIdResumePostRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentApiResponse> {
        let mut r = self
            .client
            .client
            .post(&format!("/payments/{id}/resume", id = self.id));
        r = r.push_json(json!({ "resumeToken" : self.resume_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
