//! [`PrimerClient`](struct.PrimerClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;

pub struct PrimerClient {
    pub(crate) client: httpclient::Client,
    authentication: PrimerAuthentication,
}
impl PrimerClient {
    pub fn from_env() -> Self {
        let url = std::env::var("PRIMER_BASE_URL")
            .expect("Missing environment variable PRIMER_BASE_URL");
        Self {
            client: httpclient::Client::new(Some(url)),
            authentication: PrimerAuthentication::from_env(),
        }
    }
}
impl PrimerClient {
    pub fn new(url: &str, authentication: PrimerAuthentication) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: PrimerAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            PrimerAuthentication::ApiKeyAuth { api_key_auth } => {
                r = r.header("X-API-KEY", api_key_auth);
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    /**Retrieve a client session

This API call retrieves all the details associated with the client session corresponding to the client token that is provided in the request. The fields with empty values are excluded from the response.
*/
    pub fn retrieve_client_side_token_client_session_get(
        &self,
    ) -> request::RetrieveClientSideTokenClientSessionGetRequest {
        request::RetrieveClientSideTokenClientSessionGetRequest {
            client: &self,
            client_token: None,
        }
    }
    /**Create a client session

Creating a client session generates a client token: a temporary key used to initialize [Universal Checkout](https://primer.io/docs/accept-payments/setup-universal-checkout/installation/web) and authenticate it against your account.

Universal Checkout automatically retrieves all the settings from the client session and the Dashboard to configure the payment methods and the checkout experience.

<b>Note:</b>
When creating a Client Session, please make sure to provide `currencyCode`, `orderId`, and at least one of `amount` or `lineItems`.
If any of these are not yet available, you can provide them when making the payment request.

<code>POST /client-session</code> does not have required fields as all fields are not always known when a client session is created.
Use <code>PATCH /client-session</code> to update the parameters throughout the checkout session.

Client tokens expire after 24 hours.
*/
    pub fn create_client_side_token_client_session_post(
        &self,
        args: request::CreateClientSideTokenClientSessionPostRequired,
    ) -> request::CreateClientSideTokenClientSessionPostRequest {
        request::CreateClientSideTokenClientSessionPostRequest {
            client: &self,
            order_id: args.order_id.to_owned(),
            currency_code: args.currency_code.to_owned(),
            amount: args.amount,
            order: args.order,
            customer_id: args.customer_id.to_owned(),
            customer: args.customer,
            metadata: args.metadata,
            payment_method: args.payment_method,
        }
    }
    /**Update client session

You can update a clients session created earlier with the `PATCH /client-session` [API call](#operation/create_client_side_token_client_session_post).

The only required field for the request is `clientToken`. Other supported request fields are same as for the `POST /client-session` [API call](#operation/create_client_side_token_client_session_post).

You need to specify only the fields you wish to update. However, if the items that are to be updated are of type `array`, then you need to provide the complete array along with modified items.

If you wish to update nested fields on the client session, such as the customer `emailAddress` field, you can pass the `customer` object with only one field, `emailAddress`, to update.

If you simply wish to clear the value of the field, pass `null` as your input.

You can update `paymentMethod.vaultOnSuccess` field but updating of the `paymentMethod.options` field through `PATCH /client-session` is not supported.

The response will contain all the fields of the client session including the ones that were changed.
*/
    pub fn update_client_side_token_client_session_patch(
        &self,
        args: request::UpdateClientSideTokenClientSessionPatchRequired,
    ) -> request::UpdateClientSideTokenClientSessionPatchRequest {
        request::UpdateClientSideTokenClientSessionPatchRequest {
            client: &self,
            client_token: args.client_token.to_owned(),
            customer_id: args.customer_id.to_owned(),
            order_id: args.order_id.to_owned(),
            currency_code: args.currency_code.to_owned(),
            amount: args.amount,
            metadata: args.metadata,
            customer: args.customer,
            order: args.order,
            payment_method: args.payment_method,
        }
    }
    /**Search & list payments

<p/>

Retrieve a list of your payments.

Results are paginated, they will only return up to 100 payments maximum.
To access the next page of result, set the `cursor` query parameter to the value of `nextCursor` in
your current result payload. Use `prevCursor` to go back to the previous page.

**Note:** this endpoint returns a list of
summarized payments. Not all payments attributes are present. You can use
the query parameters to filter payments. You can separate multiple query parameters with the `&` symbol.
Query parameters with types of the form "Array of strings" (such as the status parameter) can be specified as a comma-separated list.

For example, if you wanted to get both `FAILED`  and `CANCELLED` payments, for customer `john-123`, you would use:
```bash
curl --location --request GET 'https://api.primer.io/payments?status=FAILED,CANCELLED&customer_id=john-123' \
--header 'X-Api-Key: <YOUR_API_KEY>'
```

You can alternatively specify a list by repeating the parameter multiple times.

**Note:** payments will be available within a minute from being created.
*/
    pub fn list_payments_payments_get(&self) -> request::ListPaymentsPaymentsGetRequest {
        request::ListPaymentsPaymentsGetRequest {
            client: &self,
            status: None,
            payment_method_type: None,
            processor: None,
            currency_code: None,
            from_date: None,
            to_date: None,
            order_id: None,
            min_amount: None,
            max_amount: None,
            customer_id: None,
            merchant_id: None,
            customer_email_address: None,
            last4_digits: None,
            paypal_email: None,
            klarna_email: None,
            limit: None,
            cursor: None,
        }
    }
    /**Create a payment

<p/>

Create and authorize a payment for a given customer order. You
should provide a payment method token here to avoid PCI implications.

If only a payment method token is passed, the values passed with the Client Session is used to determine the amount, currency etc.
Note: `amount`, `currencyCode` and `orderId` are required during payment creation. Make sure to pass these fields when creating a client session, or if not yet available, when creating a payment.

All fields provided on this request will take preference over any field on the `order` associated with the client session. E.g. if you pass `amount` on this request, it will override the `amount` on the `order` associated with the Client Session.
*/
    pub fn create_payment_payments_post(
        &self,
        payment_method_token: &str,
    ) -> request::CreatePaymentPaymentsPostRequest {
        request::CreatePaymentPaymentsPostRequest {
            client: &self,
            x_idempotency_key: None,
            order_id: None,
            currency_code: None,
            amount: None,
            order: None,
            payment_method_token: payment_method_token.to_owned(),
            customer_id: None,
            customer: None,
            metadata: None,
            payment_method: None,
        }
    }
    /**Capture a payment

<p/>

If you have successfully authorized a payment, you can now
fully capture, or partially capture funds from the authorized payment, depending
on whether your selected payment processor supports it. The payment will
be updated to `SETTLED` or `SETTLING`, depending on the payment method type.

The payload sent in this capture request is completely optional. If you don't
send a payload with the capture request, the full amount that was authorized
will be sent for capture. Below are the available payload attributes, which
give you more granular control when capturing funds, if you require it.
*/
    pub fn capture_payment_payments_id_capture_post(
        &self,
        id: &str,
        amount: serde_json::Value,
        final_: bool,
    ) -> request::CapturePaymentPaymentsIdCapturePostRequest {
        request::CapturePaymentPaymentsIdCapturePostRequest {
            client: &self,
            id: id.to_owned(),
            x_idempotency_key: None,
            amount,
            final_,
        }
    }
    /**Cancel a payment

<p/>

Provided the payment has not reached `SETTLED` status, Primer will
send a "void" request to the payment processor, thereby cancelling the payment
and releasing the hold on customer funds. Upon success, the payment will transition
to `CANCELLED`. The payload is optional.
*/
    pub fn cancel_payment_payments_id_cancel_post(
        &self,
        id: &str,
        reason: &str,
    ) -> request::CancelPaymentPaymentsIdCancelPostRequest {
        request::CancelPaymentPaymentsIdCancelPostRequest {
            client: &self,
            id: id.to_owned(),
            x_idempotency_key: None,
            reason: reason.to_owned(),
        }
    }
    /**Refund a payment

<p/>

By default, this request will refund the full amount.

Optionally, pass in a lesser amount for a partial refund.
*/
    pub fn refund_payment_payments_id_refund_post(
        &self,
        args: request::RefundPaymentPaymentsIdRefundPostRequired,
    ) -> request::RefundPaymentPaymentsIdRefundPostRequest {
        request::RefundPaymentPaymentsIdRefundPostRequest {
            client: &self,
            id: args.id.to_owned(),
            x_idempotency_key: None,
            amount: args.amount,
            order_id: args.order_id.to_owned(),
            reason: args.reason.to_owned(),
        }
    }
    /**Resume a payment

<p/>

Resume a payment's workflow execution from a paused state. This
is usually required when a Workflow was paused in order to get further information
from the customer, or when waiting for an asynchronous response from a third
party connection.
*/
    pub fn resume_payment_payments_id_resume_post(
        &self,
        id: &str,
        resume_token: &str,
    ) -> request::ResumePaymentPaymentsIdResumePostRequest {
        request::ResumePaymentPaymentsIdResumePostRequest {
            client: &self,
            id: id.to_owned(),
            resume_token: resume_token.to_owned(),
        }
    }
    /**Get a payment

<p/>

Retrieve a payment by its ID.
*/
    pub fn get_payment_by_id_payments_id_get(
        &self,
        id: &str,
    ) -> request::GetPaymentByIdPaymentsIdGetRequest {
        request::GetPaymentByIdPaymentsIdGetRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Save a payment method token

<p/>

Save a `SINGLE_USE` payment method token so it can be used
again later. You can optionally choose to verify the payment method
before vaulting. If verification fails, no payment method data will
be vaulted. Verification can minimise fraud and boost subscription
rates for recurring payments.

If you try to vault an already vaulted token, you will get the existing vaulted token back.
*/
    pub fn vault_payment_method_payment_methods_token_vault_post(
        &self,
        payment_method_token: &str,
        customer_id: &str,
    ) -> request::VaultPaymentMethodPaymentMethodsTokenVaultPostRequest {
        request::VaultPaymentMethodPaymentMethodsTokenVaultPostRequest {
            client: &self,
            payment_method_token: payment_method_token.to_owned(),
            customer_id: customer_id.to_owned(),
            verify: None,
        }
    }
    /**List saved payment methods

Retrieve a list of stored payment methods for a customer.*/
    pub fn get_payment_methods_payment_methods_get(
        &self,
        customer_id: &str,
    ) -> request::GetPaymentMethodsPaymentMethodsGetRequest {
        request::GetPaymentMethodsPaymentMethodsGetRequest {
            client: &self,
            customer_id: customer_id.to_owned(),
        }
    }
    /**Delete a saved payment method

Delete a saved payment method.*/
    pub fn delete_payment_method_payment_methods_token_delete(
        &self,
        payment_method_token: &str,
    ) -> request::DeletePaymentMethodPaymentMethodsTokenDeleteRequest {
        request::DeletePaymentMethodPaymentMethodsTokenDeleteRequest {
            client: &self,
            payment_method_token: payment_method_token.to_owned(),
        }
    }
    /**Update the default saved payment method

Update a saved payment method to be the default stored payment method for a customer.*/
    pub fn set_payment_method_default_payment_methods_token_default_post(
        &self,
        payment_method_token: &str,
    ) -> request::SetPaymentMethodDefaultPaymentMethodsTokenDefaultPostRequest {
        request::SetPaymentMethodDefaultPaymentMethodsTokenDefaultPostRequest {
            client: &self,
            payment_method_token: payment_method_token.to_owned(),
        }
    }
}
pub enum PrimerAuthentication {
    ApiKeyAuth { api_key_auth: String },
}
impl PrimerAuthentication {
    pub fn from_env() -> Self {
        Self::ApiKeyAuth {
            api_key_auth: std::env::var("PRIMER_API_KEY_AUTH")
                .expect("Environment variable PRIMER_API_KEY_AUTH is not set."),
        }
    }
}
