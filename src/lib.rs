#![allow(non_snake_case)]
#[macro_use]
extern crate failure;
pub mod api;
pub mod opts;
use crate::api::{Containers, Images, Networks};
use crate::opts::*;
use failure::Error;
use serde::{Deserialize, Serialize};
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

    pub fn auth_token(&self, opts: AuthOpts) -> String {
        unimplemented!()
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
