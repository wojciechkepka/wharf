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
use hyper::{body::to_bytes, Body, Method};
use log::*;
use serde_json::Value;
use std::path::Path;
use std::str;
macro_rules! err_msg {
    ($t: ident, $e: expr) => {
        match serde_json::from_slice::<Msg>($t.as_ref()) {
            Ok(m) => Err(format_err!("{} - {}", $e, m.msg())),
            _ => Err(format_err!("{}", $e)),
        }
    };
}
macro_rules! post_container {
    ($e: expr, $s: ident) => {{
        let res = $s
            .docker
            .req(Method::POST, $e, None, Body::from(""), None)
            .await?;

        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);

        match status {
            204 => Ok(()),
            404 => err_msg!(text, "no such container"),
            500 => err_msg!(text, "internal server error"),
            _ => err_msg!(text, ""),
        }
    }};
}

// * Containers start *

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
            .req(
                Method::POST,
                format!("/containers/{}/start", self.id),
                None,
                Body::from(""),
                None,
            )
            .await?;

        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
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
            .req(
                Method::POST,
                format!("/containers/{}/stop", self.id),
                None,
                Body::from(""),
                None,
            )
            .await?;

        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
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
            .req(
                Method::GET,
                format!("/containers/{}/json", self.id),
                None,
                Body::from(""),
                None,
            )
            .await?;

        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
        match status {
            200 => {
                let data: ContainerInspect = serde_json::from_slice(&text)?;
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
            format!("/containers/{}/restart", self.id),
            self
        )?)
    }
    /// Kills the container
    pub async fn kill(&self) -> Result<(), Error> {
        Ok(post_container!(
            format!("/containers/{}/kill", self.id),
            self
        )?)
    }
    /// Unpauses the container
    pub async fn unpause(&self) -> Result<(), Error> {
        Ok(post_container!(
            format!("/containers/{}/unpause", self.id),
            self
        )?)
    }
    /// Pauses the container
    pub async fn pause(&self) -> Result<(), Error> {
        Ok(post_container!(
            format!("/containers/{}/pause", self.id),
            self
        )?)
    }
    /// Rename container
    pub async fn rename(&mut self, new_name: &str) -> Result<(), Error> {
        let res = self
            .docker
            .req(
                Method::POST,
                format!("/containers/{}/rename", self.id),
                Some(format!("name={}", new_name)),
                Body::from(""),
                None,
            )
            .await?;

        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
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
            .req(
                Method::DELETE,
                format!("/containers/{}", self.id),
                Some(opts.to_query()?),
                Body::from(""),
                None,
            )
            .await?;

        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
        match status {
            204 => Ok(()),
            404 => err_msg!(text, "no such container"),
            409 => err_msg!(text, "name already in use"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Work in progress...
    pub async fn logs(&self, _opts: &ContainerLogsOpts) -> Result<String, Error> {
        unimplemented!()
    }
    /// Get a tar archive of a resource in the filesystem of container id  
    /// Returns a tar archived path
    pub async fn archive_path<P: AsRef<Path>>(&self, p: P) -> Result<Vec<u8>, Error> {
        let res = self
            .docker
            .req(
                Method::GET,
                format!("/containers/{}/archive", self.id),
                Some(format!("path={}", p.as_ref().to_str().unwrap())),
                Body::from(""),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        match status {
            200 => Ok(text.to_vec()),
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
            .req(
                Method::PUT,
                format!("/containers/{}/archive", self.id),
                Some(opts.to_query()?),
                Body::from(archive.to_vec()),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
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
            .req(
                Method::HEAD,
                format!("/containers/{}/archive", self.id),
                Some(format!("path={}", path.as_ref().to_str().unwrap())),
                Body::from(""),
                None,
            )
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
                let text = to_bytes(res.into_body()).await?;
                trace!("{}", str::from_utf8(&text)?);
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
            .req(
                Method::GET,
                format!("/containers/{}/top", self.id),
                Some(format!("ps_args={}", ps_args.as_ref())),
                Body::from(""),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
        match status {
            200 => {
                let data: ContainerProcessesJson = serde_json::from_slice(&text)?;
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
    pub async fn attach(&self, opts: &AttachOpts) -> Result<hyper::upgrade::Upgraded, Error> {
        let res = self
            .docker
            .req(
                Method::POST,
                format!("/containers/{}/attach", self.id),
                Some(opts.to_query()?),
                Body::from(""),
                Some(vec![
                    ("Connection", "Upgrade".into()),
                    ("Upgrade", "tcp".into()),
                ]),
            )
            .await?;
        let status = res.status().as_u16();
        match status {
            101 => match res.into_body().on_upgrade().await {
                Ok(upgraded) => Ok(upgraded),
                Err(e) => Err(format_err!("connection upgrade failed - {:?}", e)),
            },
            other => {
                let text = to_bytes(res.into_body()).await?;
                trace!("{}", str::from_utf8(&text)?);
                match other {
                    400 => err_msg!(text, "bad parameter"),
                    404 => err_msg!(text, "no such container"),
                    500 => err_msg!(text, "server error"),
                    _ => err_msg!(text, ""),
                }
            }
        }
    }
    /// Exec a command
    pub async fn exec(&self, opts: &ExecOpts) -> Result<String, Error> {
        let exec_id = self.create_exec_instance(opts).await?;
        self.start_exec_instance(exec_id.trim_matches('"'), opts)
            .await
    }
    // Starts the exec instance
    #[allow(dead_code)]
    async fn start_exec_instance(&self, id: &str, opts: &ExecOpts) -> Result<String, Error> {
        let res = self
            .docker
            .req(
                Method::POST,
                format!("/exec/{}/start", id),
                None,
                Body::from(serde_json::to_vec(opts.opts())?),
                Some(vec![("Content-type", "application/json".into())]),
            )
            .await?;

        let status = res.status().as_u16();
        match status {
            200 => Ok(str::from_utf8(to_bytes(res.into_body()).await?.as_ref())?.to_string()),
            other => {
                let text = to_bytes(res.into_body()).await?;
                trace!("{}", str::from_utf8(&text)?);
                match other {
                    404 => err_msg!(text, "no such exec instance"),
                    409 => err_msg!(text, "container is paused"),
                    _ => err_msg!(text, ""),
                }
            }
        }
    }
    // Returns Id of exec instance
    async fn create_exec_instance(&self, opts: &ExecOpts) -> Result<String, Error> {
        let res = self
            .docker
            .req(
                Method::POST,
                format!("/containers/{}/exec", self.id),
                None,
                Body::from(serde_json::to_vec(opts.opts())?),
                Some(vec![("Content-type", "application/json".into())]),
            )
            .await?;

        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
        match status {
            201 => match serde_json::from_slice::<Value>(&text)?.get("Id") {
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
            .req(
                Method::GET,
                "/containers/json".into(),
                Some(opts.to_query()?),
                Body::from(""),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);

        match status {
            200 => {
                let data: Vec<ContainerData> = serde_json::from_slice(&text)?;
                debug!("{:?}", data);
                let docker = self.docker;
                Ok(data
                    .iter()
                    .map(|c| Container {
                        docker,
                        id: c.id.clone(),
                    })
                    .collect())
            }
            400 => err_msg!(text, "bad parameter"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Create a container
    pub async fn create(&self, name: &str, opts: &ContainerBuilderOpts) -> Result<(), Error> {
        let res = self
            .docker
            .req(
                Method::POST,
                "/containers/create".into(),
                Some(format!("name={}", name)),
                Body::from(serde_json::to_string(opts.opts())?),
                Some(vec![("Content-type", "application/json".into())]),
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
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
        // TODO add universal ListOpts for all listing methods
        let res = self
            .docker
            .req(Method::GET, "/networks".into(), None, Body::from(""), None)
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);

        match status {
            200 => Ok(serde_json::from_slice(&text)?),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    ///Remove a network
    pub async fn remove(&self, id: &str) -> Result<(), Error> {
        let res = self
            .docker
            .req(
                Method::GET,
                format!("/networks/{}", id),
                None,
                Body::from(""),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);

        match status {
            200 => Ok(serde_json::from_slice(&text)?),
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
        let res = self
            .docker
            .req(
                Method::GET,
                "/images/json".into(),
                None,
                Body::from(""),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);

        match status {
            200 => Ok(serde_json::from_slice(&text)?),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
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
        let mut headers = Vec::new();
        if opts.opts().get("fromImage").is_some() {
            headers.push(("X-Registry-Auth", opts.auth_ref().serialize()?));
        }
        let res = self
            .docker
            .req(
                Method::POST,
                "/images/create".into(),
                None,
                Body::from(""),
                Some(headers),
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);

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
            .req(
                Method::DELETE,
                format!("/images/{}", image),
                Some(format!("force={}&no_prune={}", force, no_prune)),
                Body::from(""),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);

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
            .req(
                Method::POST,
                "/images/load".into(),
                None,
                Body::from(archive.to_vec()),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
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
            .req(
                Method::POST,
                format!("/images/{}/tag", image),
                Some(format!("repo={}&tag={}", repo, tag)),
                Body::from(""),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
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
            .req(
                Method::GET,
                format!("/images/{}/json", image),
                None,
                Body::from(""),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
        match status {
            200 => Ok(serde_json::from_slice(&text)?),
            404 => err_msg!(text, "no such image"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Get the history of an image  
    /// Return parent layers of an image
    pub async fn history(&self, image: &str) -> Result<Vec<ImageHistory>, Error> {
        let res = self
            .docker
            .req(
                Method::GET,
                format!("/images/{}/history", image),
                None,
                Body::from(""),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
        match status {
            200 => Ok(serde_json::from_slice(&text)?),
            404 => err_msg!(text, "no such image"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Search for images on Docker Hub
    pub async fn search(
        &self,
        term: &str,
        limit: u64,
        filters: String,
    ) -> Result<Vec<ImageMatch>, Error> {
        let res = self
            .docker
            .req(
                Method::GET,
                "/images/search".into(),
                Some(format!("term={}&limit={}&filters={}", term, limit, filters)),
                Body::from(""),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
        match status {
            200 => Ok(serde_json::from_slice(&text)?),
            404 => err_msg!(text, "no such image"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Delete unused images
    pub async fn prune(&self, filters: &str) -> Result<ImagesDeleted, Error> {
        let res = self
            .docker
            .req(
                Method::POST,
                "/images/prune".into(),
                Some(format!("filters={}", filters)),
                Body::from(""),
                None,
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
        match status {
            200 => Ok(serde_json::from_slice(&text).unwrap_or_default()),
            404 => err_msg!(text, "no such image"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
    /// Build an image from a tar archive with a Dockerfile in it.
    ///The Dockerfile specifies how the image is built from the tar archive. It is typically in the archive's root, but can be at a different path or have a different name by specifying the dockerfile parameter. See the Dockerfile reference for more information.
    //The Docker daemon performs a preliminary validation of the Dockerfile before starting the build, and returns an error if the syntax is incorrect. After that, each instruction is run one-by-one until the ID of the new image is output.
    pub async fn build(&self, opts: &ImageBuilderOpts) -> Result<(), Error> {
        let res = self
            .docker
            .req(
                Method::POST,
                "/build".into(),
                Some(opts.to_query()?),
                Body::from(""),
                Some(vec![("Content-type", "application/x-tar".into())]),
            )
            .await?;
        let status = res.status().as_u16();
        let text = to_bytes(res.into_body()).await?;
        trace!("{}", str::from_utf8(&text)?);
        match status {
            200 => Ok(serde_json::from_slice(&text).unwrap_or_default()),
            404 => err_msg!(text, "no such image"),
            500 => err_msg!(text, "server error"),
            _ => err_msg!(text, ""),
        }
    }
}
// * Images End *
