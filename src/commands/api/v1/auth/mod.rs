pub mod gsuite;
pub mod routes;

use std::{
    collections::HashSet,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::commands::api::config::{GoogleRolesConfig, ParsedConfig};

use axum::response::{IntoResponse, Redirect, Response};
use http::{HeaderMap, StatusCode, header};
use jsonwebtoken::{DecodingKey, EncodingKey};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    #[serde(flatten)]
    identity: GoogleAuthentication,
    level: Authorization,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, strum_macros::Display)]
pub enum Authorization {
    NONE = 0,
    READER,
    REQUESTER,
    EDITOR,
}

impl Authorization {
    pub fn from_groups(config: &GoogleRolesConfig, groups: &HashSet<String>) -> Self {
        if !config.editor_roles.is_disjoint(groups) {
            return Self::EDITOR;
        }

        if !config.requester_roles.is_disjoint(groups) {
            return Self::REQUESTER;
        }

        if !config.requester_roles.is_disjoint(groups) {
            return Self::READER;
        }

        return Self::NONE;
    }

    pub fn is_minimum(&self, min: &Self) -> Option<Response> {
        if self >= min {
            None
        } else {
            Some(
                (
                    StatusCode::FORBIDDEN,
                    format!("This operation requires at least clearance level {min}"),
                )
                    .into_response(),
            )
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleAuthentication {
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
    pub fn get(h: &HeaderMap, config: &ParsedConfig) -> Result<Self, Option<Response>> {
        let oauth_cookie = match h
            .get(header::COOKIE)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| {
                h.split(";")
                    .filter_map(|h| h.trim().strip_prefix("access-token="))
                    .next()
            }) {
            Some(c) => c,
            None => return Err(None),
        };

        let info = match jsonwebtoken::decode(
            &oauth_cookie,
            &DecodingKey::from_secret(config.session_key.as_bytes()),
            &Default::default(),
        ) {
            Ok(info) => info,
            Err(e) => match e.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidSignature
                | jsonwebtoken::errors::ErrorKind::ExpiredSignature => return Err(None),
                _ => {
                    return Err(Some(
                        (
                            StatusCode::BAD_REQUEST,
                            format!("invalid authentication: {e}"),
                        )
                            .into_response(),
                    ));
                }
            },
        };

        Ok(info.claims)
    }

    pub fn get_or_redirect(
        h: &HeaderMap,
        config: &ParsedConfig,
        redirect: Option<String>,
    ) -> Result<Self, Response> {
        match Self::get(h, config) {
            Ok(auth) => return Ok(auth),
            Err(Some(resp)) => return Err(resp),
            _ => {}
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

                return match config.oauth.as_ref() {
                    None => {
                        let mut endpoint =
                            Url::parse(&format!("{}://{}/api/v1/auth/local", schema, host))
                                .unwrap();
                        endpoint
                            .query_pairs_mut()
                            .extend_pairs(&[("redirect", redirect.unwrap_or("/".into()))]);
                        let endpoint = endpoint.to_string();
                        Err(Redirect::temporary(&endpoint).into_response())
                    }
                    Some(oauth_config) => {
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
                            ("client_id", &oauth_config.client),
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
                        Err(Redirect::temporary(&endpoint).into_response())
                    }
                };
            }
            _ => {
                return Err((StatusCode::UNAUTHORIZED, "Authorization is required").into_response());
            }
        };
    }

    pub fn minimum(
        h: &HeaderMap,
        config: &ParsedConfig,
        min: &Authorization,
    ) -> Result<Self, Response> {
        let info = Self::get_or_redirect(h, config, None)?;

        if let Some(resp) = info.level.is_minimum(min) {
            return Err(resp);
        }

        Ok(info)
    }

    pub fn set_header(&self, headers: &mut HeaderMap, config: &ParsedConfig) {
        let token = jsonwebtoken::encode(
            &Default::default(),
            self,
            &EncodingKey::from_secret(config.session_key.as_bytes()),
        )
        .unwrap();

        headers.insert(
            http::header::SET_COOKIE,
            format!(
                "access-token={}; HttpOnly; Max-Age={}; Path=/; SameSite=Strict",
                token,
                self.identity.exp - self.identity.iat
            )
            .parse()
            .unwrap(),
        );
    }
}
