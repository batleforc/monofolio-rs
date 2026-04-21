use serde::de::DeserializeOwned;

#[cfg(not(feature = "ssr"))]
use gloo_net::http::Request;

/// Fetch JSON from a frontend API route during client hydration.
#[cfg(not(feature = "ssr"))]
pub async fn fetch_json<T>(url: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    let resp = Request::get(url).send().await.ok()?;
    let text = resp.text().await.ok()?;
    serde_json::from_str::<T>(&text).ok()
}

/// No-op stub for SSR builds. SSR shouldn't perform client-only API calls.
#[cfg(feature = "ssr")]
pub async fn fetch_json<T>(_url: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    None
}
