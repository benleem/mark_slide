use actix_web::web;
use reqwest::{Client, Url};
use serde::Deserialize;
use std::error::Error;

use crate::models::auth::AppState;

#[derive(Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Deserialize)]
pub struct GitHubUserResult {
    pub id: String,
    pub name: String,
    pub email: String,
}

pub async fn request_token(
    authorization_code: &str,
    data: &web::Data<AppState>,
) -> Result<OAuthResponse, Box<dyn Error>> {
    let redirect_url = data.env.github_oauth_redirect_url.to_owned();
    let client_secret = data.env.github_oauth_client_secret.to_owned();
    let client_id = data.env.github_oauth_client_id.to_owned();

    let root_url = "https://oauth2.githubapis.com/token";
    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_url.as_str()),
        ("client_id", client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", client_secret.as_str()),
    ];
    let response = client.post(root_url).form(&params).send().await?;

    if response.status().is_success() {
        let oauth_response = response.json::<OAuthResponse>().await?;
        Ok(oauth_response)
    } else {
        let message = "An error occurred while trying to retrieve access token.";
        Err(From::from(message))
    }
}

pub async fn get_github_user(
    access_token: &str,
    id_token: &str,
) -> Result<GitHubUserResult, Box<dyn Error>> {
    let client = Client::new();
    let mut url = Url::parse("https://www.githubapis.com/oauth2/v1/userinfo").unwrap();
    url.query_pairs_mut().append_pair("alt", "json");
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let response = client.get(url).bearer_auth(id_token).send().await?;

    if response.status().is_success() {
        let user_info = response.json::<GitHubUserResult>().await?;
        Ok(user_info)
    } else {
        let message = "An error occurred while trying to retrieve user information.";
        Err(From::from(message))
    }
}