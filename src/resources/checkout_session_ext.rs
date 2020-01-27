use crate::config::{Client, Response};
use crate::ids::CustomerId;
use crate::resources::{
    CheckoutSession, CheckoutSessionLocale, CheckoutSessionMode, CheckoutSessionSubmitType,
    Currency,
};
use serde_derive::{Deserialize, Serialize};

/// The parameters for `CheckoutSession::create`
///
/// For more details see [https://stripe.com/docs/api/payment_methods/attach](https://stripe.com/docs/api/payment_methods/attach).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSession<'a> {
    /// The URL the customer will be directed to if they decide to cancel payment and return to your website.
    pub cancel_url: &'a str,

    /// A list of the types of payment methods (e.g. card) this Checkout Session is allowed to accept. The only supported values today are `card` and `ideal`.
    pub payment_method_types: Vec<&'a str>,

    /// The URL the customer will be directed to after the payment or subscription creation is successful.
    pub success_url: &'a str,

    /// A unique string to reference the Checkout Session.
    ///
    /// This can be a customer ID, a cart ID, or similar, and can be used to reconcile the session with your internal systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<&'a str>,

    /// The ID of the customer for this session.
    ///
    /// A new customer will be created unless an existing customer was provided in when the session was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file.
    ///
    /// To access information about the customer once a session is complete, use the `customer` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<&'a str>,

    /// The value (`auto` or `required`) for whether Checkout should collect the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<&'a str>,

    /// The line items, plans, or SKUs purchased by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CheckoutSessionLineItem<'a>>>,

    /// The IETF language tag of the locale Checkout is displayed in.
    ///
    /// If blank or `auto`, the browser's locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<CheckoutSessionLocale>,

    /// The mode of the Checkout Session, one of `payment`, `setup`, or `subscription`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CheckoutSessionMode>,

    // A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in payment mode
    // TODO: payment_intent_data

    // A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in setup mode.
    // TODO: setup_intent_data
    /// Describes the type of transaction being performed by Checkout in order
    /// to customize relevant text on the page, such as the submit button.
    /// `submit_type` can only be specified on Checkout Sessions using line
    /// items or a SKU, but not Checkout Sessions for subscriptions.
    ///
    /// Supported values are `auto`, `book`, `donate`, or `pay`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<CheckoutSessionSubmitType>,
    // A subset of parameters to be passed to subscription creation for Checkout Sessions in subscription mode.
    // TODO: subscription_data
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutSessionLineItem<'a> {
    /// The amount to be collected per unit of the line item.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The name for the line item.
    pub name: &'a str,

    /// The quantity of the line item being purchased.
    pub quantity: u64,

    /// The description for the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// A list of images representing this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>, 
    // TODO: remaining optional fields
}

impl CheckoutSession {
    /// Attach a payment method to a customer
    ///
    /// For more details see [https://stripe.com/docs/api/payment_methods/attach](https://stripe.com/docs/api/payment_methods/attach).
    pub fn create(client: &Client, params: CreateCheckoutSession) -> Response<CheckoutSession> {
        client.post_form("/checkout/sessions", params)
    }
}
