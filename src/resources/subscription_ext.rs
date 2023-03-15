use crate::config::{Client, Response};
use crate::ids::SubscriptionId;
use crate::resources::{CreateSubscriptionItems, Subscription};
use serde_derive::Serialize;

impl Subscription {
    /// Cancels a subscription.
    ///
    /// For more details see https://stripe.com/docs/api#cancel_subscription.
    pub fn cancel(
        client: &Client,
        subscription_id: &SubscriptionId,
    ) -> Response<Subscription> {
        client.delete(&format!("/subscriptions/{}", subscription_id))
    }
}

impl CreateSubscriptionItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: Default::default(),
            metadata: Default::default(),
            plan: Default::default(),
            price: Default::default(),
            price_data: Default::default(),
            quantity: Default::default(),
            tax_rates: Default::default(),
        }
    }
}
