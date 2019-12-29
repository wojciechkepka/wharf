//! Rust-y results from docker json results
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Container data returned from containers.list()
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ContainerData {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Names")]
    pub names: Vec<String>,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "ImageID")]
    pub image_id: String,
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "Created")]
    pub created: i64,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Ports")]
    pub ports: Vec<Value>,
    #[serde(rename = "Labels")]
    pub labels: Value,
    #[serde(rename = "HostConfig")]
    pub host_config: Value,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: Value,
    #[serde(rename = "Mounts")]
    pub mounts: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
// Needed to create Vec<Process> for container.ps()
pub(crate) struct ContainerProcessesJson {
    pub(crate) Titles: Vec<String>,
    pub(crate) Processes: Vec<Vec<String>>,
}

/// Result data of container.inspect()
#[derive(Deserialize, Debug, Serialize)]
pub struct ContainerInspect {
    #[serde(rename = "AppArmorProfile")]
    pub app_armor_profile: String,
    #[serde(rename = "Args")]
    pub args: Vec<String>,
    #[serde(rename = "Config")]
    pub config: Value,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "ExecIDs")]
    pub exec_ids: Vec<String>,
    #[serde(rename = "HostConfig")]
    pub host_config: Value,
    #[serde(rename = "HostnamePath")]
    pub hostname_path: String,
    #[serde(rename = "HostsPath")]
    pub hosts_path: String,
    #[serde(rename = "LogPath")]
    pub log_path: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "MountLabel")]
    pub mount_label: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: Value,
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "ProcessLabel")]
    pub process_label: String,
    #[serde(rename = "ResolvConfPath")]
    pub resolv_conf_path: String,
    #[serde(rename = "RestartCount")]
    pub restart_count: i64,
    #[serde(rename = "State")]
    pub state: Value,
    #[serde(rename = "Mounts")]
    pub mounts: Vec<Value>,
}

/// Data returned from container.file_info()
#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub name: String,
    pub size: usize,
    pub mode: usize,
    pub mtime: String,
    pub linkTarget: String,
}

/// Image data returned from images.list()
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageData {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ParentId")]
    pub parent_id: String,
    #[serde(rename = "RepoTags")]
    pub repo_tags: Vec<String>,
    #[serde(rename = "RepoDigests")]
    pub repo_digests: Vec<String>,
    #[serde(rename = "Created")]
    pub created: i64,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "VirtualSize")]
    pub virtual_size: i64,
    #[serde(rename = "SharedSize")]
    pub shared_size: i64,
    #[serde(rename = "Labels")]
    pub labels: Value,
    #[serde(rename = "Containers")]
    pub containers: i64,
}

/// Detailed Image data returned from image.inspect()
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageInspect {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Container")]
    pub container: String,
    #[serde(rename = "Comment")]
    pub comment: String,
    #[serde(rename = "Os")]
    pub os: String,
    #[serde(rename = "Architecture")]
    pub architecture: String,
    #[serde(rename = "Parent")]
    pub parent: String,
    #[serde(rename = "ContainerConfig")]
    pub container_config: Value,
    #[serde(rename = "DockerVersion")]
    pub docker_version: String,
    #[serde(rename = "VirtualSize")]
    pub virtual_size: i64,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "Author")]
    pub author: String,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "GraphDriver")]
    pub graph_driver: Value,
    #[serde(rename = "RepoDigests")]
    pub repo_digests: Vec<String>,
    #[serde(rename = "RepoTags")]
    pub repo_tags: Vec<String>,
    #[serde(rename = "Config")]
    pub config: Value,
    #[serde(rename = "RootFS")]
    pub root_fs: Value,
}

/// The history of image usage returned from image.history()
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageHistory {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Created")]
    pub created: i64,
    #[serde(rename = "CreatedBy")]
    pub created_by: String,
    #[serde(rename = "Tags")]
    pub tags: Value,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "Comment")]
    pub comment: String,
}

/// Image search result from images.search()
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageMatch {
    pub description: String,
    pub is_official: bool,
    pub is_automated: bool,
    pub name: String,
    pub star_count: i64,
}

// Actual output from images.prune()
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ImagesDeleteOut {
    #[serde(rename = "ImagesDeleted")]
    pub images_deleted: Vec<ImagesDeleted>,
    #[serde(rename = "SpaceReclaimed")]
    pub space_reclaimed: i64,
}

/// Deleted images from images.prune()
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ImagesDeleted {
    #[serde(rename = "Untagged")]
    pub untagged: String,
    #[serde(rename = "Deleted")]
    pub deleted: String,
}

/// Information about a process returned from container.ps()  
#[derive(Debug)]
pub struct Process {
    /// May contain different information based on the flags passed to .ps()
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

/// Information about a network returned from networks.list()
#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkData {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "Scope")]
    pub scope: String,
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "EnableIPv6")]
    pub enable_ipv6: bool,
    #[serde(rename = "Internal")]
    pub internal: bool,
    #[serde(rename = "Attachable")]
    pub attachable: bool,
    #[serde(rename = "Ingress")]
    pub ingress: bool,
    #[serde(rename = "IPAM")]
    pub ipam: Value,
    #[serde(rename = "Options")]
    pub options: Value,
    #[serde(rename = "Containers")]
    pub containers: Option<Value>,
}

/// Exec output and data
#[derive(Debug)]
pub struct CmdOut {
    pub out: String,
    pub info: ExecInspect,
}

/// Information about a exec instance
#[derive(Serialize, Deserialize, Debug)]
pub struct ExecInspect {
    #[serde(rename = "CanRemove")]
    pub can_remove: bool,
    #[serde(rename = "ContainerID")]
    pub container_id: String,
    #[serde(rename = "DetachKeys")]
    pub detach_keys: String,
    #[serde(rename = "ExitCode")]
    pub exit_code: i64,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "OpenStderr")]
    pub open_stderr: bool,
    #[serde(rename = "OpenStdin")]
    pub open_stdin: bool,
    #[serde(rename = "OpenStdout")]
    pub open_stdout: bool,
    #[serde(rename = "ProcessConfig")]
    pub process_config: Option<Value>,
    #[serde(rename = "Running")]
    pub running: bool,
    #[serde(rename = "Pid")]
    pub pid: Option<i64>,
}
