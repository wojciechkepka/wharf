#![allow(non_snake_case)]
#[macro_use] extern crate failure;
use std::collections::HashMap;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    Id: String,
    Names: Vec<String>,
    Image: String,
    ImageID: String,
    Command: String,
    Created: u32,
    State: String,
    Status: String,
    Ports: Vec<Value>,
    Labels: Value,
    HostConfig: Value,
    NetworkSettings: Value,
    Mounts: Vec<Value>,
}
impl Container {
    /// Starts the container
    pub async fn start(docker: &Docker, id: String) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let mut res = client.post(docker.url.join(&format!("containers/{}/start", id))?).body("").send().await?;
        match res.status().as_u16() {
            204 => Ok(()),
            304 => Err(format_err!("container already started")),
            404 => Err(format_err!("no such container")),
            500 => Err(format_err!("internal server error")),
            _ => Err(format_err!("unknown error")),
        }
    }
    /// Stops the container
    pub async fn stop(docker: &Docker, id: String) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let mut res = client.post(docker.url.join(&format!("containers/{}/stop", id))?).body("").send().await?;
        match res.status().as_u16() {
            204 => Ok(()),
            304 => Err(format_err!("container already stopped")),
            404 => Err(format_err!("no such container")),
            500 => Err(format_err!("internal server error")),
            _ => Err(format_err!("unknown error")),
        }
    }
    /// Restarts the container
    pub async fn restart(docker: &Docker, id: String) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let mut res = client.post(docker.url.join(&format!("containers/{}/restart", id))?).body("").send().await?;
        match res.status().as_u16() {
            204 => Ok(()),
            404 => Err(format_err!("no such container")),
            500 => Err(format_err!("internal server error")),
            _ => Err(format_err!("unknown error")),
        }
    }
    /// Kills the container
    pub async fn kill(docker: &Docker, id: String) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let mut res = client.post(docker.url.join(&format!("containers/{}/kill", id))?).body("").send().await?;
        match res.status().as_u16() {
            204 => Ok(()),
            404 => Err(format_err!("no such container")),
            500 => Err(format_err!("internal server error")),
            _ => Err(format_err!("unknown error")),
        }
    }
    /// Work in progress...
    pub async fn logs(docker: &Docker, id: String, params: LogsQueryParameters) -> Result<String, Error> {
        let client = reqwest::Client::new();
        
        let mut res = client.get(docker.url.join(&format!("containers/{}/logs", id))?).body(serde_json::to_string(&params)?).send().await?;
        match res.status().as_u16() {
            204 => Ok(res.text().await?),
            404 => Err(format_err!("no such container")),
            500 => Err(format_err!("internal server error")),
            _ => Err(format_err!("unknown error")),
        }
    }
}
#[derive(Default, Serialize, Deserialize)]
pub struct LogsQueryParameters {
    follow: bool,
    pub stdout: bool,
    stderr: bool,
    since: u32,
    until: u32,
    timestamps: bool,
    tail: String,
}

/// Api wrapper for containers
pub struct Containers {}
impl Containers {
    pub async fn list(docker: &Docker) -> Result<Vec<Container> , Error> {
        let mut res = reqwest::get(docker.url.join("containers/json")?).await?;
        let body = res.text().await?;
        Ok(serde_json::from_str(&body)?)
    }
}

pub struct Images {}
impl Images {
    pub async fn list(docker: &Docker) -> Result<Vec<Image> , Error> {
        let mut res = reqwest::get(docker.url.join("images/json")?).await?;
        let body = res.text().await?;
        Ok(serde_json::from_str(&body)?)
    }
}

pub struct Networks {}
impl Networks {
    pub async fn list(docker: &Docker) -> Result<Vec<Network> , Error> {
        let mut res = reqwest::get(docker.url.join("networks")?).await?;
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
