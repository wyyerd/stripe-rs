use crate::config::{Client, Response};
use crate::ids::{CustomerId, BillingPortalConfigurationId};
use crate::resources::{BillingPortalSession};
use serde_derive::{Deserialize, Serialize};

/// The parameters for `BillingPortalSession::create`
///
/// For more details see [https://stripe.com/docs/api/customer_portal/sessions/create](https://stripe.com/docs/api/customer_portal/sessions/create).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateBillingPortalSession {
    /// The ID of an existing customer.
    pub customer: CustomerId,

    /// The default URL to redirect customers to when they click on the portal’s link to return to your website.
    /// 
    /// Required if the configuration's default return url is not set
    pub return_url: Option<String>,

    /// The ID of an existing configuration to use for this session, describing its functionality and features. 
    ///
    /// If not specified, the session uses the default configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<BillingPortalConfigurationId>,
}

impl BillingPortalSession {
    /// Attach a payment method to a customer
    ///
    /// For more details see [https://stripe.com/docs/api/customer_portal/sessions/create](https://stripe.com/docs/api/customer_portal/sessions/create).
    pub fn create(client: &Client, params: CreateBillingPortalSession) -> Response<BillingPortalSession> {
        client.post_form("/billing_portal/sessions", params)
    }
}
