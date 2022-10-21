#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let response = client
        .list_payments_payments_get()
        .status(&["your status"])
        .payment_method_type(&["your payment method type"])
        .processor(&["your processor"])
        .currency_code(&["your currency code"])
        .from_date("your from date")
        .to_date("your to date")
        .order_id("your order id")
        .min_amount(1)
        .max_amount(1)
        .customer_id(&["your customer id"])
        .merchant_id(&["your merchant id"])
        .customer_email_address(&["your customer email address"])
        .last4_digits(&["your last 4 digits"])
        .paypal_email(&["your paypal email"])
        .klarna_email(&["your klarna email"])
        .limit(::serde_json::json!({}))
        .cursor("your cursor")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
