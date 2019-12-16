//! # Wharf ⚓🦀
//!
//! ## Example
//! ```ignore
//! use failure::Error;
//! use wharf::Docker;
//! use wharf::opts::{ContainerBuilderOpts, ListContainersOpts};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     // create docker api instance
//!     let d = Docker::new("http://0.0.0.0:2376")?;
//!     // get containers api handle from d
//!     let containers = d.containers();
//!     // Create instance of query options
//!     let mut opts = ListContainersOpts::new();
//!     opts.all(true);
//!     // iterate over containers
//!     for container in containers.list(opts).await? {
//!         // manipulate container
//!         container.start().await?;
//!         container.stop().await?;
//!     }
//!     // Create a container
//!     let mut container_opts = ContainerBuilderOpts::new();
//!     container_opts
//! 	.image("ubuntu")
//! 	.cmd(&["/bin/echo".into(), "hello".into()])
//! 	.env(&["HTTPS_PROXY=proxy.domain.com:1337"]);
//!
//!     containers.create("jimmy-falcon", &container_opts).await?;
//!
//!     Ok(())
//! }
//! ```

#![allow(non_snake_case)]
#[macro_use]
extern crate failure;
pub mod api;
pub mod opts;
pub mod result;
use crate::api::{Container, Containers, Images, Networks};
use crate::opts::*;
use failure::Error;
use log::*;
use serde::{Deserialize, Serialize};
use url::Url;

/// The main interface to interact with an instance of Docker.
#[derive(Debug)]
pub struct Docker {
    client: reqwest::Client,
    url: Url,
}

impl Docker {
    /// Creates a new instance of docker interface.  
    /// May return an error in case of a bad url.
    pub fn new(url: &str) -> Result<Self, Error> {
        let c = reqwest::ClientBuilder::new();

        Ok(Docker {
            url: Url::parse(url)?,
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
    /// Returns a base64 encoded json with user data.
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
    #[allow(dead_code)]
    fn status(&self) -> String {
        self.Status.clone()
    }
    fn token(&self) -> String {
        self.IdentityToken.clone()
    }
}
