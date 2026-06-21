// pub mod headers;
pub mod routes;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::commands::api::config::ParsedConfig;

use axum::response::{IntoResponse, Redirect, Response};
use http::{HeaderMap, StatusCode, header};
use jsonwebtoken::{DecodingKey, EncodingKey};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    exp: usize,
    iat: usize,
    sub: String,
    email: String,
    name: String,
    picture: String,
    given_name: String,
    family_name: String,
}

#[derive(Serialize, Deserialize)]
struct CsrfData {
    exp: usize,
}

impl AuthInfo {
    pub fn get(h: &HeaderMap, config: &ParsedConfig) -> Option<Self> {
        let oauth_cookie = h
            .get(header::COOKIE)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| {
                h.split(";")
                    .filter_map(|h| h.trim().strip_prefix("access-token="))
                    .next()
            })?;

        let info = jsonwebtoken::decode(
            &oauth_cookie,
            &DecodingKey::from_secret(config.session_key.as_bytes()),
            &Default::default(),
        )
        .ok()?;

        Some(info.claims)
    }

    pub fn get_or_redirect(
        h: &HeaderMap,
        config: &ParsedConfig,
        redirect: Option<String>,
    ) -> Result<Self, Response> {
        if let Some(auth) = Self::get(h, config) {
            return Ok(auth);
        }

        match (h.get(header::ACCEPT), h.get(header::HOST)) {
            (Some(accept), Some(host))
                if accept.to_str().unwrap_or_default().contains("text/html") =>
            {
                let host = match host.to_str() {
                    Ok(h) => h,
                    Err(_) => {
                        return Err((StatusCode::BAD_REQUEST, "bad host header").into_response());
                    }
                };
                let schema = match ["localhost", "[::1]", "127.0.0.1"]
                    .iter()
                    .any(|p| host.starts_with(p))
                {
                    true => "http",
                    false => "https",
                };
                let csrf = {
                    let mut exp = SystemTime::now();
                    exp += Duration::from_secs(60 * 60);
                    let exp = exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
                    jsonwebtoken::encode(
                        &Default::default(),
                        &CsrfData { exp },
                        &EncodingKey::from_secret(config.session_key.as_bytes()),
                    )
                    .unwrap()
                };
                let mut endpoint =
                    Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();
                endpoint.query_pairs_mut().extend_pairs(&[
                    ("response_type", "code"),
                    ("scope", "openid email profile"),
                    ("client_id", &config.oauth_client),
                    (
                        "redirect_uri",
                        &format!("{}://{}/api/v1/auth/google_cb", schema, host),
                    ),
                    (
                        "state",
                        &format!("csrf={}&url={}", csrf, redirect.unwrap_or("/".into())),
                    ),
                    ("hd", "fluufff.org"),
                ]);
                let endpoint = endpoint.to_string();
                return Err(Redirect::temporary(&endpoint).into_response());
            }
            _ => {
                return Err((StatusCode::UNAUTHORIZED, "Authorization is required").into_response());
            }
        };
    }
}
