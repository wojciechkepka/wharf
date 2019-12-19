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
        unimplemented!()
    }
    /// Stops the container
    pub async fn stop(&self) -> Result<(), Error> {
        unimplemented!()
    }
    /// Inspect a container
    /// Return low-level information about a container.
    pub async fn inspect(&self) -> Result<ContainerInspect, Error> {
        unimplemented!()
    }
    /// Restarts the container
    pub async fn restart(&self) -> Result<(), Error> {
        unimplemented!()
    }
    /// Kills the container
    pub async fn kill(&self) -> Result<(), Error> {
        unimplemented!()
    }
    /// Unpauses the container
    pub async fn unpause(&self) -> Result<(), Error> {
        unimplemented!()
    }
    /// Pauses the container
    pub async fn pause(&self) -> Result<(), Error> {
        unimplemented!()
    }
    /// Rename container
    pub async fn rename(&mut self, new_name: &str) -> Result<(), Error> {
        unimplemented!()
    }
    /// Remove a container
    pub async fn remove(&self, opts: &RmContainerOpts) -> Result<(), Error> {
        unimplemented!()
    }
    /// Work in progress...
    pub async fn logs(&self, opts: &ContainerLogsOpts) -> Result<String, Error> {
        unimplemented!()
    }
    /// Get a tar archive of a resource in the filesystem of container id  
    /// Returns URL to the archived resource
    pub async fn archive_path<P: AsRef<Path>>(&self, p: P) -> Result<String, Error> {
        unimplemented!()
    }
    /// Upload a tar archive to be extracted to a path in the filesystem of container id.  
    /// The input file must be a tar archive compressed with one of the following algorithms: identity (no compression), gzip, bzip2, xz.
    pub async fn upload_archive(
        &self,
        archive: &[u8],
        opts: &UploadArchiveOpts,
    ) -> Result<(), Error> {
        unimplemented!()
    }
    /// Get information about files in a container  
    /// A response header X-Docker-Container-Path-Stat is return containing a base64 - encoded JSON object with some filesystem header information about the path.
    pub async fn file_info<P: AsRef<Path>>(&self, path: P) -> Result<FileInfo, Error> {
        unimplemented!()
    }
    /// List processes running inside a container  
    /// On Unix systems, this is done by running the ps command. This endpoint is not supported on Windows.
    pub async fn ps<S: AsRef<str>>(&self, ps_args: S) -> Result<Vec<Process>, Error> {
        unimplemented!()
    }
    /// Attach to a container
    pub async fn attach(&self) -> Result<(), Error> {
        unimplemented!()
    }
    /// Exec a command
    pub async fn exec(&self, opts: &ExecOpts) -> Result<String, Error> {
        unimplemented!()
    }
    // Starts the exec instance
    async fn start_exec_instance(
        &self,
        id: &str,
        detach: bool,
        tty: bool,
    ) -> Result<String, Error> {
        unimplemented!()
    }
    // Returns Id of exec instance
    async fn create_exec_instance(&self, opts: &ExecOpts) -> Result<String, Error> {
        unimplemented!()
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
        unimplemented!()
    }
    /// Create a container
    pub async fn create(&self, name: &str, opts: &ContainerBuilderOpts) -> Result<(), Error> {
        unimplemented!()
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
        unimplemented!()
    }
    ///Remove a network
    pub async fn remove(&self, id: &str) -> Result<(), Error> {
        unimplemented!()
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
        unimplemented!()
    }
    /// Pulls an image from registry  
    /// WARNING!  
    /// not specyfying tag will pull all tags of image
    pub async fn pull(&self, image: &str, tag: &str, auth: &AuthOpts) -> Result<(), Error> {
        unimplemented!()
    }
    /// Create an image by either pulling it from a registry or importing it.
    pub async fn create(&self, opts: &CreateImageOpts) -> Result<(), Error> {
        unimplemented!()
    }
    /// Remove an image
    pub async fn remove(&self, image: &str, force: bool, no_prune: bool) -> Result<(), Error> {
        unimplemented!()
    }
    /// Import images  
    /// Load a set of images and tags into a repository.
    pub async fn import(&self, archive: &[u8]) -> Result<(), Error> {
        unimplemented!()
    }
    /// Tag an image so that it becomes part of a repository.  
    /// **image** - name or id of image in the form: *someimage:sometag*  
    /// **repo** - The repository to tag in. For example, *someuser/someimage*  
    /// **tag** - The name of the new tag.
    pub async fn tag(&self, image: &str, repo: &str, tag: &str) -> Result<(), Error> {
        unimplemented!()
    }
    /// Inspect an image  
    /// Return low-level information about an image.
    pub async fn inspect(&self, image: &str) -> Result<ImageInspect, Error> {
        unimplemented!()
    }
    /// Get the history of an image  
    /// Return parent layers of an image
    pub async fn history(&self, image: &str) -> Result<Vec<ImageHistory>, Error> {
        unimplemented!()
    }
    /// Search for images on Docker Hub
    pub async fn search(
        &self,
        term: &str,
        limit: u64,
        filters: String,
    ) -> Result<Vec<ImageMatch>, Error> {
        unimplemented!()
    }
    /// Delete unused images
    pub async fn prune(&self, filters: &str) -> Result<ImagesDeleted, Error> {
        unimplemented!()
    }
    /// Build an image from a tar archive with a Dockerfile in it.
    ///The Dockerfile specifies how the image is built from the tar archive. It is typically in the archive's root, but can be at a different path or have a different name by specifying the dockerfile parameter. See the Dockerfile reference for more information.
    //The Docker daemon performs a preliminary validation of the Dockerfile before starting the build, and returns an error if the syntax is incorrect. After that, each instruction is run one-by-one until the ID of the new image is output.
    pub async fn build(&self, opts: &ImageBuilderOpts) -> Result<(), Error> {
        unimplemented!()
    }
}
// * Images End *
