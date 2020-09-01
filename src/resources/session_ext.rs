use crate::config::{Client, Response};
use crate::ids::{CustomerId, SessionId};
use crate::params::Timestamp;
use serde_derive::{Deserialize, Serialize};

/// Creates a session of the customer portal.
///
/// For more details see [https://stripe.com/docs/api/customer_portal/create](https://stripe.com/docs/api/customer_portal/create).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSession {
    /// The ID of an existing customer.
    pub customer: CustomerId,

    /// The URL to which Stripe should send customers when they click on the link to return to your website. This field is required if a default return URL has not been configured for the portal.
    pub return_url: Option<String>,
}

/// The Session object.
///
/// A session describes the instantiation of the customer portal for a particular customer. By visiting the session's URL, the customer can manage their subscriptions and billing details. For security reasons, sessions are short-lived and will expire if the customer does not visit the URL. Create sessions on-demand when customers intend to manage their subscriptions and billing details.
///
/// For more details see [https://stripe.com/docs/api/customer_portal/object](https://stripe.com/docs/api/customer_portal/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    /// Unique identifier for the object.
    pub id: SessionId,

    /// String representing the object’s type. Objects of the same type share the same value.
    pub object: String,

    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The ID of the customer for this session.
    pub customer: CustomerId,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The URL to which Stripe should send customers when they click on the link to return to your website.
    pub return_url: String,

    /// The short-lived URL of the session giving customers access to the customer portal.
    pub url: String,
}

impl CreateSession {
    pub fn new(customer: CustomerId) -> Self {
        CreateSession { customer, return_url: None }
    }
}

impl Session {
    /// Creates a session of the customer portal.
    ///
    /// For more details see [https://stripe.com/docs/api/customer_portal/create](https://stripe.com/docs/api/customer_portal/create).
    pub fn create(client: &Client, params: CreateSession) -> Response<Self> {
        client.post_form(&format!("/billing_portal/sessions"), &params)
    }
}
