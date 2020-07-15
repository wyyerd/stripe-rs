use crate::config::{Client, Response};
use crate::ids::SubscriptionScheduleId;
use crate::resources::SubscriptionSchedule;
use serde_derive::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelSubscriptionSchedule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_now: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
}

impl CancelSubscriptionSchedule {
    pub fn new() -> CancelSubscriptionSchedule {
        CancelSubscriptionSchedule::default()
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ReleaseSubscriptionSchedule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_cancel_date: Option<bool>,
}

impl ReleaseSubscriptionSchedule {
    pub fn new() -> ReleaseSubscriptionSchedule {
        ReleaseSubscriptionSchedule::default()
    }
}

impl SubscriptionSchedule {
    /// Cancels a subscription schedule.
    ///
    /// For more details see https://stripe.com/docs/api/subscription_schedules/cancel
    pub fn cancel(
        client: &Client,
        subscription_schedule_id: &SubscriptionScheduleId,
        params: CancelSubscriptionSchedule,
    ) -> Response<SubscriptionSchedule> {
        client.post_form(
            &format!("/subscription_schedules/{}/cancel", subscription_schedule_id),
            params,
        )
    }

    /// Releases a subscription schedule.
    ///
    /// For more details see https://stripe.com/docs/api/subscription_schedules/release
    pub fn release(
        client: &Client,
        subscription_schedule_id: &SubscriptionScheduleId,
        params: ReleaseSubscriptionSchedule,
    ) -> Response<SubscriptionSchedule> {
        client.post_form(
            &format!("/subscription_schedules/{}/release", subscription_schedule_id),
            params,
        )
    }
}
