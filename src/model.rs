use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct KlarnaPaymentSessionApiSchema {
    pub klarna_authorization_token: String,
    pub session_data: KlarnaSessionDetailsApiSchema,
}
impl std::fmt::Display for KlarnaPaymentSessionApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "AUTHORIZED")]
    Authorized,
    #[serde(rename = "SETTLING")]
    Settling,
    #[serde(rename = "PARTIALLY_SETTLED")]
    PartiallySettled,
    #[serde(rename = "SETTLED")]
    Settled,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "CANCELLED")]
    Cancelled,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressApiSchema {
    ///Postal or ZIP code
    pub postal_code: Option<String>,
    ///Apartment, Unit or Building number
    pub address_line2: Option<String>,
    ///State, County or Province
    pub state: Option<String>,
    pub first_name: Option<String>,
    ///Name of the city, district, town or village
    pub city: String,
    ///Street name, Company name or PO Box
    pub address_line1: String,
    ///Two letter ISO country code
    pub country_code: CountryCodeEnum,
    pub last_name: Option<String>,
}
impl std::fmt::Display for AddressApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentCaptureApiRequest {
    /**Indicates whether the capture request is the final capture request.

After a final capture, no subsequent captures are allowed.*/
    pub final_: Option<bool>,
    /**The amount you would like to capture, in minor units. The currency used on authorization is assumed.

If no amount is specified it defaults to the full amount.*/
    pub amount: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentCaptureApiRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutCustomerDetailsApiSchema {
    /**Customer email address.
*/
    pub email_address: Option<String>,
    ///The customer's mobile number
    pub mobile_number: Option<String>,
    ///The customer's tax id number for tax exemptions
    pub tax_id: Option<String>,
    ///The customer's first name
    pub first_name: Option<String>,
    /**Customer billing address.
*/
    pub billing_address: Option<CoreApiApiCommonsSchemasAddessAddressApiSchema>,
    ///Customer shipping address
    pub shipping_address: Option<CoreApiApiCommonsSchemasAddessAddressApiSchema>,
    ///The customer's national identification number
    pub national_document_id: Option<String>,
    ///The customer's last name
    pub last_name: Option<String>,
}
impl std::fmt::Display for CheckoutCustomerDetailsApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ThreeDSecureFailedReasonCodeEnum {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "REJECTED_BY_ISSUER")]
    RejectedByIssuer,
    #[serde(rename = "CARD_AUTHENTICATION_FAILED")]
    CardAuthenticationFailed,
    #[serde(rename = "UNKNOWN_DEVICE")]
    UnknownDevice,
    #[serde(rename = "UNSUPPORTED_DEVICE")]
    UnsupportedDevice,
    #[serde(rename = "EXCEEDS_AUTHENTICATION_FREQUENCY_LIMIT")]
    ExceedsAuthenticationFrequencyLimit,
    #[serde(rename = "EXPIRED_CARD")]
    ExpiredCard,
    #[serde(rename = "INVALID_CARD_NUMBER")]
    InvalidCardNumber,
    #[serde(rename = "INVALID_TRANSACTION")]
    InvalidTransaction,
    #[serde(rename = "NO_CARD_RECORD")]
    NoCardRecord,
    #[serde(rename = "SECURITY_FAILURE")]
    SecurityFailure,
    #[serde(rename = "STOLEN_CARD")]
    StolenCard,
    #[serde(rename = "SUSPECTED_FRAUD")]
    SuspectedFraud,
    #[serde(rename = "TRANSACTION_NOT_PERMITTED_TO_CARDHOLDER")]
    TransactionNotPermittedToCardholder,
    #[serde(rename = "CARDHOLDER_NOT_ENROLLED_IN_SERVICE")]
    CardholderNotEnrolledInService,
    #[serde(rename = "TRANSACTION_TIMED_OUT_AT_THE_ACS")]
    TransactionTimedOutAtTheAcs,
    #[serde(rename = "LOW_CONFIDENCE")]
    LowConfidence,
    #[serde(rename = "MEDIUM_CONFIDENCE")]
    MediumConfidence,
    #[serde(rename = "HIGH_CONFIDENCE")]
    HighConfidence,
    #[serde(rename = "VERY_HIGH_CONFIDENCE")]
    VeryHighConfidence,
    #[serde(rename = "EXCEEDS_ACS_MAXIMUM_CHALLENGES")]
    ExceedsAcsMaximumChallenges,
    #[serde(rename = "NON_PAYMENT_NOT_SUPPORTED")]
    NonPaymentNotSupported,
    #[serde(rename = "THREE_RI_NOT_SUPPORTED")]
    ThreeRiNotSupported,
    #[serde(rename = "ACS_TECHNICAL_ISSUE")]
    AcsTechnicalIssue,
    #[serde(rename = "DECOUPLED_REQUIRED_BY_ACS")]
    DecoupledRequiredByAcs,
    #[serde(rename = "DECOUPLED_MAX_EXPIRY_EXCEEDED")]
    DecoupledMaxExpiryExceeded,
    #[serde(rename = "DECOUPLED_AUTHENTICATION_INSUFFICIENT_TIME")]
    DecoupledAuthenticationInsufficientTime,
    #[serde(rename = "AUTHENTICATION_ATTEMPTED_BUT_NOT_PERFORMED_BY_CARDHOLDER")]
    AuthenticationAttemptedButNotPerformedByCardholder,
    #[serde(rename = "ACS_TIMED_OUT")]
    AcsTimedOut,
    #[serde(rename = "INVALID_ACS_RESPONSE")]
    InvalidAcsResponse,
    #[serde(rename = "ACS_SYSTEM_ERROR_RESPONSE")]
    AcsSystemErrorResponse,
    #[serde(rename = "ERROR_GENERATING_CAVV")]
    ErrorGeneratingCavv,
    #[serde(rename = "PROTOCOL_VERSION_NOT_SUPPORTED")]
    ProtocolVersionNotSupported,
    #[serde(rename = "TRANSACTION_EXCLUDED_FROM_ATTEMPTS_PROCESSING")]
    TransactionExcludedFromAttemptsProcessing,
    #[serde(rename = "REQUESTED_PROGRAM_NOT_SUPPORTED")]
    RequestedProgramNotSupported,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KlarnaAddressApiSchema {
    pub city: Option<String>,
    ///An enumeration.
    pub country_code: Option<CountryCodeEnum>,
    pub title: Option<String>,
    pub phone_number: Option<String>,
    pub state: Option<String>,
    pub first_name: Option<String>,
    pub email: Option<String>,
    pub last_name: Option<String>,
    pub address_line2: Option<String>,
    pub postal_code: Option<String>,
    pub address_line1: Option<String>,
    pub address_line3: Option<String>,
}
impl std::fmt::Display for KlarnaAddressApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionDeclineReasonV2Enum {
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "INVALID_CARD_NUMBER")]
    InvalidCardNumber,
    #[serde(rename = "EXPIRED_CARD")]
    ExpiredCard,
    #[serde(rename = "LOST_OR_STOLEN_CARD")]
    LostOrStolenCard,
    #[serde(rename = "SUSPECTED_FRAUD")]
    SuspectedFraud,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "REFER_TO_CARD_ISSUER")]
    ReferToCardIssuer,
    #[serde(rename = "DO_NOT_HONOR")]
    DoNotHonor,
    #[serde(rename = "INSUFFICIENT_FUNDS")]
    InsufficientFunds,
    #[serde(rename = "WITHDRAWAL_LIMIT_EXCEEDED")]
    WithdrawalLimitExceeded,
    #[serde(rename = "ISSUER_TEMPORARILY_UNAVAILABLE")]
    IssuerTemporarilyUnavailable,
    #[serde(rename = "AUTHENTICATION_REQUIRED")]
    AuthenticationRequired,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaymentCardTokenApiSchemaPaymentMethodsApi {
    ///The type of card, e.g. Debit, Credit
    pub account_funding_type: Option<String>,
    pub cardholder_name: Option<String>,
    ///An ID for the transaction assigned by the card network. Used to correlate recurring payments.
    pub network_transaction_id: Option<String>,
    pub last4_digits: String,
    pub expiration_month: String,
    pub expiration_year: String,
    pub network: Option<String>,
}
impl std::fmt::Display for PaymentCardTokenApiSchemaPaymentMethodsApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderLineItemsApiSchema {
    ///The amount charged to the customer, in minor units.
    pub amount: serde_json::Value,
    ///A unique identifier for the line item.
    pub item_id: Option<String>,
    ///A name of the item.
    pub name: Option<String>,
    ///Details related to the product
    pub product_data: Option<OrderLineItemsProductDataApiSchema>,
    ///Any discount applicable to this item, in minor units. This discount is applied for the entire line item, and not per `quantity`.
    pub discount_amount: Option<serde_json::Value>,
    ///The number of the particular line item that is being ordered.
    pub quantity: Option<i64>,
    ///The tax code associated with this item, in minor units. This is required for Primer-initiated tax calculations.
    pub tax_code: Option<String>,
    ///The tax charged on this item, in minor units. This tax amount is applied for the entire line item, and not per `quantity`.
    pub tax_amount: Option<i64>,
    ///A description of the item.
    pub description: Option<String>,
    ///An identifier for the product type.
    pub product_type: Option<String>,
}
impl std::fmt::Display for OrderLineItemsApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AccountFundingTypeEnum {
    #[serde(rename = "CREDIT")]
    Credit,
    #[serde(rename = "DEBIT")]
    Debit,
    #[serde(rename = "PREPAID")]
    Prepaid,
    #[serde(rename = "CHARGE")]
    Charge,
    #[serde(rename = "DEFERRED_DEBIT")]
    DeferredDebit,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerDetailsApiSchema {
    /**Customer email address.

Note: It is recommended to include this field if a 3DS check will be performed
*/
    pub email_address: Option<String>,
    ///The customer's last name
    pub last_name: Option<String>,
    /**Customer billing address.

Note: It is recommended to include this field if a 3DS check will be performed
*/
    pub billing_address: Option<CoreApiApiCommonsSchemasAddessAddressApiSchema>,
    ///The customer's mobile number
    pub mobile_number: Option<String>,
    ///The customer's first name
    pub first_name: Option<String>,
    ///The customer's national identification number
    pub national_document_id: Option<String>,
    ///Customer shipping address
    pub shipping_address: Option<CoreApiApiCommonsSchemasAddessAddressApiSchema>,
    ///The customer's tax id number for tax exemptions
    pub tax_id: Option<String>,
}
impl std::fmt::Display for CustomerDetailsApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Currency(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Error400Response {
    pub error_object: ErrorObject,
}
impl std::fmt::Display for Error400Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GoCardlessMandateApiSchema {
    ///Unique identifier of a GoCardless mandate agreement
    pub gocardless_mandate_id: String,
}
impl std::fmt::Display for GoCardlessMandateApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentSummaryApiSchema {
    ///Your reference for the payment.
    pub order_id: String,
    ///The payment amount, in minor units. e.g. $7 would show as `700`.
    pub amount: i64,
    /**Additional data to be used throughout the payment lifecycle.

A dictionary of key-value pairs where the values can only be strings or
integers.

e.g. `{"productId": 1001, "merchantId": "a13bsd62s"}`
*/
    pub metadata: Option<serde_json::Value>,
    ///The payment processor used for this payment.
    pub processor: Option<String>,
    ///See the payment [status table](../docs#payment-status) for more information.
    pub status: String,
    /**The unique payment ID.

You can use this ID to retrieve the payment details, or perform downstream
operations.
*/
    pub id: String,
    ///The date and time at which the payment was created in UTC format.
    pub date: String,
    /**The 3-letter currency code in [ISO 4217 format](https://en.wikipedia.org/wiki/ISO_4217#Active_codes).
e.g. use `USD` for US dollars.
*/
    pub currency_code: String,
}
impl std::fmt::Display for PaymentSummaryApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TokenTypeEnum {
    #[serde(rename = "MULTI_USE")]
    MultiUse,
    #[serde(rename = "SINGLE_USE")]
    SingleUse,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRefundApiRequest {
    ///You can optionally specify a reason for the refund. This is for your own records.
    pub reason: Option<String>,
    /**The amount you would like to refund the customer, in minor units. e.g. for $7, use `700`.

Defaults to remaining non-refunded amount.*/
    pub amount: Option<serde_json::Value>,
    /**Optionally you can pass a specific order ID for the refund.

By default this will be set to the original `orderId` given on payment creation.*/
    pub order_id: Option<String>,
}
impl std::fmt::Display for PaymentRefundApiRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionOverviewApiSchema {
    ///Transaction status, please refer to the [Transaction Status Codes](#section/API-Usage-Guide/Payment-Status) table for more information
    pub processor_status: Option<String>,
    pub transaction_type: Option<String>,
    /**Processor's main account identifier.

* Adyen: Account code
* Braintree: Merchant ID
* Stripe: Account ID"
*/
    pub processor_merchant_id: String,
    ///Transaction amount in minor units
    pub amount: serde_json::Value,
    ///Date & time of the transaction (UTC)
    pub date: String,
    ///Processor's unique identifier for the transaction
    pub processor_transaction_id: Option<String>,
    /**If the transaction has a declined or failed status.

Only if the status is `DECLINED` or `FAILED`, otherwise `null`.
*/
    pub processor_status_reason: Option<StatusReasonApiSchema>,
    ///An identifier of a processor.
    pub processor_name: Option<String>,
    /**
The 3-letter currency code in [ISO 4217 format](https://en.wikipedia.org/wiki/ISO_4217#Active_codes).
e.g. use `USD` for US dollars.
*/
    pub currency_code: String,
}
impl std::fmt::Display for TransactionOverviewApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OrderLineItemsProductDataApiSchema {
    ///The product Manufacturer Part Number
    pub manufacturer_part_number: Option<String>,
    ///The product weight
    pub weight: Option<f64>,
    ///The product brand
    pub brand: Option<String>,
    ///The product color
    pub color: Option<String>,
    ///The product Global Trade Item Number (e.g. ISBN)
    pub global_trade_item_number: Option<String>,
    ///The product weight unit (e.g. kg, g)
    pub weight_unit: Option<String>,
    ///The product SKU
    pub sku: Option<String>,
}
impl std::fmt::Display for OrderLineItemsProductDataApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderFeesApiSchema {
    ///The fee amount charged to the customer, in minor  units. e.g. for $7, use `700`.
    pub amount: serde_json::Value,
    ///The type of fee.
    pub type_: Option<String>,
    ///A description of the fee, e.g. "Currency Conversion Fee".
    pub description: Option<String>,
}
impl std::fmt::Display for OrderFeesApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutPaymentMethodOptionsApiSchema {
    ///A description of the payment, as it would typically appear on a bank statement.
    pub descriptor: Option<String>,
    /**Payment types, primarily to be used for recurring payments.
Note: If you successfully vault a `SINGLE_USE` token on payment creation, then there's no need to set a value for this field and it will be flagged as `FIRST_PAYMENT`. Otherwise, see the table below for all possible values.

| paymentType | Use case |
| --- | --- |
| `FIRST_PAYMENT` | a customer-initiated payment which is the first in a series of recurring payments or subscription, or a card on file scenario.
| `ECOMMERCE` | a customer-initiated payment using stored payment details where the cardholder is present.
| `SUBSCRIPTION` | a merchant-initiated payment as part of a series of payments on a fixed schedule and a set amount.
| `UNSCHEDULED` | a merchant-initiated payment using stored payment details with no fixed schedule or amount.*/
    pub payment_type: Option<String>,
    /**Additional options for the payment methods.
*/
    pub options: Option<serde_json::Value>,
    ///Whether the payment method should be vaulted on a successful payment or not.
    pub vault_on_success: Option<bool>,
}
impl std::fmt::Display for CheckoutPaymentMethodOptionsApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ThreeDSecureAuthenticationApiSchema {
    pub challenge_issued: Option<bool>,
    ///An enumeration.
    pub response_code: String,
    pub reason_code: Option<serde_json::Value>,
    pub protocol_version: Option<String>,
    pub reason_text: Option<String>,
}
impl std::fmt::Display for ThreeDSecureAuthenticationApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentStatusTypeEnum {
    #[serde(rename = "APPLICATION_ERROR")]
    ApplicationError,
    #[serde(rename = "GATEWAY_REJECTED")]
    GatewayRejected,
    #[serde(rename = "ISSUER_DECLINED")]
    IssuerDeclined,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VaultPaymentMethodApiRequest {
    ///The ID representing the customer
    pub customer_id: String,
    ///Whether the payment method should be verified before vaulting or not
    pub verify: Option<bool>,
}
impl std::fmt::Display for VaultPaymentMethodApiRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RefundPaymentPaymentsIdRefundPostRequired {
    pub amount: serde_json::Value,
    pub reason: String,
    pub id: String,
    pub order_id: String,
}
impl std::fmt::Display for RefundPaymentPaymentsIdRefundPostRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientSessionUpdateApiRequest {
    /**More information associated with the order.

Each of the fields in this object must be updated in its entirety, i.e. provide the entire object to update it. Anything provided previously will be overwritten.
*/
    pub order: Option<OrderDetailsApiSchema>,
    ///A unique identifier for your customer.
    pub customer_id: Option<String>,
    /**The amount you would like to charge the customer, in minor units. e.g. for $7, use `700`.

Some currencies, such as Japanese Yen, do not have minor units. In this case you should use the value as it is, without any formatting. For example for ¥100, use `100`.

If the amount is provided on this level, it would override any amount calculated from the provided line items, shipping and other amounts.*/
    pub amount: Option<serde_json::Value>,
    /**Additional data to be used throughout the payment lifecycle.

Provide the entire object to update it. Anything provided previously will be overwritten.
*/
    pub metadata: Option<serde_json::Value>,
    /**More information associated with the customer.

Each of the fields in this object must be updated in its entirety, i.e. provide the entire object to update it. Anything provided previously will be overwritten.
*/
    pub customer: Option<CheckoutCustomerDetailsApiSchema>,
    /**
The 3-letter currency code in [ISO 4217 format](https://en.wikipedia.org/wiki/ISO_4217#Active_codes).
e.g. use `USD` for US dollars.
*/
    pub currency_code: Option<String>,
    ///Client token for use in the Primer-JS SDK obtained via `POST` /client-session API call.
    pub client_token: Option<String>,
    ///Enable certain options associated with the payment method. Provide the entire object to update it. Anything provided previously will be overwritten.
    pub payment_method: Option<CheckoutPaymentMethodOptionsApiSchema>,
    ///Your reference for the order.
    pub order_id: Option<String>,
}
impl std::fmt::Display for ClientSessionUpdateApiRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentResponsePaymentMethodOptionsApiSchema {
    ///The payment method token used to authorize the transaction.
    pub payment_method_token: Option<String>,
    ///Whether the payment method token represents a vaulted payment method and can be used for future payments.
    pub is_vaulted: Option<bool>,
    ///Unique analytics identifier corresponding to a payment method
    pub analytics_id: Option<String>,
    ///Payment method type
    pub payment_method_type: Option<String>,
    ///Payment method data
    pub payment_method_data: Option<serde_json::Value>,
    pub three_d_secure_authentication: Option<ThreeDSecureAuthenticationApiSchema>,
    ///The description of the payment, as it would typically appear on a bank statement.
    pub descriptor: Option<String>,
}
impl std::fmt::Display for PaymentResponsePaymentMethodOptionsApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KlarnaCustomerTokenApiSchema {
    pub klarna_customer_token: String,
    pub session_data: KlarnaSessionDetailsApiSchema,
}
impl std::fmt::Display for KlarnaCustomerTokenApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CardNetworkEnum(pub String);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ErrorObject {
    ///Returned in case of a badly formed request
    pub validation_errors: Option<Vec<serde_json::Value>>,
    ///An ID that you can quote when contacting the support team (support@primer.io).
    pub diagnostics_id: Option<String>,
    ///A human description of the error
    pub description: Option<String>,
    ///An error ID
    pub error_id: Option<String>,
}
impl std::fmt::Display for ErrorObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionTypeEnum {
    #[serde(rename = "SALE")]
    Sale,
    #[serde(rename = "REFUND")]
    Refund,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClientSideTokenClientSessionPatchRequired {
    pub client_token: String,
    pub payment_method: CheckoutPaymentMethodOptionsApiSchema,
    pub currency_code: String,
    pub metadata: serde_json::Value,
    pub amount: serde_json::Value,
    pub order_id: String,
    pub customer_id: String,
    pub order: OrderDetailsApiSchema,
    pub customer: CheckoutCustomerDetailsApiSchema,
}
impl std::fmt::Display for UpdateClientSideTokenClientSessionPatchRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct KlarnaTokenDetails {
    pub masked_number: Option<String>,
    pub brand: Option<String>,
    pub expiry_date: Option<String>,
    pub type_: String,
}
impl std::fmt::Display for KlarnaTokenDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MerchantPaymentMethodTokenListApiResponse {
    pub data: Option<Vec<MerchantPaymentMethodTokenApiResponse>>,
}
impl std::fmt::Display for MerchantPaymentMethodTokenListApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaymentSummaryProcessorApiSchema {
    ///The payment processor used for this payment.
    pub name: String,
    ///The merchant ID registered at the payment processor used for this payment.
    pub processor_merchant_id: Option<String>,
}
impl std::fmt::Display for PaymentSummaryProcessorApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KlarnaSessionDetailsApiSchema {
    pub purchase_country: String,
    pub recurring_description: Option<String>,
    pub token_details: Option<KlarnaTokenDetails>,
    pub purchase_currency: String,
    pub locale: String,
    pub billing_address: KlarnaAddressApiSchema,
    pub shipping_address: Option<KlarnaAddressApiSchema>,
    pub order_lines: Vec<serde_json::Value>,
}
impl std::fmt::Display for KlarnaSessionDetailsApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaymentCancelApiRequest {
    ///You can optionally specify a reason for the cancellation. This is for your own records.
    pub reason: Option<String>,
}
impl std::fmt::Display for PaymentCancelApiRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiedMerchantPaymentMethodTokenApiResponse {
    pub merchant_payment_method_token_api_response: MerchantPaymentMethodTokenApiResponse,
    ///Whether or not this payment method was verified
    pub is_verified: bool,
}
impl std::fmt::Display for VerifiedMerchantPaymentMethodTokenApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StatusReasonApiSchema {
    /**If the error is of type `ISSUER_DECLINED` this will be returned.

Declines of type `SOFT_DECLINE` may be retried,
whereas declines of type `HARD_DECLINE` should not be retried.
*/
    pub decline_type: Option<String>,
    ///If the error is of type `ISSUER_DECLINED`, this will be returned.
    pub code: Option<String>,
    ///In case of an error on the processor's part, we will return the message returned by the processor. This is usually a human readable error.
    pub message: Option<String>,
    ///Type of the status.
    pub type_: String,
}
impl std::fmt::Display for StatusReasonApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayPalOrderTokenApiSchema {
    ///Information about the PayPal customer
    pub external_payer_info: Option<PayPalExternalPayerInfoApiSchema>,
    pub paypal_status: Option<String>,
    pub paypal_order_id: String,
}
impl std::fmt::Display for PayPalOrderTokenApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientSessionWithTokenApiResponse {
    ///More information associated with the customer.
    pub customer: Option<CustomerDetailsApiSchema>,
    ///Warning messages to indicate missing information that are required for payment methods, checkout modules and other features; or when third-party services are unavailable.
    pub warnings: Option<ClientSessionWarningsApiResponse>,
    ///Expiration date & time of the client token (UTC with no timezoneinfo).
    pub client_token_expiration_date: Option<String>,
    /**Additional data to be used throughout the payment lifecycle.
*/
    pub metadata: Option<serde_json::Value>,
    /**The amount you would like to charge the customer, in minor units. e.g. for $7, use `700`.

Some currencies, such as Japanese Yen, do not have minor units. In this case you should use the value as it is, without any formatting. For example for ¥100, use `100`.

If the amount is provided on this level, it would override any amount calculated from the provided line items, shipping and other amounts.*/
    pub amount: Option<serde_json::Value>,
    ///More information associated with the order.
    pub order: Option<OrderDetailsApiSchema>,
    /**e.g. use `USD` for US dollars.
*/
    pub currency_code: Option<String>,
    ///Enable certain options associated with the payment methods.
    pub payment_method: Option<CheckoutPaymentMethodOptionsApiSchema>,
    ///Client token used to initialize the SDK on all platforms.
    pub client_token: Option<String>,
    ///Your reference for the payment.
    pub order_id: Option<String>,
    ///A unique identifier for your customer.
    pub customer_id: Option<String>,
}
impl std::fmt::Display for ClientSessionWithTokenApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutPaymentMethodOptionCardNetworkApiSchema {
    /**Options for the card network `CARD_NETWORK_TYPE`.

[The list of available card networks and their `CARD_NETWORK_TYPE` can be found here.](https://www.notion.so/primerio/Payment-Method-Types-2b971a8c54c3452cae0b2fffe9167d72)
*/
    pub card_network_type: Option<serde_json::Value>,
}
impl std::fmt::Display for CheckoutPaymentMethodOptionCardNetworkApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CardRegionRestrictionEnum {
    #[serde(rename = "DOMESTIC_USE_ONLY")]
    DomesticUseOnly,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BinDataApiSchema {
    /**[The list of available card networks and their `CARD_NETWORK_TYPE` can be found here.](https://www.notion.so/primerio/Payment-Method-Types-2b971a8c54c3452cae0b2fffe9167d72)
*/
    pub network: String,
    pub product_code: String,
    pub issuer_name: Option<String>,
    ///An enumeration.
    pub issuer_country_code: Option<CountryCodeEnum>,
    ///An enumeration.
    pub prepaid_reloadable_indicator: String,
    pub product_name: String,
    ///Enumerates all supported currencies
    pub issuer_currency_code: Option<Currency>,
    ///An enumeration.
    pub product_usage_type: String,
    ///An enumeration.
    pub account_funding_type: String,
    ///An enumeration.
    pub regional_restriction: String,
    ///An enumeration.
    pub account_number_type: String,
}
impl std::fmt::Display for BinDataApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaymentRequestPaymentMethodOptionsApiSchema {
    ///A description of the payment, as it would typically appear on a bank statement.
    pub descriptor: Option<String>,
    /**Payment types, primarily to be used for recurring payments.
Note: If you successfully vault a `SINGLE_USE` token on payment creation, then there's no need to set a value for this field and it will be flagged as `FIRST_PAYMENT`. Otherwise, see the table below for all possible values.

| paymentType | Use case |
| --- | --- |
| `FIRST_PAYMENT` | a customer-initiated payment which is the first in a series of recurring payments or subscription, or a card on file scenario.
| `ECOMMERCE` | a customer-initiated payment using stored payment details where the cardholder is present.
| `SUBSCRIPTION` | a merchant-initiated payment as part of a series of payments on a fixed schedule and a set amount.
| `UNSCHEDULED` | a merchant-initiated payment using stored payment details with no fixed schedule or amount.*/
    pub payment_type: Option<String>,
    ///Whether the payment method should be vaulted on a successful payment or not.
    pub vault_on_success: Option<bool>,
}
impl std::fmt::Display for PaymentRequestPaymentMethodOptionsApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderShippingApiSchema {
    ///The shipping amount charged to the customer, in minor units. e.g. for $7, use `700`.
    pub amount: Option<serde_json::Value>,
}
impl std::fmt::Display for OrderShippingApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentCardTokenApiSchema {
    pub bin_data: Option<BinDataApiSchema>,
    pub last4_digits: String,
    pub is_network_tokenized: Option<bool>,
    pub first6_digits: Option<String>,
    pub expiration_year: String,
    pub cardholder_name: Option<String>,
    pub network: Option<String>,
    pub expiration_month: String,
}
impl std::fmt::Display for PaymentCardTokenApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaymentListApiResponse {
    pub next_cursor: Option<String>,
    pub data: Option<Vec<PaymentSummaryApiSchema>>,
    pub prev_cursor: Option<String>,
}
impl std::fmt::Display for PaymentListApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaymentResumeApiRequest {
    ///A token containing any information that is sent back from the checkout to complete a blocked payment flow.
    pub resume_token: String,
}
impl std::fmt::Display for PaymentResumeApiRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CheckoutPaymentMethodOptionSurchargeApiSchema {
    ///The surcharge amount, in minor units. Surcharge amount must be used in conjunction with line item amounts, if a top level amount is passed then surcharge will not be calculated.
    pub amount: Option<i64>,
}
impl std::fmt::Display for CheckoutPaymentMethodOptionSurchargeApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientSessionApiRequest {
    ///More information associated with the customer.
    pub customer: Option<CheckoutCustomerDetailsApiSchema>,
    /**The 3-letter currency code in [ISO 4217 format](https://en.wikipedia.org/wiki/ISO_4217#Active_codes).
e.g. use `USD` for US dollars.
*/
    pub currency_code: Option<String>,
    ///Enable certain options associated with the payment method.
    pub payment_method: Option<CheckoutPaymentMethodOptionsApiSchema>,
    /**The amount you would like to charge the customer, in minor units. e.g. for $7, use `700`.

Some currencies, such as Japanese Yen, do not have minor units. In this case you should use the value as it is, without any formatting. For example for ¥100, use `100`.

If the amount is provided on this level, it would override any amount calculated from the provided line items, shipping and other amounts.*/
    pub amount: Option<serde_json::Value>,
    /**Additional data to be used throughout the payment lifecycle.

A dictionary of key-value pairs where the values can only be strings or
integers.

e.g. `{"productId": 1001, "merchantId": "a13bsd62s"}`
*/
    pub metadata: Option<serde_json::Value>,
    ///Your reference for the payment.
    pub order_id: Option<String>,
    ///More information associated with the order.
    pub order: Option<OrderDetailsApiSchema>,
    /**A unique identifier for your customer.
Create a client session token with a `customerId` to enable the client-side SDK to retrieve and manage your customer's saved payment methods. A client session token also enables saving payment methods against this customer id.*/
    pub customer_id: Option<String>,
}
impl std::fmt::Display for ClientSessionApiRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodTypeEnum(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum CardProductTypeEnum {
    #[serde(rename = "CONSUMER")]
    Consumer,
    #[serde(rename = "BUSINESS")]
    Business,
    #[serde(rename = "GOVERNMENT")]
    Government,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Error422Response {
    pub error_object: ErrorObject,
}
impl std::fmt::Display for Error422Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientSessionApiResponse {
    ///A unique identifier for your customer.
    pub customer_id: Option<String>,
    ///More information associated with the order.
    pub order: Option<OrderDetailsApiSchema>,
    /**The 3-letter currency code in [ISO 4217 format](https://en.wikipedia.org/wiki/ISO_4217#Active_codes).
e.g. use `USD` for US dollars.
*/
    pub currency_code: Option<String>,
    /**The amount you are going to charge the customer, in minor units. This amount is calculated from the line items, shipping and other amounts provided in the `order`.
If a top-level amount is provided, it would override any calculated amount.*/
    pub amount: Option<serde_json::Value>,
    ///Your reference for the payment.
    pub order_id: Option<String>,
    /**Additional data to be used throughout the payment lifecycle.
*/
    pub metadata: Option<serde_json::Value>,
    ///More information associated with the customer.
    pub customer: Option<CustomerDetailsApiSchema>,
    ///Enable certain options associated with the payment methods.
    pub payment_method: Option<CheckoutPaymentMethodOptionsApiSchema>,
}
impl std::fmt::Display for ClientSessionApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentCreationApiRequest {
    ///Your reference for the payment.
    pub order_id: Option<String>,
    ///More information associated with the order.
    pub order: Option<OrderDetailsApiSchema>,
    /**The payment method token used to authorize the payment.
*/
    pub payment_method_token: String,
    /**Additional data to be used throughout the payment lifecycle.

A dictionary of key-value pairs where the values can only be strings or
integers.

e.g. `{"productId": 1001, "merchantId": "a13bsd62s"}`
*/
    pub metadata: Option<serde_json::Value>,
    /**The amount you would like to charge the customer, in minor units. e.g. for $7, use `700`.

Some currencies, such as Japanese Yen, do not have minor units. In this case you should use the value as it is, without any formatting. For example for ¥100, use `100`.*/
    pub amount: Option<serde_json::Value>,
    /**The 3-letter currency code in [ISO 4217 format](https://en.wikipedia.org/wiki/ISO_4217#Active_codes).
e.g. use `USD` for US dollars.
*/
    pub currency_code: Option<String>,
    /**A unique identifier for your customer.
This attribute is required if `paymentMethod.vaultOnSuccess` is set to `True`.*/
    pub customer_id: Option<String>,
    ///Enable certain options associated with the payment method.
    pub payment_method: Option<PaymentRequestPaymentMethodOptionsApiSchema>,
    /**More information associated with the customer.
*/
    pub customer: Option<CustomerDetailsApiSchema>,
}
impl std::fmt::Display for PaymentCreationApiRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IdealPayNlTokenApiSchema {
    pub payment_method_config_id: String,
}
impl std::fmt::Display for IdealPayNlTokenApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ProductTypeEnum {
    #[serde(rename = "PHYSICAL")]
    Physical,
    #[serde(rename = "DIGITAL")]
    Digital,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaymentResponseProcessorApiSchema {
    ///The merchant ID registered at the payment processor used for this payment.
    pub processor_merchant_id: Option<String>,
    /**If no capture was performed, this value will be set to `0`.

If one or more partial captures were performed, this value will be a sum
of all partial capture amounts.
*/
    pub amount_captured: Option<i64>,
    ///The payment processor used for this payment.
    pub name: Option<String>,
    /**If no refund was performed, this value will be set to `0`.

If one or more partial refunds were performed, this value will be a sum
of all partial refund amounts.
*/
    pub amount_refunded: Option<i64>,
}
impl std::fmt::Display for PaymentResponseProcessorApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DeclineTypeEnum {
    #[serde(rename = "SOFT_DECLINE")]
    SoftDecline,
    #[serde(rename = "HARD_DECLINE")]
    HardDecline,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VerifiedMerchantPaymentMethodTokenListApiResponse {
    pub data: Option<Vec<VerifiedMerchantPaymentMethodTokenApiResponse>>,
}
impl std::fmt::Display for VerifiedMerchantPaymentMethodTokenListApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BinDataOptionalApiSchema {
    /**[The list of available card networks and their `CARD_NETWORK_TYPE` can be found here.](https://www.notion.so/primerio/Payment-Method-Types-2b971a8c54c3452cae0b2fffe9167d72)
*/
    pub network: Option<String>,
}
impl std::fmt::Display for BinDataOptionalApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PayPalExternalPayerInfoApiSchema {
    pub last_name: Option<String>,
    pub external_payer_id: Option<String>,
    pub first_name: Option<String>,
    pub email: Option<String>,
}
impl std::fmt::Display for PayPalExternalPayerInfoApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDetailsApiSchema {
    ///The details of the line items of the order.
    pub line_items: Option<Vec<OrderLineItemsApiSchema>>,
    ///The country in which the order is created
    pub country_code: Option<CountryCodeEnum>,
    ///The details of fees charged.
    pub fees: Option<Vec<OrderFeesApiSchema>>,
    ///The details of shipping charged.
    pub shipping: Option<OrderShippingApiSchema>,
}
impl std::fmt::Display for OrderDetailsApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ThreeDSecureAuthResponseCodeEnum {
    #[serde(rename = "NOT_PERFORMED")]
    NotPerformed,
    #[serde(rename = "SKIPPED")]
    Skipped,
    #[serde(rename = "AUTH_SUCCESS")]
    AuthSuccess,
    #[serde(rename = "AUTH_FAILED")]
    AuthFailed,
    #[serde(rename = "CHALLENGE")]
    Challenge,
    #[serde(rename = "METHOD")]
    Method,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CardAccountNumberTypeEnum {
    #[serde(rename = "PRIMARY_ACCOUNT_NUMBER")]
    PrimaryAccountNumber,
    #[serde(rename = "NETWORK_TOKEN")]
    NetworkToken,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApayaCustomerTokenApiSchema {
    pub mnc: Option<i64>,
    pub mx: String,
    pub mcc: Option<i64>,
}
impl std::fmt::Display for ApayaCustomerTokenApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum RecurringTransactionTypeEnum {
    #[serde(rename = "FIRST_PAYMENT")]
    FirstPayment,
    #[serde(rename = "ECOMMERCE")]
    Ecommerce,
    #[serde(rename = "SUBSCRIPTION")]
    Subscription,
    #[serde(rename = "UNSCHEDULED")]
    Unscheduled,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentApiResponse {
    ///More information associated with the order.
    pub order: Option<OrderDetailsApiSchema>,
    ///More information associated with the payment processor, including the processor name.
    pub processor: Option<PaymentResponseProcessorApiSchema>,
    ///The date and time at which the payment was created in UTC format.
    pub date: Option<String>,
    ///The payment method options provided in the request, as well as the token used to process the payment.
    pub payment_method: Option<PaymentResponsePaymentMethodOptionsApiSchema>,
    ///The amount you charged the customer, in minor units.
    pub amount: Option<serde_json::Value>,
    /**Check this field for more information regarding the payment's status. This is especially useful when the status is `DECLINED` or `FAILED`.
*/
    pub status_reason: Option<StatusReasonApiSchema>,
    /**A list summarizing the transactions that occurred while processing the payment.

Note: a refund is a separate transaction and so will appear in this `transactions` list if a refund was performed.*/
    pub transactions: Option<Vec<TransactionOverviewApiSchema>>,
    /**The unique payment ID.

You can use this ID to retrieve the payment details, or perform downstream
operations.
*/
    pub id: Option<String>,
    ///See the payment [status table](../docs#payment-status) for more information.
    pub status: Option<String>,
    ///Required action to perform in order to resume the payment workflow. This can be requiring a 3DS check from the customer for instance.
    pub required_action: Option<PaymentRequiredActionApiSchema>,
    /**Additional data to be used throughout the payment lifecycle.
*/
    pub metadata: Option<serde_json::Value>,
    ///Your reference for the payment.
    pub order_id: Option<String>,
    /**The 3-letter currency code in [ISO 4217 format](https://en.wikipedia.org/wiki/ISO_4217#Active_codes).
e.g. use `USD` for US dollars.
*/
    pub currency_code: Option<String>,
    ///The unique identifier for your customer.
    pub customer_id: Option<String>,
    ///More information associated with the customer.
    pub customer: Option<CustomerDetailsApiSchema>,
}
impl std::fmt::Display for PaymentApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutPaymentMethodCardOptionApiSchema {
    pub networks: Option<CheckoutPaymentMethodOptionCardNetworkApiSchema>,
}
impl std::fmt::Display for CheckoutPaymentMethodCardOptionApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayPalBillingAgreementApiSchema {
    ///The PayPal customer's shipping address
    pub shipping_address: Option<AddressApiSchema>,
    pub paypal_billing_agreement_id: String,
    pub paypal_status: Option<String>,
    ///Information about the PayPal customer
    pub external_payer_info: Option<PayPalExternalPayerInfoApiSchema>,
}
impl std::fmt::Display for PayPalBillingAgreementApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClientSideTokenClientSessionPostRequired {
    pub order: OrderDetailsApiSchema,
    pub metadata: serde_json::Value,
    pub customer: CheckoutCustomerDetailsApiSchema,
    pub customer_id: String,
    pub payment_method: CheckoutPaymentMethodOptionsApiSchema,
    pub currency_code: String,
    pub order_id: String,
    pub amount: serde_json::Value,
}
impl std::fmt::Display for CreateClientSideTokenClientSessionPostRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaymentRequiredActionApiSchema {
    ///Action name
    pub name: String,
    ///If the action requires customer data, instantiate the checkout SDK with this client session token to resume the session.
    pub client_token: Option<String>,
    ///Human description of the required action to perform.
    pub description: String,
}
impl std::fmt::Display for PaymentRequiredActionApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ThreeDSecureSkippedReasonCodeEnum {
    #[serde(rename = "GATEWAY_UNAVAILABLE")]
    GatewayUnavailable,
    #[serde(rename = "DISABLED_BY_MERCHANT")]
    DisabledByMerchant,
    #[serde(rename = "NOT_SUPPORTED_BY_ISSUER")]
    NotSupportedByIssuer,
    #[serde(rename = "FAILED_TO_NEGOTIATE")]
    FailedToNegotiate,
    #[serde(rename = "UNKNOWN_ACS_RESPONSE")]
    UnknownAcsResponse,
    #[serde(rename = "3DS_SERVER_ERROR")]
    ThreeDSecureSkippedReasonCodeEnum3DsServerError,
    #[serde(rename = "ACQUIRER_NOT_CONFIGURED")]
    AcquirerNotConfigured,
    #[serde(rename = "ACQUIRER_NOT_PARTICIPATING")]
    AcquirerNotParticipating,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClientSessionWarningsApiResponse {
    ///The type of the connection involved
    pub type_: Option<String>,
    ///A unique code describing the particular issue
    pub code: Option<String>,
    ///More information as to the reason for the warning
    pub message: Option<String>,
}
impl std::fmt::Display for ClientSessionWarningsApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MerchantPaymentMethodTokenApiResponse {
    ///The vaulted payment method token.
    pub token: Option<String>,
    ///Date & time when this object was revoked. (UTC)
    pub deleted_at: Option<String>,
    ///* `MULTI_USE` a vaulted token that can be re-used with subsequent payments
    pub token_type: Option<String>,
    ///Payment method data
    pub payment_method_data: Option<serde_json::Value>,
    ///The ID representing the customer
    pub customer_id: Option<String>,
    ///Whether or not this object has been revoked.
    pub deleted: Option<bool>,
    ///A friendly description given by the user
    pub description: Option<String>,
    ///Whether or not this payment method is the default
    pub default: Option<bool>,
    ///Payment method type
    pub payment_method_type: Option<String>,
    ///Unique analytics identifier corresponding to a payment method
    pub analytics_id: Option<String>,
    ///Creation date & time of the object (UTC)
    pub created_at: Option<String>,
}
impl std::fmt::Display for MerchantPaymentMethodTokenApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BlockingPaymentActionTypeEnum {
    #[serde(rename = "3DS_AUTHENTICATION")]
    BlockingPaymentActionTypeEnum3DsAuthentication,
    #[serde(rename = "USE_PRIMER_SDK")]
    UsePrimerSdk,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PrepaidReloadableEnum {
    #[serde(rename = "RELOADABLE")]
    Reloadable,
    #[serde(rename = "NON_RELOADABLE")]
    NonReloadable,
    #[serde(rename = "NOT_APPLICABLE")]
    NotApplicable,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutPaymentMethodOptionApiSchema {
    ///Surcharge information
    pub surcharge: Option<CheckoutPaymentMethodOptionSurchargeApiSchema>,
}
impl std::fmt::Display for CheckoutPaymentMethodOptionApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CoreApiApiCommonsSchemasAddessAddressApiSchema {
    pub postal_code: Option<String>,
    pub address_line1: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub state: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    ///An enumeration.
    pub country_code: Option<CountryCodeEnum>,
}
impl std::fmt::Display for CoreApiApiCommonsSchemasAddessAddressApiSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CountryCodeEnum(pub serde_json::Value);
