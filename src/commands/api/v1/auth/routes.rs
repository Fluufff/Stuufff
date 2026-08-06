use std::time::{SystemTime, UNIX_EPOCH};

use axum::response::{IntoResponse, Response};
use axum::{
    Json,
    extract::{Query, State},
};
use http::{HeaderMap, StatusCode, header};
use jsonwebtoken::DecodingKey;
use serde::Deserialize;
use tracing::{error, info};

use crate::commands::api::v1::auth::gsuite::get_groups_for_user;
use crate::commands::api::v1::auth::{Authorization, GoogleAuthentication};

use super::super::ApiState;
use super::AuthInfo;

#[derive(Deserialize)]
pub struct FromParams {
    from: Option<String>,
}
pub async fn whoami(
    headers: HeaderMap,
    State(state): State<ApiState>,
    Query(params): Query<FromParams>,
) -> Response {
    let auth = match AuthInfo::get_or_redirect(&headers, &state.config, params.from) {
        Ok(auth) => auth,
        Err(r) => return r,
    };

    Json(auth).into_response()
}

#[derive(Deserialize, Debug)]
pub struct LocalCallbackParams {
    redirect: Option<String>,
}
pub async fn local_cb(
    State(state): State<ApiState>,
    Query(params): Query<LocalCallbackParams>,
) -> Response {
    match state.config.oauth.as_ref() {
        None => {}
        _ => return (StatusCode::BAD_REQUEST, "Oauth is enabled in config").into_response(),
    };

    let iat = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let exp = iat + 60 * 60 * 24; // 24h token

    let mut auth = AuthInfo {
        identity: GoogleAuthentication {
            iat,
            exp,
            sub: "admin".into(),
            email: "admin@localhost".into(),
            name: "local admin".into(),
            picture: "/media/local_admin.png".into(),
            given_name: "local".into(),
            family_name: "admin".into(),
        },
        level: Authorization::EDITOR,
    };
    auth.identity.exp = auth.identity.iat + 60 * 60 * 24; // 24h token

    let redirect = params.redirect.unwrap_or("/".into());
    info!("redirecting after callback: {redirect}");

    let mut r = format!("<script>window.location = '{redirect}';</script>",).into_response();

    auth.set_header(r.headers_mut(), &state.config);

    r.headers_mut().insert(
        http::header::CONTENT_TYPE,
        mime_guess::mime::TEXT_HTML_UTF_8
            .to_string()
            .parse()
            .unwrap(),
    );

    r
}

#[derive(Deserialize, Debug)]
pub struct GoogleCallbackParams {
    error: Option<String>,
    code: Option<String>,
    state: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct GoogleCallbackState {
    csrf: String,
    url: String,
}
pub async fn google_cb(
    headers: HeaderMap,
    State(state): State<ApiState>,
    Query(params): Query<GoogleCallbackParams>,
) -> Response {
    let oauth_config = match state.config.oauth.as_ref() {
        Some(c) => c,
        None => return (StatusCode::BAD_REQUEST, "Oauth is disabled in config").into_response(),
    };
    let params_state = params
        .state
        .as_deref()
        .map(serde_urlencoded::from_str::<GoogleCallbackState>)
        .transpose()
        .unwrap_or_default();

    let (code, params_state) = match (&params.error, &params.code, &params_state) {
        (None, Some(c), Some(s)) => (c.as_str(), s),

        (Some(e), _, _) => return (StatusCode::BAD_REQUEST, e.clone()).into_response(),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                "no 'code' or 'access_token' query parameter",
            )
                .into_response();
        }
    };
    let host_header = match headers.get(header::HOST) {
        Some(h) => h,
        None => return (StatusCode::BAD_REQUEST, "you are not a browser").into_response(),
    };
    let host_header = match host_header.to_str() {
        Ok(h) => h,
        Err(_) => return (StatusCode::BAD_REQUEST, "bad host header").into_response(),
    };

    match jsonwebtoken::decode::<super::CsrfData>(
        &params_state.csrf,
        &DecodingKey::from_secret(state.config.session_key.as_bytes()),
        &Default::default(),
    ) {
        Ok(_) => {}
        _ => return (StatusCode::BAD_REQUEST, "bad CSRF data").into_response(),
    };

    let schema = match ["localhost", "[::1]", "127.0.0.1"]
        .iter()
        .any(|p| host_header.starts_with(p))
    {
        true => "http",
        false => "https",
    };
    let params = [
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("client_id", &oauth_config.client),
        ("client_secret", &oauth_config.secret),
        (
            "redirect_uri",
            &format!("{}://{}/api/v1/auth/google_cb", schema, host_header,),
        ),
    ];
    let client = reqwest::Client::builder()
        .connect_timeout(core::time::Duration::from_secs(30))
        .build()
        .unwrap();
    let resp = match client
        .post("https://oauth2.googleapis.com/token")
        // .post(&cri.oauth.as_ref().unwrap().oauth_token_endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("cannot send oauth request: {:?}", e);
            return (StatusCode::SERVICE_UNAVAILABLE, "bad oauth response").into_response();
        }
    };

    #[derive(Deserialize)]
    struct OauthResponse {
        id_token: String,
    }
    let id_token = match resp.status() {
        StatusCode::OK => {
            let resp = match resp.json::<OauthResponse>().await {
                Ok(r) => r,
                Err(e) => {
                    error!("bad oauth json payload: {:?}", e);
                    return (
                        StatusCode::SERVICE_UNAVAILABLE,
                        "bad oauth response payload",
                    )
                        .into_response();
                }
            };
            resp.id_token.clone()
        }
        _ => {
            let bytes = resp.bytes().await.ok().unwrap_or_default();
            error!("bad oauth response payload: {:?}", bytes);
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "bad oauth response payload",
            )
                .into_response();
        }
    };

    let id_token = match jsonwebtoken::dangerous::insecure_decode::<GoogleAuthentication>(id_token)
    {
        Ok(t) => t,
        Err(e) => {
            error!("no valid google token payload: {}", e);
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "no valid google token payload",
            )
                .into_response();
        }
    };

    let groups = match get_groups_for_user(oauth_config, &id_token.claims.email).await {
        Ok(r) => r,
        Err(e) => {
            error!("cannot send oauth request: {:?}", e);
            return (StatusCode::SERVICE_UNAVAILABLE, "bad oauth response").into_response();
        }
    };

    let mut auth = AuthInfo {
        identity: id_token.claims,
        level: Authorization::from_groups(&state.config.access.google_roles, &groups),
    };
    auth.identity.exp = auth.identity.iat + 60 * 60 * 24; // 24h token

    info!(
        "redirecting after callback: {}",
        format!("{}://{}{}", schema, host_header, params_state.url)
    );

    let mut r = format!(
        "<script>window.location = '{}://{}{}';</script>",
        schema, host_header, params_state.url
    )
    .into_response();

    auth.set_header(r.headers_mut(), &state.config);

    r.headers_mut().insert(
        http::header::CONTENT_TYPE,
        mime_guess::mime::TEXT_HTML_UTF_8
            .to_string()
            .parse()
            .unwrap(),
    );

    r
}

pub async fn logout() -> Response {
    let mut r = "OK".into_response();
    r.headers_mut().insert(
        http::header::SET_COOKIE,
        "access-token=; HttpOnly; Max-Age=0; Path=/; SameSite=Strict"
            .parse()
            .unwrap(),
    );

    r
}
