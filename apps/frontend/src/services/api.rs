use leptos::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

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

/// `fetch_json` wrapped so the future is `Send`, as `Resource` requires.
#[cfg(not(feature = "ssr"))]
async fn fetch_json_send_safe<T>(url: &'static str) -> Option<T>
where
    T: DeserializeOwned + 'static,
{
    let (tx, rx) = futures::channel::oneshot::channel::<Option<T>>();
    wasm_bindgen_futures::spawn_local(async move {
        let _ = tx.send(fetch_json(url).await);
    });
    rx.await.ok().flatten()
}

/// Server side: build what an API endpoint would return, straight from the
/// in-memory content database, typed as the frontend's mirror struct.
#[cfg(feature = "ssr")]
pub fn from_content_db<S, T>(build: impl FnOnce(&content::ContentDatabase) -> S) -> Option<T>
where
    S: Serialize,
    T: DeserializeOwned,
{
    // shortcut: JSON round-trip between the API type and its frontend mirror; costs one
    // serialisation per SSR render, fine for a few dozen entries.
    with_context::<content::ContentDatabase, _>(|db| serde_json::to_value(build(db)).ok())
        .flatten()
        .and_then(|value| serde_json::from_value(value).ok())
}

/// Resource rendered on the server from `initial` and serialised into the page,
/// then fetched from `url` on client-side navigation. Read it inside `<Suspense>`.
pub fn json_resource<T>(url: &'static str, initial: Option<T>) -> Resource<Option<T>>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
{
    Resource::new(
        || (),
        move |_| {
            let initial = initial.clone();
            async move {
                #[cfg(feature = "ssr")]
                {
                    let _ = url;
                    initial
                }
                #[cfg(not(feature = "ssr"))]
                {
                    let _ = initial;
                    fetch_json_send_safe(url).await
                }
            }
        },
    )
}
