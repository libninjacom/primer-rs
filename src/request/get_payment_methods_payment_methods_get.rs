use serde_json::json;
use crate::model::*;
use crate::PrimerClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetPaymentMethodsPaymentMethodsGetRequest<'a> {
    pub(crate) client: &'a PrimerClient,
    pub customer_id: String,
}
impl<'a> GetPaymentMethodsPaymentMethodsGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<VerifiedMerchantPaymentMethodTokenListApiResponse> {
        let mut r = self.client.client.get("/payment-instruments");
        r = r.push_query("customer_id", &self.customer_id.to_string());
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
