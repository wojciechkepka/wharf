use crate::{Docker, Msg};
use serde_json::Value;
use serde::{Deserialize, Serialize};
use failure::Error;

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

pub struct Networks<'d> {
    docker: &'d Docker,
}
impl<'a> Networks<'a> {
    pub fn new(docker: &'d Docker) -> Networks {
        Networks { docker }
    }
    pub async fn list(&self) -> Result<Vec<Network>, Error> {
        let res = reqwest::get(docker.url.join("networks")?).await?;
        let body = res.text().await?;
        Ok(serde_json::from_str(&body)?)
    }
}
