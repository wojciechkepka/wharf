#![allow(non_snake_case)]
#[macro_use]
extern crate failure;
pub mod opts;
pub mod containers;
use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

pub struct Docker {
    url: Url,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    Id: String,
    ParentId: String,
    RepoTags: Vec<String>,
    RepoDigests: Vec<String>,
    Created: u32,
    Size: isize,
    VirtualSize: u32,
    Labels: Value,
    Containers: isize,
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


pub struct Images {}
impl Images {
    pub async fn list(docker: &Docker) -> Result<Vec<Image>, Error> {
        let res = reqwest::get(docker.url.join("images/json")?).await?;
        let body = res.text().await?;
        Ok(serde_json::from_str(&body)?)
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

impl Docker {
    pub fn new(s: &str) -> Result<Self, Error> {
        Ok(Docker {
            url: Url::parse(s)?,
        })
    }
}
