use serde_json::json;
use crate::model::*;
use crate::PrimerClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetPaymentByIdPaymentsIdGetRequest<'a> {
    pub(crate) client: &'a PrimerClient,
    pub id: String,
}
impl<'a> GetPaymentByIdPaymentsIdGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentApiResponse> {
        let mut r = self.client.client.get(&format!("/payments/{id}", id = self.id));
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
