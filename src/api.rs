//!API interfaces for docker subparts like Containers, Images or Networks.
//!
//! ## Example
//! - Spawn a handle directly from docker instance
//! ```ignore
//! use failure::Error;
//! use wharf::Docker;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error>{
//!     let d = Docker::new("http://0.0.0.0:1337")?;
//!     // Containers
//!     let containers = d.containers();
//!     // Images
//!     let images = d.images();
//!     // Networks
//!     let networks = d.networks();
//!     // Specific container
//!     let container = d.container("container-id-or-name");
//!
//! }
//! ```
extern crate base64;
use crate::opts::*;
use crate::result::*;
use crate::{Docker, Msg};
use failure::Error;
use log::*;
use serde_json::{json, Value};
use std::path::Path;
use std::str;
use url::Url;

macro_rules! err_msg {
    ($t: ident, $e: expr) => {
        match serde_json::from_str::<Msg>(&$t) {
            Ok(m) => Err(format_err!("{} - {}", $e, m.msg())),
            _ => Err(format_err!("{}", $e)),
        }
    };
}

// * Containers start *

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

/// Api wrapper for a single container
/// # Example
/// ```ignore
/// let d = Docker::new("0.0.0.0:1234")?;
///
/// let mut container = d.container("boring_johnny");
/// container.rename("new_name").await?;
/// container.start().await?;
///
/// // Get details about a container
/// let container_data = container.inspect().await?;
/// // List processes
/// for process in container.ps("afx").await? {
///     println!("{:?}", process);
/// }
/// // Get info about `/etc` file or directory
/// println!("{:?}", container.file_info("/etc").await?);
///
/// container.stop().await?;
/// ```
#[derive(Debug)]
pub struct Container<'d> {
    docker: &'d Docker,
    id: String,
}
impl<'d> Container<'d> {
    /// new API interface for containers
    pub fn new<S: Into<String>>(docker: &'d Docker, id: S) -> Container<'d> {
        Container {
            docker,
            id: id.into(),
        }
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
        let status = res.status().as_u16();
        let text = res.text().await?;
        match status {
            204 => Ok(()),
            304 => err_msg!(text, "container already started"),
            404 => err_msg!(text, "no such container"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
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
        let status = res.status().as_u16();
        let text = res.text().await?;
        match status {
            204 => Ok(()),
            304 => err_msg!(text, "container already stopped"),
            404 => err_msg!(text, "no such container"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Inspect a container
    /// Return low-level information about a container.
    pub async fn inspect(&self) -> Result<ContainerInspect, Error> {
        let res = self
            .docker
            .client
            .get(
                self.docker
                    .url
                    .join(&format!("containers/{}/json", self.id))?,
            )
            .send()
            .await?;
        let status = res.status().as_u16();
        let text = res.text().await?;
        match status {
            200 => {
                let data: ContainerInspect = serde_json::from_str(&text)?;
                Ok(data)
            }
            404 => err_msg!(text, "no such container"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
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
    pub async fn rename(&mut self, new_name: &str) -> Result<(), Error> {
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
        debug!("{:?}", res);
        let status = res.status().as_u16();
        let text = res.text().await?;
        debug!("{}", text);
        match status {
            204 => {
                self.id = new_name.to_string();
                Ok(())
            }
            404 => err_msg!(text, "no such container"),
            409 => err_msg!(text, "name already in use"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Remove a container
    pub async fn remove(&self, opts: &RmContainerOpts) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .post(
                self.docker
                    .url
                    .join(&format!("containers/{}/rename", self.id))?,
            )
            .query(opts.opts())
            .send()
            .await?;
        let status = res.status().as_u16();
        let text = res.text().await?;
        match status {
            204 => Ok(()),
            404 => err_msg!(text, "no such container"),
            409 => err_msg!(text, "name already in use"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Work in progress...
    pub async fn logs(&self, opts: &ContainerLogsOpts) -> Result<String, Error> {
        let res = self
            .docker
            .client
            .get(
                self.docker
                    .url
                    .join(&format!("containers/{}/logs", self.id))?,
            )
            .query(opts.opts())
            .send()
            .await?;
        let status = res.status().as_u16();
        let text = res.text().await?;
        match status {
            200 => Ok(text),
            404 => err_msg!(text, "no such container"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
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
        let url = res.url().clone();
        let text = res.text().await?;
        match status {
            200 => Ok(url),
            400 => err_msg!(text, "container or path does not exist"),
            404 => err_msg!(text, "no such container"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Upload a tar archive to be extracted to a path in the filesystem of container id.  
    /// The input file must be a tar archive compressed with one of the following algorithms: identity (no compression), gzip, bzip2, xz.
    pub async fn upload_archive(
        &self,
        archive: &[u8],
        opts: &UploadArchiveOpts,
    ) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .put(
                self.docker
                    .url
                    .join(&format!("containers/{}/archive", self.id))?,
            )
            .query(opts.opts())
            .body(archive.to_vec())
            .send()
            .await?;
        let status = res.status().as_u16();
        let text = res.text().await?;
        match status {
            200 => Ok(()),
            400 => err_msg!(text, "container or path does not exist"),
            403 => err_msg!(
                text,
                "permission denied, the volume or container rootfs is marked as read-only"
            ),
            404 => err_msg!(text, "no such container"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
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
            other => {
                let text = res.text().await?;
                match other {
                    400 => err_msg!(text, "bad parameter"),
                    404 => err_msg!(text, "container or path does not exist"),
                    500 => err_msg!(text, "server error"),
                    _ => err_msg!(text, ""),
                }
            }
        }
    }
    /// List processes running inside a container  
    /// On Unix systems, this is done by running the ps command. This endpoint is not supported on Windows.
    pub async fn ps<S: AsRef<str>>(&self, ps_args: S) -> Result<Vec<Process>, Error> {
        let res = self
            .docker
            .client
            .get(
                self.docker
                    .url
                    .join(&format!("containers/{}/top", self.id))?,
            )
            .query(&[("ps_args", ps_args.as_ref())])
            .send()
            .await?;
        let status = res.status().as_u16();
        let text = res.text().await?;
        match status {
            200 => {
                let data: ContainerProcessesJson = serde_json::from_str(&text)?;
                Ok(data
                    .Processes
                    .iter()
                    .map(|p| Process::new(&data.Titles, &p))
                    .collect())
            }
            404 => err_msg!(text, "no such container"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Attach to a container
    pub async fn attach(&self) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .post(
                self.docker
                    .url
                    .join(&format!("containers/{}/attach", self.id))?,
            )
            .header("Connection", "Upgrade")
            .header("Upgrade", "tcp")
            .send()
            .await?;
        let status = res.status().as_u16();
        let text = res.text().await?;
        match status {
            101 => {
                // The response body is a stream
                // #TODO
                // implement a stream reader for:
                // - https://docs.docker.com/engine/api/v1.40/#operation/ContainerAttach
                Ok(())
            }
            200 => Ok(()),
            404 => err_msg!(text, "no such container"),
            409 => err_msg!(text, "name already in use"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Exec a command
    pub async fn exec(&self, opts: &ExecOpts) -> Result<String, Error> {
        let exec_id = self.create_exec_instance(opts).await?;
        self.start_exec_instance(exec_id.trim_matches('"'), opts._detach(), opts._tty())
            .await
    }
    // Starts the exec instance
    async fn start_exec_instance(
        &self,
        id: &str,
        detach: bool,
        tty: bool,
    ) -> Result<String, Error> {
        let res = self
            .docker
            .client
            .post(self.docker.url.join(&format!("exec/{}/start", id))?)
            .json(&json!({"Detach": detach, "Tty": tty}))
            .send()
            .await?;
        debug!("{:?}", res);
        let status = res.status().as_u16();
        let text = res.text().await?;
        debug!("{}", text);
        match status {
            200 => Ok(text),
            404 => err_msg!(text, "no such exec instance"),
            409 => err_msg!(text, "container is paused"),
            _ => err_msg!(text, ""),
        }
    }
    // Returns Id of exec instance
    async fn create_exec_instance(&self, opts: &ExecOpts) -> Result<String, Error> {
        let res = self
            .docker
            .client
            .post(
                self.docker
                    .url
                    .join(&format!("containers/{}/exec", self.id))?,
            )
            .json(opts.opts())
            .send()
            .await?;
        debug!("{:?}", res);
        let status = res.status().as_u16();
        let text = res.text().await?;
        debug!("{}", text);
        match status {
            201 => match serde_json::from_str::<Value>(&text)?.get("Id") {
                Some(id) => Ok(id.to_string()),
                _ => Err(format_err!("there was no field Id in the response body.")),
            },
            404 => err_msg!(text, "no such container"),
            409 => err_msg!(text, "container is paused"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
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
    /// List all containers
    pub async fn list(&self, opts: &ListContainersOpts) -> Result<Vec<Container<'_>>, Error> {
        let res = self
            .docker
            .client
            .get(self.docker.url.join("containers/json")?)
            .query(opts.opts())
            .send()
            .await?;
        let docker = self.docker;
        debug!("{:?}", res);
        let text = res.text().await?;
        debug!("{}", text);
        let data: Vec<ContainerData> = serde_json::from_str(&text)?;
        debug!("{:?}", data);
        Ok(data
            .iter()
            .map(|c| Container {
                docker,
                id: c.id.clone(),
            })
            .collect())
    }
    /// Create a container
    pub async fn create(&self, name: &str, opts: &ContainerBuilderOpts) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .post(self.docker.url.join("containers/create")?)
            .query(&[("name", name)])
            .header("Content-type", "application/json")
            .json(opts.opts())
            .send()
            .await?;
        let status = res.status().as_u16();
        let text = res.text().await?;
        match status {
            201 => Ok(()),
            400 => err_msg!(text, "bad parameter"),
            404 => err_msg!(text, "no such container"),
            409 => err_msg!(text, "conflict"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
}
// * Containers end *

// * Networks start *

/// Api wrapper for networks
pub struct Networks<'d> {
    docker: &'d Docker,
}
impl<'d> Networks<'d> {
    /// new API interface for networks
    pub fn new(docker: &'d Docker) -> Networks {
        Networks { docker }
    }
    /// List all networks
    pub async fn list(&self) -> Result<Vec<NetworkData>, Error> {
        let res = self
            .docker
            .client
            .get(self.docker.url.join("networks")?)
            .send()
            .await?;
        let body = res.text().await?;
        Ok(serde_json::from_str(&body)?)
    }
    ///Remove a network
    pub async fn remove(&self, id: &str) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .delete(self.docker.url.join(&format!("networks/{}", id))?)
            .send()
            .await?;
        let status = res.status().as_u16();
        let text = res.text().await?;
        match status {
            204 => Ok(()),
            403 => err_msg!(text, "operation not supported for pre-defined networks"),
            404 => err_msg!(text, "no such network"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
}
// * Networks end *

// * Images start *

/// Api wrapper for images
pub struct Images<'d> {
    docker: &'d Docker,
}
impl<'d> Images<'d> {
    /// new API interface for images
    pub fn new(docker: &'d Docker) -> Self {
        Images { docker }
    }
    /// List all images
    pub async fn list(&self) -> Result<Vec<ImageData>, Error> {
        // FIXME later
        let res = self
            .docker
            .client
            .get(self.docker.url.join("images/json")?)
            .send()
            .await?;
        let body = res.text().await?;
        Ok(serde_json::from_str(&body)?)
    }
    /// Pulls an image from registry  
    /// WARNING!  
    /// not specyfying tag will pull all tags of image
    pub async fn pull(&self, image: &str, tag: &str, auth: &AuthOpts) -> Result<(), Error> {
        let mut opts = CreateImageOpts::new();
        opts.from_image(image).tag(tag).set_auth(&auth);
        self.create(&opts).await
    }
    /// Create an image by either pulling it from a registry or importing it.
    pub async fn create(&self, opts: &CreateImageOpts) -> Result<(), Error> {
        let mut req = self
            .docker
            .client
            .post(self.docker.url.join("images/create")?);
        // if we're pulling from registry we need to authenticate
        if opts.opts().get("fromImage").is_some() {
            req = req.header("X-Registry-Auth", opts.auth_ref().serialize()?);
        }
        req = req.query(opts.opts());
        //req = req.query(&[("fromImage", "ubuntu")]);
        debug!("{:?}", req);
        let res = req.send().await?;
        let status = res.status().as_u16();
        debug!("{:?}", res);
        let text = res.text().await?;
        debug!("{}", text);
        match status {
            200 => Ok(()),
            404 => err_msg!(text, "repository does not exist or no read access"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Remove an image
    pub async fn remove(&self, image: &str, force: bool, no_prune: bool) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .delete(self.docker.url.join(&format!("images/{}", image))?)
            .query(&[("force", force), ("noprune", no_prune)])
            .send()
            .await?;
        debug!("{:?}", res);
        let status = res.status().as_u16();
        let text = res.text().await?;
        debug!("{}", text);
        match status {
            200 => Ok(()),
            404 => err_msg!(text, "no such image"),
            409 => err_msg!(text, "conflict"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Import images  
    /// Load a set of images and tags into a repository.
    pub async fn import(&self, archive: &[u8]) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .post(self.docker.url.join("images/load")?)
            .body(archive.to_vec())
            .send()
            .await?;
        debug!("{:?}", res);
        let status = res.status().as_u16();
        let text = res.text().await?;
        debug!("{}", text);
        match status {
            200 => Ok(()),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Tag an image so that it becomes part of a repository.  
    /// **image** - name or id of image in the form: *someimage:sometag*  
    /// **repo** - The repository to tag in. For example, *someuser/someimage*  
    /// **tag** - The name of the new tag.
    pub async fn tag(&self, image: &str, repo: &str, tag: &str) -> Result<(), Error> {
        let res = self
            .docker
            .client
            .post(self.docker.url.join(&format!("images/{}/tag", image))?)
            .query(&json!({"repo": repo, "tag": tag}))
            .send()
            .await?;
        debug!("{:?}", res);
        let status = res.status().as_u16();
        let text = res.text().await?;
        debug!("{}", text);
        match status {
            201 => Ok(()),
            400 => err_msg!(text, "bad parameter"),
            404 => err_msg!(text, "no such image"),
            409 => err_msg!(text, "conflict"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Inspect an image  
    /// Return low-level information about an image.
    pub async fn inspect(&self, image: &str) -> Result<ImageInspect, Error> {
        let res = self
            .docker
            .client
            .get(self.docker.url.join(&format!("images/{}/json", image))?)
            .send()
            .await?;
        debug!("{:?}", res);
        let status = res.status().as_u16();
        let text = res.text().await?;
        debug!("{}", text);
        match status {
            200 => Ok(serde_json::from_str(&text)?),
            404 => err_msg!(text, "no such image"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
}
// * Images End *
