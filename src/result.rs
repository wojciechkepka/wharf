//! Rust-y results from docker json results
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub(crate) struct ContainerJson {
    pub(crate) Id: String,
    Names: Vec<String>,
    Image: String,
    ImageID: String,
    Command: String,
    Created: usize,
    State: Value,
    Status: String,
    Ports: Vec<Value>,
    Labels: Value,
    HostConfig: Value,
    NetworkSettings: Value,
    Mounts: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ContainerProcessesJson {
    pub(crate) Titles: Vec<String>,
    pub(crate) Processes: Vec<Vec<String>>,
}

/// Result data of container.inspect()
#[derive(Deserialize, Debug, Serialize)]
pub struct InspectContainer {
    AppArmorProfile: String,
    Args: Value,
    Config: Value,
    Created: String,
    Driver: String,
    ExecIDs: Value,
    HostConfig: Value,
    HostnamePath: String,
    HostsPath: String,
    LogPath: String,
    Id: String,
    MountLabel: String,
    Name: String,
    NetworkSettings: Value,
    Path: String,
    ProcessLabel: String,
    ResolvConfPath: String,
    RestartCount: usize,
    State: Value,
    Mounts: Vec<Value>,
}

// TODO: add some public api for this
#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    name: String,
    size: usize,
    mode: usize,
    mtime: String,
    linkTarget: String,
}

// TODO: Rethink making this public
#[derive(Serialize, Deserialize, Debug)]
pub struct ImagesJson {
    Id: String,
    ParentId: String,
    RepoTags: Vec<String>,
    RepoDigests: Vec<String>,
    Created: u64,
    Size: isize,
    VirtualSize: isize,
    SharedSize: isize,
    Labels: Value,
    Containers: isize,
}
#[derive(Debug)]
pub struct Process {
    pub info: HashMap<String, String>,
}
impl Process {
    pub(crate) fn new(titles: &[String], processes: &[String]) -> Self {
        Process {
            info: titles
                .iter()
                .cloned()
                .zip(processes.iter().cloned())
                .collect(),
        }
    }
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
