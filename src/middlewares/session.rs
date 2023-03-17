use crate::error::Error;
use async_session::Session;
use async_session::SessionStore;
use axum::body::Body;
use axum::body::HttpBody;
use axum::headers::Cookie;
use axum::headers::HeaderMap;
use axum::headers::HeaderMapExt;
use axum::http;
use axum::http::header::HeaderName;
use axum::http::HeaderValue;
use axum::http::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use cookie;
use cookie::SameSite;
use std::str::FromStr;
use tracing::debug;
use tracing::error;
use uuid::Uuid;

const KOTOSIRO_SHARING_SESSION_COOKIE: &str = "kotosiro-sharing-session";

const KOTOSIRO_SHARING_SESSION_ID: &str = "kotosiro-sharing-session-id";

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn handler<
    B,
    T: std::marker::Sync + SessionStore,
    U: Default + std::marker::Sync + serde::Serialize,
>(
    mut request: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    let domain = get_domain_name();
    debug!(domain = &domain);
    let store = request
        .extensions()
        .get::<T>()
        .expect("requires `RedisSessionStore` extension");
    let request_headers = request.headers();
    let cookies = request_headers.typed_try_get::<Cookie>().unwrap();
    let cookie = cookies
        .as_ref()
        .and_then(|cookies| cookies.get(KOTOSIRO_SHARING_SESSION_COOKIE));
    debug!("session cookie: {:?}", &cookie);
    // If session cookie is not set, then create and store it.
    if cookie.is_none() {
        let session_id = SessionId::new();
        let session_uuid = session_id.0.hyphenated().to_string();
        let mut session = Session::new();
        match session.insert("user", U::default()) {
            Ok(value) => value,
            Err(e) => {
                error!("unable to update session with user: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        let session_before = session.clone();
        let cookie: String = match store.store_session(session).await {
            Ok(value) => match value {
                Some(cookie) => cookie,
                None => {
                    error!("unable to fetch cookie value from new session");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            },
            Err(e) => {
                error!("error whilst attempting to update store {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        debug!("updated Session: {:?}", &session_before.id());
        debug!(
            r#"created cookie "{:?}" = "{:?}" for UUID "{}" / for session: "{:?}""#,
            KOTOSIRO_SHARING_SESSION_COOKIE, &cookie, &session_uuid, &session_before
        );
        let body = Body::empty().boxed_unsync();
        let mut response = Response::builder()
            .status(StatusCode::SEE_OTHER)
            .body(body)
            .unwrap();
        let headers = response.headers_mut();
        headers.insert(
            http::header::LOCATION,
            request.uri().to_string().parse().unwrap(),
        );
        let cookie = cookie::Cookie::build(KOTOSIRO_SHARING_SESSION_COOKIE, &cookie)
            .domain(&domain)
            .path("/")
            .secure(true)
            .same_site(SameSite::Strict)
            .http_only(true)
            .finish();
        let cookie = HeaderValue::from_str(cookie.to_string().as_str()).unwrap();
        headers.insert(http::header::SET_COOKIE, cookie);
        let response = response.into_response();
        debug!(r#"session UUID creation: done. response: "{:?}""#, response);
        return Ok(response);
    }
    // Session cookie is found.
    debug!(
        r#"got session cookie from user agent, "{:?}" = "{:?}""#,
        KOTOSIRO_SHARING_SESSION_COOKIE,
        &cookie.unwrap()
    );
    let cookie_before = cookie.clone();
    let session: Session = match store.load_session(cookie.unwrap().to_string()).await {
        Ok(value) => match value {
            Some(value) => value,
            None => {
                error!(
                    r#"unable to locate session in backend for cookie: "{:?}""#,
                    cookie_before
                );
                return Err(StatusCode::REQUEST_TIMEOUT);
            }
        },
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let mut response_headers = copy_headers(request_headers);
    let request_headers = request.headers_mut();
    request_headers.insert(
        KOTOSIRO_SHARING_SESSION_ID,
        HeaderValue::from_str(format!("{}", &session.id()).as_str()).unwrap(),
    );
    let mut response = next.run(request).await;
    let mut _response_headers = response.headers_mut();
    response_headers.insert(
        KOTOSIRO_SHARING_SESSION_ID,
        HeaderValue::from_str(format!("{}", session.id()).as_str()).unwrap(),
    );
    _response_headers = &mut response_headers;
    debug!(r#"response headers: "{:?}"#, &_response_headers);
    Ok(response)
}

fn get_domain_name() -> String {
    let domain = match std::env::var("DOMAIN") {
        Ok(value) => value,
        Err(e) => panic!("requires $DOMAIN environment variable: {:?}", e),
    };
    if domain.as_str() == "" {
        panic!(
            "empty $DOMAIN environment variable: {:?}",
            Error::EnvironmentVariableMissing
        )
    };
    domain
}

fn copy_headers(request_headers: &HeaderMap) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for header in request_headers.iter() {
        let (k, v) = header;
        let value: &str = v.to_str().expect("unable to fetch header value");
        let value: HeaderValue = HeaderValue::from_str(value).unwrap();
        let name: HeaderName = HeaderName::from_str(k.as_str()).unwrap();
        headers.insert(name, value);
    }
    headers
}
