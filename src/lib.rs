#![allow(non_snake_case)]
#[macro_use] extern crate failure;
use opts::Query;
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
#[derive(Serialize, Deserialize)]
struct Msg {
    message: String
}
impl Msg {
    fn msg(self) -> String {
        self.message
    }
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
    }}
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
    pub async fn start(docker: &Docker, id: String) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let res = client.post(docker.url.join(&format!("containers/{}/start", id))?).body("").send().await?;
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
    pub async fn stop(docker: &Docker, id: String) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let res = client.post(docker.url.join(&format!("containers/{}/stop", id))?).body("").send().await?;
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
    pub async fn restart(docker: &Docker, id: String) -> Result<(), Error> {
        Ok(post_container!(&format!("containers/{}/restart", id), docker)?)
    }
    /// Kills the container
    pub async fn kill(docker: &Docker, id: String) -> Result<(), Error> {
        Ok(post_container!(&format!("containers/{}/kill", id), docker)?)
    }
    /// Unpauses the container
    pub async fn unpause(docker: &Docker, id: String) -> Result<(), Error> {
        Ok(post_container!(&format!("containers/{}/unpause", id), docker)?)
    }
    /// Pauses the container
    pub async fn pause(docker: &Docker, id: String) -> Result<(), Error> {
        Ok(post_container!(&format!("containers/{}/pause", id), docker)?)
    }
    /// Rename container
    pub async fn rename(docker: &Docker, id: String, new_name: String) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let res = client.post(docker.url.join(&format!("containers/{}/rename", id))?).query(&[("name", new_name)]).send().await?;
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
    pub async fn remove(docker: &Docker, id: String, opts: opts::RmContainerOpts) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let res = client.post(docker.url.join(&format!("containers/{}/rename", id))?).query(&opts.to_query()).send().await?;
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
    pub async fn logs(docker: &Docker, id: String, opts: opts::ContainerLogsOpts) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let res = client.get(docker.url.join(&format!("containers/{}/logs", id))?).query(&opts.to_query()).send().await?;
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
}
pub mod opts {
    pub trait Query {
        fn to_query(self) -> Vec<(&'static str, String)>;
    }
    /// Options for Container::remove method
    pub struct RmContainerOpts {
        v: bool,
        force: bool,
        link: bool,
    }
    impl Query for RmContainerOpts {
        fn to_query(self) -> Vec<(&'static str, String)> {
            vec![("v", self.v.to_string()), ("force", self.force.to_string())]
        }
        
    }
    impl RmContainerOpts {
        pub fn new() -> Self {
            RmContainerOpts {
                v: false,
                force: false,
                link: false,
            }
        }
        /// Remove the volumes associated with the container.
        pub fn volumes(&mut self, v: bool) {
            self.v = v;
        }
        /// If the container is running, kill it before removing it.
        pub fn force(&mut self, force: bool) {
            self.force = force;
        }
        /// Remove the specified link associated with the container.
        pub fn link(&mut self, link: bool) {
            self.link = link;
        }
    }
    /// Options for Container::logs method
    pub struct ContainerLogsOpts {
        follow: bool,
        stdout: bool,
        stderr: bool,
        since: u32,
        until: u32,
        timestamps: bool,
        tail: String,
    }
    impl Query for ContainerLogsOpts {
        fn to_query(self) -> Vec<(&'static str, String)> {
            vec![
                ("follow", self.follow.to_string()),
                ("stdout", self.stdout.to_string()),
                ("stderr", self.stderr.to_string()),
                ("since", self.since.to_string()),
                ("until", self.until.to_string()),
                ("timestamps", self.timestamps.to_string()),
                ("tail", self.tail),
            ]
        }
        
    }
    impl ContainerLogsOpts {
        pub fn new() -> Self {
            ContainerLogsOpts {
                follow: false,
                stdout: false,
                stderr: false,
                since: 0,
                until: 0,
                timestamps: false,
                tail: "all".to_string(),
            }
        }
        /// Keep connection after returning logs.
        pub fn follow(&mut self, follow: bool) {
            self.follow = follow;
        }
        /// Return logs from stdout
        pub fn stdout(&mut self, stdout: bool) {
            self.stdout = stdout;
        }
        /// Return logs from stderr
        pub fn stderr(&mut self, stderr: bool) {
            self.stderr = stderr;
        }
        /// Only return logs since this time, as a UNIX timestamp
        pub fn since(&mut self, since: u32) {
            self.since = since;
        }
        /// Only return logs before this time, as a UNIX timestamp
        pub fn until(&mut self, until: u32) {
            self.until = until;
        }
        /// Add timestamps to every log file
        pub fn timestamps(&mut self, timestamps: bool) {
            self.timestamps = timestamps;
        }
        /// Only return this number of log lines from the end of the logs. Specify as an integer or all to output all log lines
        pub fn tail(&mut self, tail: String) {
            self.tail = tail;
        }
    }
}

/// Api wrapper for containers
pub struct Containers {}
impl Containers {
    pub async fn list(docker: &Docker) -> Result<Vec<Container> , Error> {
        let res = reqwest::get(docker.url.join("containers/json")?).await?;
        let body = res.text().await?;
        Ok(serde_json::from_str(&body)?)
    }
}

pub struct Images {}
impl Images {
    pub async fn list(docker: &Docker) -> Result<Vec<Image> , Error> {
        let res = reqwest::get(docker.url.join("images/json")?).await?;
        let body = res.text().await?;
        Ok(serde_json::from_str(&body)?)
    }
}

pub struct Networks {}
impl Networks {
    pub async fn list(docker: &Docker) -> Result<Vec<Network> , Error> {
        let res = reqwest::get(docker.url.join("networks")?).await?;
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
