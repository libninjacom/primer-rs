use serde_json::json;
use crate::model::*;
use crate::PrimerClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeletePaymentMethodPaymentMethodsTokenDeleteRequest<'a> {
    pub(crate) client: &'a PrimerClient,
    pub payment_method_token: String,
}
impl<'a> DeletePaymentMethodPaymentMethodsTokenDeleteRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<VerifiedMerchantPaymentMethodTokenApiResponse> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/payment-instruments/{payment_method_token}", payment_method_token =
                    self.payment_method_token
                ),
            );
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
