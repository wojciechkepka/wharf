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
#[macro_use]
pub mod api;
pub mod opts;
pub mod result;
use crate::api::*;
use crate::opts::*;
use failure::Error;
use http::header::HeaderValue;
use http::uri::PathAndQuery;
use hyper::{body::to_bytes, client::HttpConnector, Body, Method, Request, Response, Uri};
use log::*;
use serde::{Deserialize, Serialize};
use std::str;
use std::str::FromStr;

/// The main interface to interact with an instance of Docker.
#[derive(Debug)]
pub struct Docker {
    client: hyper::Client<HttpConnector>,
    url: Uri,
}

impl Docker {
    /// Creates a new instance of docker interface.  
    /// May return an error in case of a bad url.
    pub fn new(url: &str) -> Result<Self, Error> {
        Ok(Docker {
            url: url.parse()?,
            client: hyper::Client::new(),
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
    async fn req(
        &self,
        method: Method,
        path: String,
        query: Option<String>,
        body: Body,
        headers: Option<Vec<(&'static str, String)>>,
    ) -> Result<Response<Body>, Error> {
        let mut uri = self.url.clone().into_parts();
        match query {
            Some(q) => {
                uri.path_and_query = Some(PathAndQuery::from_str(&format!("{}?{}", path, q))?)
            }
            None => uri.path_and_query = Some(PathAndQuery::from_str(&path)?),
        }
        let uri = Uri::from_parts(uri)?;
        let mut req = Request::builder().method(method).uri(uri);
        if let Some(req_h) = req.headers_mut() {
            if let Some(h) = headers {
                h.iter().for_each(|header| {
                    req_h.insert(header.0, HeaderValue::from_str(&header.1).unwrap());
                });
            }
        }
        let req = req.body(body).expect("failed to build a request");

        trace!("{:?}", req);
        let res = self.client.request(req).await?;

        trace!("{:?}", res);
        Ok(res)
    }
    /// Get auth token for authorized operations  
    /// Returns a base64 encoded json with user data.
    pub async fn authenticate(&self, opts: &AuthOpts) -> Result<String, Error> {
        let res = self
            .req(
                Method::POST,
                "/auth".into(),
                None,
                Body::from(serde_json::to_string(opts.opts())?),
                None,
            )
            .await?;
        debug!("{:?}", res);
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(text.as_ref())?);
        match status {
            200 => {
                let msg: AuthMsg = serde_json::from_slice(&text)?;
                Ok(msg.token())
            }
            204 => Ok("".to_string()),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, "unknown error"),
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
