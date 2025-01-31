use crate::{error::*, XivApi};
use async_trait::async_trait;
use futures::future::TryFutureExt;
use log::debug;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A builder for a request to XIVAPI.
#[async_trait]
pub trait Builder<'x>
where
    Self: Serialize,
    for<'de> Self::Output: Deserialize<'de>,
{
    type Output;

    /// Sends the request to XIVAPI.
    ///
    /// Generally, this is the function you want. This will download the response and parse it into
    /// the appropriate model for the request.
    ///
    /// # Notes
    /// If you are doing something that changes the output, like specifying specific columns, you will
    /// need to use the `json` function to deserialise into a custom data structure.
    async fn send(&mut self) -> Result<Self::Output> {
        let text = self.text().await?;
        match serde_json::from_str::<ApiError>(&text) {
            Ok(e) => Err(Error::Api(e)),
            Err(_) => serde_json::from_str(&text).map_err(Error::Json),
        }
    }

    /// Deserialises the response into a custom data structure.
    ///
    /// This is only necessary if how you're calling the route changes its default output, like only
    /// fetching specific columns.
    async fn json<T>(&mut self) -> Result<T>
    where
        for<'de> T: Deserialize<'de>,
    {
        let text = self.text().await?;
        serde_json::from_str(&text).map_err(Error::Json)
    }

    /// Downloads the response as text.
    async fn text(&mut self) -> Result<String> {
        let mut route = self.api().url(&self.route());

        let query = serde_urlencoded::to_string(&self).map_err(Error::UrlEncode)?;
        let mut route_query = route.query().map(ToString::to_string).unwrap_or_default();
        if !query.is_empty() {
            if !route_query.is_empty() {
                route_query += "&";
            }
            route_query += &query;
        }
        if !route_query.is_empty() {
            route.set_query(Some(&route_query));
        }

        debug!("route: {:#?}", route);

        let api = self.api();

        api.client
            .get(route)
            .send()
            .map_err(Error::Reqwest)
            .await?
            .text()
            .map_err(Error::Reqwest)
            .await
    }

    fn api(&self) -> &'x XivApi;

    fn route(&self) -> Cow<str>;
}
