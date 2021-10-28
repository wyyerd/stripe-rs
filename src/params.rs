use crate::config::{err, ok, Client, Response};
use crate::error::Error;
use crate::resources::ApiVersion;
use futures_util::stream::TryStream;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct AppInfo {
    pub name: String,
    pub url: Option<String>,
    pub version: Option<String>,
}

#[derive(Clone, Default)]
pub struct Headers {
    pub client_id: Option<String>,
    pub stripe_version: Option<ApiVersion>,
    pub stripe_account: Option<String>,
    pub user_agent: Option<String>,
}

/// Implemented by types which represent stripe objects.
pub trait Object {
    /// The canonical id type for this object.
    type Id;
    /// The id of the object.
    fn id(&self) -> Self::Id;
    /// The object's type, typically represented in wire format as the `object` property.
    fn object(&self) -> &'static str;
}

/// A deleted object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deleted<T> {
    /// Unique identifier for the object.
    pub id: T,
    /// Always true for a deleted object.
    pub deleted: bool,
}

/// The `Expand` struct is used to serialize `expand` arguments in retrieve and list apis.
#[doc(hidden)]
#[derive(Serialize)]
pub struct Expand<'a> {
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl Expand<'_> {
    pub(crate) fn is_empty(expand: &[&str]) -> bool {
        expand.is_empty()
    }
}

/// An id or object.
///
/// By default stripe will return an id for most fields, but if more detail is
/// necessary the `expand` parameter can be provided to ask for the id to be
/// loaded as an object instead:
///
/// ```rust,ignore
/// Charge::retrieve(&client, &charge_id, &["invoice.customer"])
/// ```
///
/// See [https://stripe.com/docs/api/expanding_objects](https://stripe.com/docs/api/expanding_objects).
#[derive(Clone, Debug, Serialize, Deserialize)] // TODO: Implement deserialize by hand for better error messages
#[serde(untagged)]
pub enum Expandable<T: Object> {
    Id(T::Id),
    Object(Box<T>),
}

impl<T> Expandable<T>
where
    T: Object,
    T::Id: Clone,
{
    pub fn id(&self) -> T::Id {
        match self {
            Expandable::Id(id) => id.clone(),
            Expandable::Object(obj) => obj.id(),
        }
    }
}

impl<T: Object> Expandable<T> {
    pub fn is_object(&self) -> bool {
        match self {
            Expandable::Id(_) => false,
            Expandable::Object(_) => true,
        }
    }

    pub fn as_object(&self) -> Option<&T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(&obj),
        }
    }

    #[deprecated(
        note = "Renamed `into_object` per rust api design guidelines (may be removed in v0.12)"
    )]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_object(self) -> Option<T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(*obj),
        }
    }

    pub fn into_object(self) -> Option<T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(*obj),
        }
    }
}

/// Implemented by types which support cursor-based pagination,
/// typically with an id, allowing them to be fetched using a `List`
/// returned by the corresponding "list" api request.
pub trait Paginate {
    type Cursor: AsCursor;
    fn cursor(&self) -> Self::Cursor;
}

pub trait AsCursor: AsRef<str> {}

impl<'a> AsCursor for &'a str {}
impl AsCursor for String {}

impl<T> Paginate for T
where
    T: Object,
    T::Id: AsCursor,
{
    type Cursor = T::Id;
    fn cursor(&self) -> Self::Cursor {
        self.id()
    }
}

/// A single page of a cursor-paginated list of an object.
#[derive(Debug, Deserialize, Serialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub total_count: Option<u64>,
    pub url: String,

    #[serde(default)]
    #[serde(skip)]
    pub(crate) params: Option<String>,
}

impl<T> List<T> {
    pub fn params<P: serde::Serialize>(mut self, params: &P) -> Result<Self, Error> {
        self.params = Some(serde_qs::to_string(params).map_err(Error::serialize)?);
        Ok(self)
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List {
            data: Vec::new(),
            has_more: false,
            total_count: None,
            url: String::new(),
            params: None,
        }
    }
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        List {
            data: self.data.clone(),
            has_more: self.has_more,
            total_count: self.total_count,
            url: self.url.clone(),
            params: self.params.clone(),
        }
    }
}

impl<T: DeserializeOwned + Send + 'static> List<T> {
    /// Prefer `List::next` when possible
    pub fn get_next(
        client: &Client,
        url: &str,
        last_id: &str,
        params: Option<&str>,
    ) -> Response<List<T>> {
        if url.starts_with("/v1/") {
            // TODO: Maybe parse the URL?  Perhaps `List` should always parse its `url` field.
            let mut url = url.trim_start_matches("/v1/").to_string();
            if url.contains('?') {
                url.push_str(&format!("&starting_after={}", last_id));
            } else {
                url.push_str(&format!("?starting_after={}", last_id));
            }
            if let Some(params) = params {
                if !params.is_empty() {
                    url.push_str(&format!("&{}", params));
                }
            }
            let resp = client.get(&url);

            #[cfg(feature = "blocking")]
            let resp: Result<List<T>, Error> = resp.map(|mut list: List<T>| {
                list.params = params.map(|s| s.to_string());
                list
            });

            #[cfg(not(feature = "blocking"))]
            let resp = {
                let params_string = params.map(|p| p.to_string());
                let resp = resp.then(move |res: Result<List<T>, _>| {
                    let res = match res {
                        Ok(mut list) => {
                            if let Some(params_string) = params_string {
                                list.params = Some(params_string);
                            }
                            Ok(list)
                        }
                        Err(e) => Err(e),
                    };

                    futures_util::future::ready(res)
                });

                Box::pin(resp)
            };

            resp
        } else {
            err(Error::Unsupported("URL for fetching additional data uses different API version"))
        }
    }
}

impl<T: Paginate + DeserializeOwned + Send + 'static> List<T> {
    /// Repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// the page size specified in params, or Stripe's default page size if none is specified.
    #[cfg(feature = "blocking")]
    pub fn get_all(self, client: &Client) -> Response<Vec<T>> {
        let mut data = Vec::new();
        let mut next = self;
        loop {
            if next.has_more {
                let resp = next.next(&client)?;
                data.extend(next.data);
                next = resp;
            } else {
                data.extend(next.data);
                break;
            }
        }
        Ok(data)
    }

    /// Get all values in this list, consuming self and paginating until all values are fetched.
    ///
    /// This function repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// the page size specified in params, or Stripe's default page size if none is specified.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    ///
    /// let value_stream = list.get_all(&client).boxed();
    /// while let Some(val) = value_stream.try_next().await? {
    ///     println!("GOT = {:?}", val);
    /// }
    ///
    /// // Alternatively, you can collect all values into a Vec
    /// let all_values = list.get_all(&client).try_collect::<Vec<_>>().await?;
    /// ```
    #[cfg(not(feature = "blocking"))]
    pub fn get_all(self, client: &Client) -> impl TryStream<Ok = T, Error = Error> {
        // We are going to be popping items off the end of the list, so we need to reverse it.
        let mut init_list = self;
        init_list.data.reverse();

        futures_util::stream::unfold(Some((init_list, client.clone())), |state| async move {
            let (mut list, client) = state?; // if none, we sent the last item in the list last iteration
            let val = list.data.pop()?; // the initial list was empty, so we're done.

            if !list.data.is_empty() {
                return Some((Ok(val), Some((list, client)))); // some value on this page that isn't the last value on the page
            }

            if !list.has_more {
                return Some((Ok(val), None)); // final value of the stream, no errors
            }

            // We're on the last value of this page, but there's more. We need to fetch the next page.
            let last_id = val.cursor();
            let resp = List::get_next(&client, &list.url, last_id.as_ref(), list.params.as_deref());

            match resp.await {
                Ok(mut next_list) => {
                    next_list.data.reverse();

                    // Yield last value of this page, the next page (and client) becomes the state
                    Some((Ok(val), Some((next_list, client))))
                }
                Err(e) => Some((Err(e), None)), // we ran into an error. the last value of the stream will be the error.
            }
        })
    }

    /// Fetch an additional page of data from stripe.
    pub fn next(&self, client: &Client) -> Response<List<T>> {
        if let Some(last_id) = self.data.last().map(|d| d.cursor()) {
            List::get_next(client, &self.url, last_id.as_ref(), self.params.as_deref())
        } else {
            ok(List {
                data: Vec::new(),
                has_more: false,
                total_count: self.total_count,
                url: self.url.clone(),
                params: self.params.clone(),
            })
        }
    }
}

pub type Metadata = HashMap<String, String>;
pub type Timestamp = i64;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct RangeBounds<T> {
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
}

impl<T> Default for RangeBounds<T> {
    fn default() -> Self {
        RangeBounds { gt: None, gte: None, lt: None, lte: None }
    }
}

/// A set of generic request parameters that can be used on
/// list endpoints to filter their results by some timestamp.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RangeQuery<T> {
    Exact(T),
    Bounds(RangeBounds<T>),
}

impl<T> RangeQuery<T> {
    /// Filter results to exactly match a given value
    pub fn eq(value: T) -> RangeQuery<T> {
        RangeQuery::Exact(value)
    }

    /// Filter results to be after a given value
    pub fn gt(value: T) -> RangeQuery<T> {
        let mut bounds = RangeBounds::default();
        bounds.gt = Some(value);
        RangeQuery::Bounds(bounds)
    }

    /// Filter results to be after or equal to a given value
    pub fn gte(value: T) -> RangeQuery<T> {
        let mut bounds = RangeBounds::default();
        bounds.gte = Some(value);
        RangeQuery::Bounds(bounds)
    }

    /// Filter results to be before to a given value
    pub fn lt(value: T) -> RangeQuery<T> {
        let mut bounds = RangeBounds::default();
        bounds.lt = Some(value);
        RangeQuery::Bounds(bounds)
    }

    /// Filter results to be before or equal to a given value
    pub fn lte(value: T) -> RangeQuery<T> {
        let mut bounds = RangeBounds::default();
        bounds.lte = Some(value);
        RangeQuery::Bounds(bounds)
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum IdOrCreate<'a, T> {
    Id(&'a str),
    Create(&'a T),
}

// NOTE: Only intended to handle conversion from ASCII CamelCase to SnakeCase
//   This function is used to convert static Rust identifiers to snakecase
// TODO: pub(crate) fn
pub fn to_snakecase(camel: &str) -> String {
    let mut i = 0;
    let mut snake = String::new();
    let mut chars = camel.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            if i > 0 && !chars.peek().unwrap_or(&'A').is_uppercase() {
                snake.push('_');
            }
            snake.push(ch.to_lowercase().next().unwrap_or(ch));
        } else {
            snake.push(ch);
        }
        i += 1;
    }

    snake
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_snakecase() {
        use super::to_snakecase;

        assert_eq!(to_snakecase("snake_case").as_str(), "snake_case");
        assert_eq!(to_snakecase("CamelCase").as_str(), "camel_case");
        assert_eq!(to_snakecase("XMLHttpRequest").as_str(), "xml_http_request");
        assert_eq!(to_snakecase("UPPER").as_str(), "upper");
        assert_eq!(to_snakecase("lower").as_str(), "lower");
    }
}
