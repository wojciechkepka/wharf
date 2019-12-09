#![allow(non_snake_case)]
#[macro_use]
extern crate failure;
pub mod api;
pub mod opts;
use crate::api::{Container, Containers, Images, Networks};
use crate::opts::*;
use failure::Error;
use log::*;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug)]
pub struct Docker {
    client: reqwest::Client,
    url: Url,
}

impl Docker {
    pub fn new(s: &str) -> Result<Self, Error> {
        let c = reqwest::ClientBuilder::new();

        Ok(Docker {
            url: Url::parse(s)?,
            client: c.no_proxy().build()?,
        })
    }
    /// Get reference to a specific container interface
    pub fn container(&self, id: &str) -> Container {
        Container::new(&self, id)
    }
    /// Get reference to api interface of containers
    pub fn containers(&self) -> Containers {
        Containers::new(&self)
    }
    /// Get reference to api interface of images
    pub fn images(&self) -> Images {
        Images::new(&self)
    }
    /// Get reference to api interface of networks
    pub fn networks(&self) -> Networks {
        Networks::new(&self)
    }

    /// Get auth token for authorized operations
    pub async fn authenticate(&self, opts: AuthOpts) -> Result<String, Error> {
        let res = self
            .client
            .post(self.url.join("/auth")?)
            .json(opts.opts())
            .send()
            .await?;
        debug!("{:?}", res);
        let status = res.status().as_u16();
        let text = res.text().await?;
        debug!("{}", text);
        match status {
            200 => {
                let msg: AuthMsg = serde_json::from_str(&text)?;
                Ok(msg.token())
            }
            204 => Ok("".to_string()),
            500 => Err(format_err!("server error")),
            _ => Err(format_err!("{}", text)),
        }
    }
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
#[derive(Serialize, Deserialize)]
struct AuthMsg {
    Status: String,
    IdentityToken: String,
}
impl AuthMsg {
    fn status(&self) -> String {
        self.Status.clone()
    }
    fn token(&self) -> String {
        self.IdentityToken.clone()
    }
}
