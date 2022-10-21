#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
use primer_api::request::CreateClientSideTokenClientSessionPostRequired;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let args = CreateClientSideTokenClientSessionPostRequired {
        order_id: "your order id",
        metadata: ::serde_json::json!({}),
        currency_code: "your currency code",
        customer_id: "your customer id",
        order: OrderDetailsApiSchema {
            fees: Some(
                vec![
                    OrderFeesApiSchema { type_ : Some("your type".to_owned()), amount :
                    ::serde_json::json!({}), description : Some("your description"
                    .to_owned()) }
                ],
            ),
            country_code: Some(CountryCodeEnum(::serde_json::json!({}))),
            shipping: Some(OrderShippingApiSchema {
                amount: Some(::serde_json::json!({})),
            }),
            line_items: Some(
                vec![
                    OrderLineItemsApiSchema { item_id : Some("your item id".to_owned()),
                    quantity : Some(1), tax_code : Some("your tax code".to_owned()), name
                    : Some("your name".to_owned()), amount : ::serde_json::json!({}),
                    product_data : Some(OrderLineItemsProductDataApiSchema { brand :
                    Some("your brand".to_owned()), sku : Some("your sku".to_owned()),
                    color : Some("your color".to_owned()), global_trade_item_number :
                    Some("your global trade item number".to_owned()), weight_unit :
                    Some("your weight unit".to_owned()), manufacturer_part_number :
                    Some("your manufacturer part number".to_owned()), weight : Some(1.0)
                    }), product_type : Some("your product type".to_owned()),
                    discount_amount : Some(::serde_json::json!({})), tax_amount :
                    Some(1), description : Some("your description".to_owned()) }
                ],
            ),
        },
        customer: CheckoutCustomerDetailsApiSchema {
            last_name: Some("your last name".to_owned()),
            email_address: Some("your email address".to_owned()),
            shipping_address: Some(CoreApiApiCommonsSchemasAddessAddressApiSchema {
                address_line1: Some("your address line 1".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                address_line2: Some("your address line 2".to_owned()),
                last_name: Some("your last name".to_owned()),
                state: Some("your state".to_owned()),
                first_name: Some("your first name".to_owned()),
                city: Some("your city".to_owned()),
                country_code: Some(CountryCodeEnum(::serde_json::json!({}))),
            }),
            billing_address: Some(CoreApiApiCommonsSchemasAddessAddressApiSchema {
                address_line1: Some("your address line 1".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                address_line2: Some("your address line 2".to_owned()),
                last_name: Some("your last name".to_owned()),
                state: Some("your state".to_owned()),
                first_name: Some("your first name".to_owned()),
                city: Some("your city".to_owned()),
                country_code: Some(CountryCodeEnum(::serde_json::json!({}))),
            }),
            tax_id: Some("your tax id".to_owned()),
            first_name: Some("your first name".to_owned()),
            mobile_number: Some("your mobile number".to_owned()),
            national_document_id: Some("your national document id".to_owned()),
        },
        payment_method: CheckoutPaymentMethodOptionsApiSchema {
            vault_on_success: Some(true),
            descriptor: Some("your descriptor".to_owned()),
            payment_type: Some("your payment type".to_owned()),
            options: Some(::serde_json::json!({})),
        },
        amount: ::serde_json::json!({}),
    };
    let response = client
        .create_client_side_token_client_session_post(args)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
