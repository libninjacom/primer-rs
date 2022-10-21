use serde_json::json;
use crate::model::*;
use crate::PrimerClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RefundPaymentPaymentsIdRefundPostRequest<'a> {
    pub(crate) client: &'a PrimerClient,
    pub id: String,
    pub x_idempotency_key: Option<String>,
    pub amount: serde_json::Value,
    pub order_id: String,
    pub reason: String,
}
impl<'a> RefundPaymentPaymentsIdRefundPostRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentApiResponse> {
        let mut r = self
            .client
            .client
            .post(&format!("/payments/{id}/refund", id = self.id));
        if let Some(ref unwrapped) = self.x_idempotency_key {
            r = r.header("X-Idempotency-Key", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "orderId" : self.order_id }));
        r = r.push_json(json!({ "reason" : self.reason }));
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
    pub fn x_idempotency_key(mut self, x_idempotency_key: &str) -> Self {
        self.x_idempotency_key = Some(x_idempotency_key.to_owned());
        self
    }
}
pub struct RefundPaymentPaymentsIdRefundPostRequired<'a> {
    pub id: &'a str,
    pub amount: serde_json::Value,
    pub order_id: &'a str,
    pub reason: &'a str,
}
impl<'a> RefundPaymentPaymentsIdRefundPostRequired<'a> {}
