extern crate base64;
use crate::opts::*;
use crate::{Docker, Msg};
use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str;
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContainerJson {
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
impl ContainerJson {
    pub fn id(&self) -> &str {
        &self.Id
    }
    pub fn names(&self) -> &Vec<String> {
        &self.Names
    }
    pub fn image(&self) -> &str {
        &self.Image
    }
    pub fn image_id(&self) -> &str {
        &self.ImageID
    }
    pub fn command(&self) -> &str {
        &self.Command
    }
    pub fn created(&self) -> u32 {
        self.Created
    }
    pub fn state(&self) -> &str {
        &self.State
    }
    pub fn status(&self) -> &str {
        &self.Status
    }
    pub fn ports(&self) -> &Vec<Value> {
        &self.Ports
    }
    pub fn labels(&self) -> &Value {
        &self.Labels
    }
    pub fn host_config(&self) -> &Value {
        &self.HostConfig
    }
    pub fn network_settings(&self) -> &Value {
        &self.NetworkSettings
    }
    pub fn mounts(&self) -> &Vec<Value> {
        &self.Mounts
    }
}

macro_rules! post_container {
    ($api:expr, $d:ident) => {{
        let res = $d
            .docker
            .client
            .post($d.docker.url.join($api)?)
            .body("")
            .send()
            .await?;
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
pub struct Container<'d> {
    docker: &'d Docker,
    data: Option<ContainerJson>,
    id: String,
}
impl<'d> Container<'d> {
    pub fn new<S: Into<String>>(docker: &'d Docker, id: S) -> Container<'d> {
        Container {
            docker,
            data: None,
            id: id.into(),
        }
    }
    pub fn data(&self) -> Option<ContainerJson> {
        self.data.clone()
    }
    /// Starts the container
    pub async fn start(&self) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .post(
                self.docker
                    .url
                    .join(&format!("containers/{}/start", self.id))?,
            )
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
    pub async fn stop(&self) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .post(
                self.docker
                    .url
                    .join(&format!("containers/{}/stop", self.id))?,
            )
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
    pub async fn restart(&self) -> Result<(), Error> {
        Ok(post_container!(
            &format!("containers/{}/restart", self.id),
            self
        )?)
    }
    /// Kills the container
    pub async fn kill(&self) -> Result<(), Error> {
        Ok(post_container!(
            &format!("containers/{}/kill", self.id),
            self
        )?)
    }
    /// Unpauses the container
    pub async fn unpause(&self) -> Result<(), Error> {
        Ok(post_container!(
            &format!("containers/{}/unpause", self.id),
            self
        )?)
    }
    /// Pauses the container
    pub async fn pause(&self) -> Result<(), Error> {
        Ok(post_container!(
            &format!("containers/{}/pause", self.id),
            self
        )?)
    }
    /// Rename container
    pub async fn rename(&self, new_name: &str) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .post(
                self.docker
                    .url
                    .join(&format!("containers/{}/rename", self.id))?,
            )
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
    pub async fn remove(&self, opts: RmContainerOpts) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .post(
                self.docker
                    .url
                    .join(&format!("containers/{}/rename", self.id))?,
            )
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
    pub async fn logs(&self, opts: ContainerLogsOpts) -> Result<String, Error> {
        let res = self
            .docker
            .client
            .get(
                self.docker
                    .url
                    .join(&format!("containers/{}/logs", self.id))?,
            )
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
    pub async fn archive_path<P: AsRef<Path>>(&self, p: P) -> Result<Url, Error> {
        let res = self
            .docker
            .client
            .get(
                self.docker
                    .url
                    .join(&format!("containers/{}/archive", self.id))?,
            )
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
    /// Upload a tar archive to be extracted to a path in the filesystem of container id.
    /// The input file must be a tar archive compressed with one of the following algorithms: identity (no compression), gzip, bzip2, xz.
    pub async fn upload_archive<P: AsRef<Path>>(
        &self,
        path_to_archive: P,
        opts: UploadArchiveOpts,
    ) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .get(
                self.docker
                    .url
                    .join(&format!("containers/{}/archive", self.id))?,
            )
            .query(&opts.to_query())
            // #TODO
            // This is not working
            .body(fs::read_to_string(path_to_archive)?)
            .send()
            .await?;
        let status = res.status().as_u16();
        match status {
            200 => Ok(()),
            400 => Err(format_err!("bad parameter")),
            403 => Err(format_err!(
                "permission denied, the volume or container rootfs is marked as read-only"
            )),
            404 => Err(format_err!("container or path does not exist")),
            500 => Err(format_err!("server error")),
            _ => {
                let m: Msg = serde_json::from_str(&res.text().await?)?;
                Err(format_err!("{}", m.msg()))
            }
        }
    }
    /// Get information about files in a container
    /// A response header X-Docker-Container-Path-Stat is return containing a base64 - encoded JSON object with some filesystem header information about the path.
    pub async fn file_info<P: AsRef<Path>>(&self, path: P) -> Result<FileInfo, Error> {
        let res = self
            .docker
            .client
            .head(
                self.docker
                    .url
                    .join(&format!("containers/{}/archive", self.id))?,
            )
            .query(&[("path", path.as_ref().to_str())])
            .send()
            .await?;
        let status = res.status().as_u16();
        match status {
            200 => match res.headers().get("X-Docker-Container-Path-Stat") {
                Some(data) => {
                    let data = base64::decode(data)?;
                    let file_info: FileInfo = serde_json::from_str(str::from_utf8(&data)?)?;
                    Ok(file_info)
                }
                None => Err(format_err!(
                    "could not parse FileInfo from base64 encoded header"
                )),
            },
            400 => Err(format_err!("bad parameter")),
            403 => Err(format_err!(
                "permission denied, the volume or container rootfs is marked as read-only"
            )),
            404 => Err(format_err!("container or path does not exist")),
            500 => Err(format_err!("server error")),
            _ => {
                let m: Msg = serde_json::from_str(&res.text().await?)?;
                Err(format_err!("{}", m.msg()))
            }
        }
    }
    /// Create a container
    pub async fn create(&self, name: &str, opts: ContainerBuilderOpts) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .post(self.docker.url.join("containers/create")?)
            .query(&[("name", name)])
            .header("Content-type", "application/json")
            .json(&opts.opts)
            .send()
            .await?;
        let status = res.status().as_u16();
        println!("{:?}", res);
        match status {
            201 => Ok(()),
            400 => Err(format_err!("bad parameter")),
            404 => Err(format_err!("no such container")),
            _ => {
                let m: Msg = serde_json::from_str(&res.text().await?)?;
                Err(format_err!("{}", m.msg()))
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    name: String,
    size: usize,
    mode: usize,
    mtime: String,
    linkTarget: String,
}

/// Api wrapper for containers
pub struct Containers<'d> {
    docker: &'d Docker,
}
impl<'d> Containers<'d> {
    /// new API interface for containers
    pub fn new(docker: &'d Docker) -> Containers {
        Containers { docker }
    }
    pub async fn list(&self) -> Result<Vec<Container<'_>>, Error> {
        let res = self
            .docker
            .client
            .get(self.docker.url.join("containers/json")?)
            .send()
            .await?;
        let docker = self.docker;
        let data: Vec<ContainerJson> = serde_json::from_str(&res.text().await?)?;
        Ok(data
            .iter()
            .map(|c| Container {
                docker,
                data: Some(c.clone()),
                id: c.Id.clone(),
            })
            .collect())
    }
}
