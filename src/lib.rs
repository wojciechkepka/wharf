#![allow(non_snake_case)]
#[macro_use]
extern crate failure;
pub mod containers;
pub mod images;
pub mod networks;
pub mod opts;
use crate::containers::Containers;
use crate::images::Images;
use crate::networks::Networks;
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
