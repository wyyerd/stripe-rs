use crate::config::{Client, Response};
use crate::ids::{InvoiceId, SubscriptionItemId, UsageRecordId, UsageRecordSummaryId};
use crate::params::{Expand, List, Object, Timestamp};
use crate::OpenPeriod;
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
    pub fn new(quantity: u64, timestamp: Timestamp) -> Self {
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

/// The resource representing a Stripe "UsageRecordSummary".
///
/// For more details see [https://stripe.com/docs/api/usage_records/subscription_item_summary_list](https://stripe.com/docs/api/usage_records/subscription_item_summary_list).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UsageRecordSummary {
    /// Unique identifier for the object.
    pub id: UsageRecordSummaryId,

    /// The invoice in which this usage period has been billed for.
    pub invoice: Option<InvoiceId>,

    // The usage period.
    pub period: OpenPeriod,

    /// The ID of the subscription item this summary is describing.
    pub subscription_item: SubscriptionItemId,

    /// The total usage within this usage period.
    pub total_usage: u64,
}

impl UsageRecordSummary {
    /// For the specified subscription item, returns a list of summary objects. Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).
    ///
    /// The list is sorted in reverse-chronological order (newest first). The first list item represents the most current usage period that hasn’t ended yet. Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.
    pub fn list(
        client: &Client,
        id: &SubscriptionItemId,
        params: ListUsageRecordSummaries<'_>,
    ) -> Response<List<UsageRecordSummary>> {
        // This is a bit of a strange API since params.subscription_item needs to go into the URL,
        // but the rest of the parameters (except subscription_item) need to be passed via query params.
        let url = format!("/subscription_items/{}/usage_record_summaries", &id);
        client.get_query(&url, &params)
    }
}

impl Object for UsageRecordSummary {
    type Id = UsageRecordSummaryId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "usage_record_summary"
    }
}

/// For the specified subscription item, returns a list of summary objects. Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).
///
/// The list is sorted in reverse-chronological order (newest first). The first list item represents the most current usage period that hasn’t ended yet. Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.
/// For more details see [https://stripe.com/docs/api/usage_records/subscription_item_summary_list](https://stripe.com/docs/api/usage_records/subscription_item_summary_list).
#[derive(Clone, Debug, Serialize)]
pub struct ListUsageRecordSummaries<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<UsageRecordSummaryId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<UsageRecordSummaryId>,
}

impl<'a> ListUsageRecordSummaries<'a> {
    pub fn new() -> Self {
        ListUsageRecordSummaries {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
