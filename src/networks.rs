use crate::{Docker, Msg};
use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
impl<'d> Networks<'d> {
    pub fn new(docker: &'d Docker) -> Networks {
        Networks { docker }
    }
    pub async fn list(&self) -> Result<Vec<Network>, Error> {
        let res = self
            .docker
            .client
            .get(self.docker.url.join("networks")?)
            .send()
            .await?;
        let body = res.text().await?;
        Ok(serde_json::from_str(&body)?)
    }
}
