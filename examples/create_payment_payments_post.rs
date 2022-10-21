#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let payment_method_token = "your payment method token";
    let response = client
        .create_payment_payments_post(payment_method_token)
        .x_idempotency_key("your x idempotency key")
        .order_id("your order id")
        .currency_code("your currency code")
        .amount(::serde_json::json!({}))
        .order(OrderDetailsApiSchema {
            line_items: Some(
                vec![
                    OrderLineItemsApiSchema { tax_code : Some("your tax code"
                    .to_owned()), item_id : Some("your item id".to_owned()), description
                    : Some("your description".to_owned()), quantity : Some(1), name :
                    Some("your name".to_owned()), amount : ::serde_json::json!({}),
                    tax_amount : Some(1), discount_amount :
                    Some(::serde_json::json!({})), product_data :
                    Some(OrderLineItemsProductDataApiSchema { weight : Some(1.0),
                    weight_unit : Some("your weight unit".to_owned()), brand :
                    Some("your brand".to_owned()), sku : Some("your sku".to_owned()),
                    global_trade_item_number : Some("your global trade item number"
                    .to_owned()), manufacturer_part_number :
                    Some("your manufacturer part number".to_owned()), color :
                    Some("your color".to_owned()) }), product_type :
                    Some("your product type".to_owned()) }
                ],
            ),
            fees: Some(
                vec![
                    OrderFeesApiSchema { type_ : Some("your type".to_owned()), amount :
                    ::serde_json::json!({}), description : Some("your description"
                    .to_owned()) }
                ],
            ),
            shipping: Some(OrderShippingApiSchema {
                amount: Some(::serde_json::json!({})),
            }),
            country_code: Some(CountryCodeEnum(::serde_json::json!({}))),
        })
        .customer_id("your customer id")
        .customer(CustomerDetailsApiSchema {
            billing_address: Some(CoreApiApiCommonsSchemasAddessAddressApiSchema {
                address_line1: Some("your address line 1".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                country_code: Some(CountryCodeEnum(::serde_json::json!({}))),
                address_line2: Some("your address line 2".to_owned()),
                last_name: Some("your last name".to_owned()),
                first_name: Some("your first name".to_owned()),
                city: Some("your city".to_owned()),
                state: Some("your state".to_owned()),
            }),
            shipping_address: Some(CoreApiApiCommonsSchemasAddessAddressApiSchema {
                address_line1: Some("your address line 1".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                country_code: Some(CountryCodeEnum(::serde_json::json!({}))),
                address_line2: Some("your address line 2".to_owned()),
                last_name: Some("your last name".to_owned()),
                first_name: Some("your first name".to_owned()),
                city: Some("your city".to_owned()),
                state: Some("your state".to_owned()),
            }),
            email_address: Some("your email address".to_owned()),
            mobile_number: Some("your mobile number".to_owned()),
            last_name: Some("your last name".to_owned()),
            national_document_id: Some("your national document id".to_owned()),
            first_name: Some("your first name".to_owned()),
            tax_id: Some("your tax id".to_owned()),
        })
        .metadata(::serde_json::json!({}))
        .payment_method(PaymentRequestPaymentMethodOptionsApiSchema {
            vault_on_success: Some(true),
            descriptor: Some("your descriptor".to_owned()),
            payment_type: Some("your payment type".to_owned()),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
