#![allow(non_snake_case)]
#[macro_use]
extern crate failure;
pub mod containers;
pub mod images;
pub mod opts;
use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

pub struct Docker {
    client: reqwest::Client,
    url: Url,
}

impl Docker {
    pub fn new(s: &str) -> Result<Self, Error> {
        Ok(Docker {
            url: Url::parse(s)?,
            client: reqwest::Client::new(),
        })
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    Name: String,
    Id: String,
    Created: String,
    Scope: String,
    Driver: String,
    EnableIPv6: bool,
    Internal: bool,
    Attachable: bool,
    Ingress: bool,
    IPAM: Value,
    Options: Value,
}

#[derive(Serialize, Deserialize)]
struct Msg {
    message: String,
}
impl Msg {
    fn msg(self) -> String {
        self.message
    }
}

pub struct Networks {}
impl Networks {
    pub async fn list(docker: &Docker) -> Result<Vec<Network>, Error> {
        let res = reqwest::get(docker.url.join("networks")?).await?;
        let body = res.text().await?;
        Ok(serde_json::from_str(&body)?)
    }
}
