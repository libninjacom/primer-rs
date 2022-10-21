use serde_json::json;
use crate::model::*;
use crate::PrimerClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveClientSideTokenClientSessionGetRequest<'a> {
    pub(crate) client: &'a PrimerClient,
    pub client_token: Option<String>,
}
impl<'a> RetrieveClientSideTokenClientSessionGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ClientSessionApiResponse> {
        let mut r = self.client.client.get("/client-session");
        if let Some(ref unwrapped) = self.client_token {
            r = r.push_query("clientToken", &unwrapped.to_string());
        }
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
    pub fn client_token(mut self, client_token: &str) -> Self {
        self.client_token = Some(client_token.to_owned());
        self
    }
}
