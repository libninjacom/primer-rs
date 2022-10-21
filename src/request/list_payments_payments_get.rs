use serde_json::json;
use crate::model::*;
use crate::PrimerClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListPaymentsPaymentsGetRequest<'a> {
    pub(crate) client: &'a PrimerClient,
    pub status: Option<Vec<String>>,
    pub payment_method_type: Option<Vec<String>>,
    pub processor: Option<Vec<String>>,
    pub currency_code: Option<Vec<String>>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub order_id: Option<String>,
    pub min_amount: Option<i64>,
    pub max_amount: Option<i64>,
    pub customer_id: Option<Vec<String>>,
    pub merchant_id: Option<Vec<String>>,
    pub customer_email_address: Option<Vec<String>>,
    pub last4_digits: Option<Vec<String>>,
    pub paypal_email: Option<Vec<String>>,
    pub klarna_email: Option<Vec<String>>,
    pub limit: Option<serde_json::Value>,
    pub cursor: Option<String>,
}
impl<'a> ListPaymentsPaymentsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentListApiResponse> {
        let mut r = self.client.client.get("/payments");
        if let Some(ref unwrapped) = self.status {
            for item in unwrapped {
                r = r.push_query("status[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.payment_method_type {
            for item in unwrapped {
                r = r.push_query("payment_method_type[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.processor {
            for item in unwrapped {
                r = r.push_query("processor[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.currency_code {
            for item in unwrapped {
                r = r.push_query("currency_code[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.from_date {
            r = r.push_query("from_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.to_date {
            r = r.push_query("to_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.order_id {
            r = r.push_query("order_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.min_amount {
            r = r.push_query("min_amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.max_amount {
            r = r.push_query("max_amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer_id {
            for item in unwrapped {
                r = r.push_query("customer_id[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.merchant_id {
            for item in unwrapped {
                r = r.push_query("merchant_id[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.customer_email_address {
            for item in unwrapped {
                r = r.push_query("customer_email_address[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.last4_digits {
            for item in unwrapped {
                r = r.push_query("last_4_digits[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.paypal_email {
            for item in unwrapped {
                r = r.push_query("paypal_email[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.klarna_email {
            for item in unwrapped {
                r = r.push_query("klarna_email[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_query("cursor", &unwrapped.to_string());
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
    pub fn status(mut self, status: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.status = Some(status.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn payment_method_type(
        mut self,
        payment_method_type: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .payment_method_type = Some(
            payment_method_type.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn processor(
        mut self,
        processor: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .processor = Some(
            processor.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn currency_code(
        mut self,
        currency_code: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .currency_code = Some(
            currency_code.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn from_date(mut self, from_date: &str) -> Self {
        self.from_date = Some(from_date.to_owned());
        self
    }
    pub fn to_date(mut self, to_date: &str) -> Self {
        self.to_date = Some(to_date.to_owned());
        self
    }
    pub fn order_id(mut self, order_id: &str) -> Self {
        self.order_id = Some(order_id.to_owned());
        self
    }
    pub fn min_amount(mut self, min_amount: i64) -> Self {
        self.min_amount = Some(min_amount);
        self
    }
    pub fn max_amount(mut self, max_amount: i64) -> Self {
        self.max_amount = Some(max_amount);
        self
    }
    pub fn customer_id(
        mut self,
        customer_id: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .customer_id = Some(
            customer_id.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn merchant_id(
        mut self,
        merchant_id: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .merchant_id = Some(
            merchant_id.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn customer_email_address(
        mut self,
        customer_email_address: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .customer_email_address = Some(
            customer_email_address.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn last4_digits(
        mut self,
        last4_digits: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .last4_digits = Some(
            last4_digits.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn paypal_email(
        mut self,
        paypal_email: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .paypal_email = Some(
            paypal_email.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn klarna_email(
        mut self,
        klarna_email: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .klarna_email = Some(
            klarna_email.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn limit(mut self, limit: serde_json::Value) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
