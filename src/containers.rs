use crate::opts::*;
use url::Url;
use failure::Error;
use crate::{Docker, Msg};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

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
macro_rules! post_container {
    ($api:expr, $d:ident) => {{
        let client = reqwest::Client::new();
        let res = client.post($d.url.join($api)?).body("").send().await?;
        match res.status().as_u16() {
            204 => Ok(()),
            404 => Err(format_err!("no such container")),
            500 => Err(format_err!("internal server error")),
            _ => {
                let m: Msg = serde_json::from_str(&res.text().await?)?;
                Err(format_err!("{}", m.msg()))
            }
        }
    }};
}
impl Container {
    pub fn id(&self) -> String {
        self.Id.clone()
    }
    pub fn names(&self) -> Vec<String> {
        self.Names.clone()
    }
    pub fn image(&self) -> String {
        self.Image.clone()
    }
    pub fn image_id(&self) -> String {
        self.ImageID.clone()
    }
    pub fn command(&self) -> String {
        self.Command.clone()
    }
    pub fn created(&self) -> u32 {
        self.Created.clone()
    }
    pub fn state(&self) -> String {
        self.State.clone()
    }
    pub fn status(&self) -> String {
        self.Status.clone()
    }
    pub fn ports(&self) -> Vec<Value> {
        self.Ports.clone()
    }
    pub fn labels(&self) -> Value {
        self.Labels.clone()
    }
    pub fn host_config(&self) -> Value {
        self.HostConfig.clone()
    }
    pub fn network_settings(&self) -> Value {
        self.NetworkSettings.clone()
    }
    pub fn mounts(&self) -> Vec<Value> {
        self.Mounts.clone()
    }
    /// Starts the container
    pub async fn start(docker: &Docker, id: &str) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(docker.url.join(&format!("containers/{}/start", id))?)
            .body("")
            .send()
            .await?;
        match res.status().as_u16() {
            204 => Ok(()),
            304 => Err(format_err!("container already started")),
            404 => Err(format_err!("no such container")),
            500 => Err(format_err!("internal server error")),
            _ => {
                let m: Msg = serde_json::from_str(&res.text().await?)?;
                Err(format_err!("{}", m.msg()))
            }
        }
    }
    /// Stops the container
    pub async fn stop(docker: &Docker, id: &str) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(docker.url.join(&format!("containers/{}/stop", id))?)
            .body("")
            .send()
            .await?;
        match res.status().as_u16() {
            204 => Ok(()),
            304 => Err(format_err!("container already stopped")),
            404 => Err(format_err!("no such container")),
            500 => Err(format_err!("internal server error")),
            _ => {
                let m: Msg = serde_json::from_str(&res.text().await?)?;
                Err(format_err!("{}", m.msg()))
            }
        }
    }
    /// Restarts the container
    pub async fn restart(docker: &Docker, id: &str) -> Result<(), Error> {
        Ok(post_container!(
            &format!("containers/{}/restart", id),
            docker
        )?)
    }
    /// Kills the container
    pub async fn kill(docker: &Docker, id: &str) -> Result<(), Error> {
        Ok(post_container!(&format!("containers/{}/kill", id), docker)?)
    }
    /// Unpauses the container
    pub async fn unpause(docker: &Docker, id: &str) -> Result<(), Error> {
        Ok(post_container!(
            &format!("containers/{}/unpause", id),
            docker
        )?)
    }
    /// Pauses the container
    pub async fn pause(docker: &Docker, id: &str) -> Result<(), Error> {
        Ok(post_container!(
            &format!("containers/{}/pause", id),
            docker
        )?)
    }
    /// Rename container
    pub async fn rename(docker: &Docker, id: &str, new_name: &str) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(docker.url.join(&format!("containers/{}/rename", id))?)
            .query(&[("name", new_name)])
            .send()
            .await?;
        let status = res.status().as_u16();
        match status {
            204 => Ok(()),
            404 => Err(format_err!("no such container")),
            409 => Err(format_err!("name already in use")),
            500 => Err(format_err!("internal server error")),
            _ => {
                let m: Msg = serde_json::from_str(&res.text().await?)?;
                Err(format_err!("{}", m.msg()))
            }
        }
    }
    /// Remove a container
    pub async fn remove(
        docker: &Docker,
        id: &str,
        opts: RmContainerOpts,
    ) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(docker.url.join(&format!("containers/{}/rename", id))?)
            .query(&opts.to_query())
            .send()
            .await?;
        let status = res.status().as_u16();
        match status {
            204 => Ok(()),
            404 => Err(format_err!("no such container")),
            409 => Err(format_err!("name already in use")),
            500 => Err(format_err!("internal server error")),
            _ => {
                let m: Msg = serde_json::from_str(&res.text().await?)?;
                Err(format_err!("{}", m.msg()))
            }
        }
    }
    /// Work in progress...
    pub async fn logs(
        docker: &Docker,
        id: &str,
        opts: ContainerLogsOpts,
    ) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(docker.url.join(&format!("containers/{}/logs", id))?)
            .query(&opts.to_query())
            .send()
            .await?;
        match res.status().as_u16() {
            200 => Ok(res.text().await?),
            204 => Ok(res.text().await?),
            404 => Err(format_err!("no such container")),
            500 => Err(format_err!("internal server error")),
            _ => {
                let m: Msg = serde_json::from_str(&res.text().await?)?;
                Err(format_err!("{}", m.msg()))
            }
        }
    }
    /// Get a tar archive of a resource in the filesystem of container id
    /// Returns URL to the archived resource
    pub async fn archive_path<P: AsRef<Path>>(docker: &Docker, id: &str, p: P) -> Result<Url, Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(docker.url.join(&format!("containers/{}/archive", id))?)
            .query(&[("path", p.as_ref())])
            .send()
            .await?;
        let status = res.status().as_u16();
        match status {
            200 => Ok(res.url().clone()),
            400 => Err(format_err!("bad parameter")),
            404 => Err(format_err!("container or path does not exist")),
            500 => Err(format_err!("server error")),
            _ => {
                let m: Msg = serde_json::from_str(&res.text().await?)?;
                Err(format_err!("{}", m.msg()))
            }
        }
    }
}

/// Api wrapper for containers
pub struct Containers {}
impl Containers {
    pub async fn list(docker: &Docker) -> Result<Vec<Container>, Error> {
        let res = reqwest::get(docker.url.join("containers/json")?).await?;
        let body = res.text().await?;
        Ok(serde_json::from_str(&body)?)
    }
}
