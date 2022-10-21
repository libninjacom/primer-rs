use serde_json::json;
use crate::model::*;
use crate::PrimerClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreatePaymentPaymentsPostRequest<'a> {
    pub(crate) client: &'a PrimerClient,
    pub x_idempotency_key: Option<String>,
    pub order_id: Option<String>,
    pub currency_code: Option<String>,
    pub amount: Option<serde_json::Value>,
    pub order: Option<OrderDetailsApiSchema>,
    pub payment_method_token: String,
    pub customer_id: Option<String>,
    pub customer: Option<CustomerDetailsApiSchema>,
    pub metadata: Option<serde_json::Value>,
    pub payment_method: Option<PaymentRequestPaymentMethodOptionsApiSchema>,
}
impl<'a> CreatePaymentPaymentsPostRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentApiResponse> {
        let mut r = self.client.client.post("/payments");
        if let Some(ref unwrapped) = self.x_idempotency_key {
            r = r.header("X-Idempotency-Key", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.order_id {
            r = r.push_json(json!({ "orderId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.currency_code {
            r = r.push_json(json!({ "currencyCode" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.amount {
            r = r.push_json(json!({ "amount" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.order {
            r = r.push_json(json!({ "order" : unwrapped }));
        }
        r = r.push_json(json!({ "paymentMethodToken" : self.payment_method_token }));
        if let Some(ref unwrapped) = self.customer_id {
            r = r.push_json(json!({ "customerId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.push_json(json!({ "customer" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.metadata {
            r = r.push_json(json!({ "metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_method {
            r = r.push_json(json!({ "paymentMethod" : unwrapped }));
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
    pub fn x_idempotency_key(mut self, x_idempotency_key: &str) -> Self {
        self.x_idempotency_key = Some(x_idempotency_key.to_owned());
        self
    }
    pub fn order_id(mut self, order_id: &str) -> Self {
        self.order_id = Some(order_id.to_owned());
        self
    }
    pub fn currency_code(mut self, currency_code: &str) -> Self {
        self.currency_code = Some(currency_code.to_owned());
        self
    }
    pub fn amount(mut self, amount: serde_json::Value) -> Self {
        self.amount = Some(amount);
        self
    }
    pub fn order(mut self, order: OrderDetailsApiSchema) -> Self {
        self.order = Some(order);
        self
    }
    pub fn customer_id(mut self, customer_id: &str) -> Self {
        self.customer_id = Some(customer_id.to_owned());
        self
    }
    pub fn customer(mut self, customer: CustomerDetailsApiSchema) -> Self {
        self.customer = Some(customer);
        self
    }
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn payment_method(
        mut self,
        payment_method: PaymentRequestPaymentMethodOptionsApiSchema,
    ) -> Self {
        self.payment_method = Some(payment_method);
        self
    }
}
