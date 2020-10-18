const CLIENT_NAME: &str = "test";

use serde::{Deserialize, Serialize};
use percent_encoding::utf8_percent_encode;
use percent_encoding::NON_ALPHANUMERIC;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientRegisterResponse {
    id: String,
    name: String,
    website: Option<String>,
    redirect_uri: Option<String>,
    client_id: String,
    client_secret: String,
    vapid_key: String
}

pub fn register_client(host: &str) -> Result<ClientRegisterResponse, Box<dyn std::error::Error>> {
    let request_body = format!(
        "client_name={}&redirect_uris=urn:ietf:wg:oauth:2.0:oob&scopes=read write follow",
        CLIENT_NAME
    );

    let url = format!(
        "https://{}/api/v1/apps",
        host
    );

    let client = reqwest::blocking::Client::new();
    let resp = client.post(&url)
            .body(request_body)
            .send()?;

    let body = resp.text()?;

    // let body = dbg!(body);

    let ret: ClientRegisterResponse = serde_json::from_str(&body)?;
    Ok(ret)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Toot {
    pub id: String,
    pub content: String,
    pub account: Account
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub display_name: String,
}

pub fn post_status(host: &String, access_token: &String, status: &String) -> Result<Vec<Toot>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://{}/api/v1/statuses",
        host
    );

    let status_encoded = utf8_percent_encode(status, NON_ALPHANUMERIC).to_string();
    let body = dbg!("status=".to_owned() + &status_encoded);

    let client = reqwest::blocking::Client::new();

    let auth_value = format!(
        "Bearer {}",
        access_token
    );

    let url = dbg!(url);

    let resp = client.post(&url)
            .header("Authorization", auth_value)
            .body(body)
            .send()?;

    let body = resp.text()?;

    let ret: Vec<Toot> = serde_json::from_str(&body)?;
    Ok(ret)
}


pub fn get_toot(host: &String, access_token: &String) -> Result<Vec<Toot>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://{}/api/v1/timelines/home",
        host
    );

    let client = reqwest::blocking::Client::new();

    let auth_value = format!(
        "Bearer {}",
        access_token
    );

    let url = dbg!(url);

    let resp = client.get(&url)
            .header("Authorization", auth_value)
            .send()?;
    // let resp = dbg!(resp);

    let body = resp.text()?;

    // let body = dbg!(body);

    let ret: Vec<Toot> = serde_json::from_str(&body)?;
    Ok(ret)
}
