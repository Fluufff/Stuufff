use crate::config::error::RuntimeError;
use serde::Deserialize;
use std::collections::HashSet;
use yup_oauth2::{ServiceAccountAuthenticator, read_service_account_key};

#[derive(Deserialize)]
pub struct WorkspaceGroupsResponse {
    groups: Vec<WorkspaceGroupsEntry>,
}

#[derive(Deserialize)]
pub struct WorkspaceGroupsEntry {
    id: String,
    // kind: String,
    // etag: String,
    // email: String,
    // name: String,
    // description: String,
    // directMembersCount: String,
    // adminCreated: bool,
    // nonEditableAliases: Vec<String>,
}

pub async fn get_groups_for_user(email: &str) -> Result<HashSet<String>, RuntimeError> {
    let key = read_service_account_key("local_secrets/groups-reader.json").await?;

    let auth = ServiceAccountAuthenticator::builder(key)
        .subject("juravenator@fluufff.org")
        .build()
        .await?;

    let token = auth
        .token(&["https://www.googleapis.com/auth/admin.directory.group.readonly"])
        .await?;

    let client = reqwest::Client::new();

    let resp = client
        .get("https://admin.googleapis.com/admin/directory/v1/groups")
        .query(&[("userKey", email)])
        .bearer_auth(token.token().unwrap())
        .send()
        .await?;

    let resp: WorkspaceGroupsResponse = resp.json().await?;

    let groups = resp.groups.into_iter().map(|e| e.id).collect();

    Ok(groups)
}
