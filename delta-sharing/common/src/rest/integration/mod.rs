use axum::{
    body::Body,
    http::{self, Method, Request},
    Router,
};
use http_body_util::BodyExt;

pub use self::catalogs::*;
pub use self::external_locations::*;

mod catalogs;
mod external_locations;

pub async fn collect_body<T>(response: axum::http::Response<Body>) -> T
where
    T: serde::de::DeserializeOwned,
{
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).unwrap()
}

pub fn create_request<T>(method: Method, uri: &str, body: Option<T>) -> Request<Body>
where
    T: serde::Serialize,
{
    if let Some(body) = body {
        Request::builder()
            .method(method)
            .uri(uri)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(serde_json::to_vec(&body).unwrap()))
            .unwrap()
    } else {
        Request::builder()
            .method(method)
            .uri(uri)
            .body(Body::empty())
            .unwrap()
    }
}
