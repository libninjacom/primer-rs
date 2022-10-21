#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
use primer_api::request::RefundPaymentPaymentsIdRefundPostRequired;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let args = RefundPaymentPaymentsIdRefundPostRequired {
        amount: ::serde_json::json!({}),
        reason: "your reason",
        id: "your id",
        order_id: "your order id",
    };
    let response = client
        .refund_payment_payments_id_refund_post(args)
        .x_idempotency_key("your x idempotency key")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
