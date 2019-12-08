extern crate base64;
use std::str;
use crate::opts::*;
use crate::{Docker, Msg};
use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use url::Url;

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
    pub async fn remove(docker: &Docker, id: &str, opts: RmContainerOpts) -> Result<(), Error> {
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
    pub async fn logs(docker: &Docker, id: &str, opts: ContainerLogsOpts) -> Result<String, Error> {
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
    pub async fn archive_path<P: AsRef<Path>>(
        docker: &Docker,
        id: &str,
        p: P,
    ) -> Result<Url, Error> {
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
    /// Upload a tar archive to be extracted to a path in the filesystem of container id.
    /// The input file must be a tar archive compressed with one of the following algorithms: identity (no compression), gzip, bzip2, xz.
    pub async fn upload_archive<P: AsRef<Path>>(
        docker: &Docker,
        id: &str,
        path_to_archive: P,
        opts: UploadArchiveOpts,
    ) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(docker.url.join(&format!("containers/{}/archive", id))?)
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
    pub async fn file_info<P: AsRef<Path>>(
        docker: &Docker,
        id: &str,
        path: P,
    ) -> Result<FileInfo, Error> {
        let client = reqwest::Client::new();
        let res = client
            .head(docker.url.join(&format!("containers/{}/archive", id))?)
            .query(&[("path", path.as_ref().to_str())])
            // #TODO
            // This is not working
            .send()
            .await?;
        let status = res.status().as_u16();
        match status {
            200 => {
                match res.headers().get("X-Docker-Container-Path-Stat") {
                    Some(data) => {
                        let data = base64::decode(data)?;
                        let file_info: FileInfo = serde_json::from_str(str::from_utf8(&data)?)?;
                        Ok(file_info)
                    }
                    None => Err(format_err!("could not parse FileInfo from base64 encoded header")),
                }
            }
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

}
#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    name: String,
    size: usize,
    mode: usize,
    mtime: String,
    linkTarget: String
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
