#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
use primer_api::request::CreateClientSideTokenClientSessionPostRequired;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let args = CreateClientSideTokenClientSessionPostRequired {
        order: OrderDetailsApiSchema {
            line_items: Some(
                vec![
                    OrderLineItemsApiSchema { amount : ::serde_json::json!({}), item_id :
                    Some("your item id".to_owned()), name : Some("your name".to_owned()),
                    product_data : Some(OrderLineItemsProductDataApiSchema {
                    manufacturer_part_number : Some("your manufacturer part number"
                    .to_owned()), weight : Some(1.0), brand : Some("your brand"
                    .to_owned()), color : Some("your color".to_owned()),
                    global_trade_item_number : Some("your global trade item number"
                    .to_owned()), weight_unit : Some("your weight unit".to_owned()), sku
                    : Some("your sku".to_owned()) }), discount_amount :
                    Some(::serde_json::json!({})), quantity : Some(1), tax_code :
                    Some("your tax code".to_owned()), tax_amount : Some(1), description :
                    Some("your description".to_owned()), product_type :
                    Some("your product type".to_owned()) }
                ],
            ),
            country_code: Some(CountryCodeEnum(::serde_json::json!({}))),
            fees: Some(
                vec![
                    OrderFeesApiSchema { amount : ::serde_json::json!({}), type_ :
                    Some("your type".to_owned()), description : Some("your description"
                    .to_owned()) }
                ],
            ),
            shipping: Some(OrderShippingApiSchema {
                amount: Some(::serde_json::json!({})),
            }),
        },
        metadata: ::serde_json::json!({}),
        customer: CheckoutCustomerDetailsApiSchema {
            email_address: Some("your email address".to_owned()),
            mobile_number: Some("your mobile number".to_owned()),
            tax_id: Some("your tax id".to_owned()),
            first_name: Some("your first name".to_owned()),
            billing_address: Some(CoreApiApiCommonsSchemasAddessAddressApiSchema {
                postal_code: Some("your postal code".to_owned()),
                address_line1: Some("your address line 1".to_owned()),
                first_name: Some("your first name".to_owned()),
                last_name: Some("your last name".to_owned()),
                state: Some("your state".to_owned()),
                address_line2: Some("your address line 2".to_owned()),
                city: Some("your city".to_owned()),
                country_code: Some(CountryCodeEnum(::serde_json::json!({}))),
            }),
            shipping_address: Some(CoreApiApiCommonsSchemasAddessAddressApiSchema {
                postal_code: Some("your postal code".to_owned()),
                address_line1: Some("your address line 1".to_owned()),
                first_name: Some("your first name".to_owned()),
                last_name: Some("your last name".to_owned()),
                state: Some("your state".to_owned()),
                address_line2: Some("your address line 2".to_owned()),
                city: Some("your city".to_owned()),
                country_code: Some(CountryCodeEnum(::serde_json::json!({}))),
            }),
            national_document_id: Some("your national document id".to_owned()),
            last_name: Some("your last name".to_owned()),
        },
        customer_id: "your customer id",
        payment_method: CheckoutPaymentMethodOptionsApiSchema {
            descriptor: Some("your descriptor".to_owned()),
            payment_type: Some("your payment type".to_owned()),
            options: Some(::serde_json::json!({})),
            vault_on_success: Some(true),
        },
        currency_code: "your currency code",
        order_id: "your order id",
        amount: ::serde_json::json!({}),
    };
    let response = client
        .create_client_side_token_client_session_post(args)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
