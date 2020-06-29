use crate::config::{Client, Response};
use crate::ids::{SubscriptionItemId, UsageRecordId};
use crate::params::Timestamp;
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "UsageRecord".
///
/// For more details see [https://stripe.com/docs/api/usage_records/](https://stripe.com/docs/api/usage_records/).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UsageRecord {
    /// Unique identifier for the object.
    pub id: UsageRecordId,

    /// The usage quantity for the specified date.
    pub quantity: u64,

    /// The ID of the subscription item this usage record contains data for.
    pub subscription_item: SubscriptionItemId,

    /// The timestamp when this usage occurred.
    pub timestamp: Timestamp,
}

/// Creates a usage record for a specified subscription item and date, and fills it with a quantity.
///
/// For more details see [https://stripe.com/docs/api/usage_records/create](https://stripe.com/docs/api/usage_records/create).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateUsageRecord {
    /// The usage quantity for the specified date.
    pub quantity: u64,

    /// The timestamp when this usage occurred.
    pub timestamp: Timestamp,

    /// Valid values are `Increment` (default) or `Set`.
    /// When using `Increment` the specified quantity will be added to the usage at the specified timestamp.
    /// The `Set` action will overwrite the usage quantity at that timestamp.
    /// If the subscription has billing thresholds, `Increment` is the only allowed value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<UsageRecordAction>,
}

impl CreateUsageRecord {
    pub fn new(quantity: u64, timestamp: Timestamp) {
        CreateUsageRecord { quantity, timestamp, action: None }
    }
}

/// An enum specifying possible values for `CreateUsageRecord`'s `action` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UsageRecordAction {
    /// Deafult. The specified quantity will be added to the usage.
    Increment,

    /// Overwrites the usage quantity.
    Set,
}

impl UsageRecord {
    /// Creates a usage record for a specified subscription item and date, and fills it with a quantity.
    ///
    /// For more details see [https://stripe.com/docs/api/usage_records/create](https://stripe.com/docs/api/usage_records/create).
    pub fn create(
        client: &Client,
        id: SubscriptionItemId,
        params: CreateUsageRecord,
    ) -> Response<Self> {
        client.post_form(&format!("/subscription_items/{}/usage_records", id), &params)
    }
}
