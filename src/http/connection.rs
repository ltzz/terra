const CLIENT_NAME: &str = "test";

use serde::{Deserialize, Serialize};

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
    pub content: String
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
