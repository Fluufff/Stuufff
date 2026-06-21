use axum::response::{IntoResponse, Redirect, Response};
use axum::{
    Json,
    extract::{Query, State},
};
use http::{HeaderMap, StatusCode, header};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::Deserialize;
use tracing::{error, info};

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
pub struct CallbackParams {
    error: Option<String>,
    code: Option<String>,
    state: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct CallbackState {
    csrf: String,
    url: String,
}
pub async fn google_cb(
    headers: HeaderMap,
    State(state): State<ApiState>,
    Query(params): Query<CallbackParams>,
) -> Response {
    let params_state = params
        .state
        .as_deref()
        .map(serde_urlencoded::from_str::<CallbackState>)
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
        ("client_id", &state.config.oauth_client),
        ("client_secret", &state.config.oauth_secret),
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

    let id_token = match jsonwebtoken::dangerous::insecure_decode::<AuthInfo>(id_token) {
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

    let token = jsonwebtoken::encode(
        &Default::default(),
        &id_token.claims,
        &EncodingKey::from_secret(state.config.session_key.as_bytes()),
    )
    .unwrap();

    info!(
        "redirecting after callback: {}",
        format!("{}://{}{}", schema, host_header, params_state.url)
    );

    let mut r = Redirect::temporary(&format!("{}://{}{}", schema, host_header, params_state.url))
        .into_response();
    r.headers_mut().insert(
        "Set-Cookie",
        format!(
            "access-token={}; HttpOnly; Max-Age=86400; Path=/; SameSite=Strict",
            token
        )
        .parse()
        .unwrap(),
    );

    r
}

pub async fn logout() -> Response {
    let mut r = "OK".into_response();
    r.headers_mut().insert(
        "Set-Cookie",
        "openshift-token=; HttpOnly; Max-Age=0; Path=/; SameSite=Strict"
            .parse()
            .unwrap(),
    );

    r
}
