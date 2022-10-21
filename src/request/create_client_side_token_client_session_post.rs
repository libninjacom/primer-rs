use serde_json::json;
use crate::model::*;
use crate::PrimerClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateClientSideTokenClientSessionPostRequest<'a> {
    pub(crate) client: &'a PrimerClient,
    pub order_id: String,
    pub currency_code: String,
    pub amount: serde_json::Value,
    pub order: OrderDetailsApiSchema,
    pub customer_id: String,
    pub customer: CheckoutCustomerDetailsApiSchema,
    pub metadata: serde_json::Value,
    pub payment_method: CheckoutPaymentMethodOptionsApiSchema,
}
impl<'a> CreateClientSideTokenClientSessionPostRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ClientSessionWithTokenApiResponse> {
        let mut r = self.client.client.post("/client-session");
        r = r.push_json(json!({ "orderId" : self.order_id }));
        r = r.push_json(json!({ "currencyCode" : self.currency_code }));
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "order" : self.order }));
        r = r.push_json(json!({ "customerId" : self.customer_id }));
        r = r.push_json(json!({ "customer" : self.customer }));
        r = r.push_json(json!({ "metadata" : self.metadata }));
        r = r.push_json(json!({ "paymentMethod" : self.payment_method }));
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
pub struct CreateClientSideTokenClientSessionPostRequired<'a> {
    pub order_id: &'a str,
    pub currency_code: &'a str,
    pub amount: serde_json::Value,
    pub order: OrderDetailsApiSchema,
    pub customer_id: &'a str,
    pub customer: CheckoutCustomerDetailsApiSchema,
    pub metadata: serde_json::Value,
    pub payment_method: CheckoutPaymentMethodOptionsApiSchema,
}
impl<'a> CreateClientSideTokenClientSessionPostRequired<'a> {}
