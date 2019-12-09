use failure::Error;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
macro_rules! insert {
    ($s:ident, $k:expr, $v:ident) => {
        $s.opts.insert($k, serde_json::to_value($v).unwrap());
    };
}
macro_rules! query {
    ($s:ident) => {
        $s.opts
            .iter()
            .map(|(k, v)| (*k, serde_json::to_string(v).unwrap()))
            .collect()
    };
}
pub trait Query {
    fn to_query(self) -> Vec<(&'static str, String)>;
}
/// Options for Container::upload_archive method
pub struct UploadArchiveOpts {
    opts: HashMap<&'static str, Value>,
}
impl Query for UploadArchiveOpts {
    fn to_query(self) -> Vec<(&'static str, String)> {
        query!(self)
    }
}
impl UploadArchiveOpts {
    pub fn new() -> Self {
        UploadArchiveOpts {
            opts: HashMap::new(),
        }
    }
    /// Path to a directory in the container to extract the archive’s contents into.
    pub fn path<T: Into<String> + Serialize>(&mut self, path: T) {
        // It's a valid utf-8 string so its ok to unwrap here
        insert!(self, "path", path);
    }
    /// If “1”, “true”, or “True” then it will be an error if unpacking the given content would cause an existing directory to be replaced with a non-directory and vice versa.
    pub fn no_overwrite<T: Into<String> + Serialize>(&mut self, no_overwrite: T) {
        insert!(self, "noOverwriteDirNonDir", no_overwrite);
    }
    /// If “1”, “true”, then it will copy UID/GID maps to the dest file or dir
    pub fn copy_uid_gid<T: Into<String> + Serialize>(&mut self, copy_uid_gid: T) {
        insert!(self, "copyUIDGID", copy_uid_gid);
    }
}
/// Options for listing containers
pub struct ListContainersOpts {
    opts: HashMap<&'static str, Value>,
}
impl Query for ListContainersOpts {
    fn to_query(self) -> Vec<(&'static str, String)> {
        query!(self)
    }
}

impl ListContainersOpts {
    pub fn new() -> Self {
        ListContainersOpts {
            opts: HashMap::new(),
        }
    }
    pub fn all(&mut self, all: bool) {
        insert!(self, "all", all);
    }
    pub fn limit(&mut self, limit: usize) {
        insert!(self, "limit", limit);
    }
    pub fn size(&mut self, size: bool) {
        insert!(self, "size", size);
    }
    pub fn filters(&mut self, filters: bool) {
        insert!(self, "filters", filters);
    }
}
/// Options for Container::remove method
pub struct RmContainerOpts {
    opts: HashMap<&'static str, Value>,
}
impl Query for RmContainerOpts {
    fn to_query(self) -> Vec<(&'static str, String)> {
        query!(self)
    }
}
impl RmContainerOpts {
    pub fn new() -> Self {
        RmContainerOpts {
            opts: HashMap::new(),
        }
    }
    /// Remove the volumes associated with the container.
    pub fn volumes(&mut self, v: bool) {
        insert!(self, "volumes", v);
    }
    /// If the container is running, kill it before removing it.
    pub fn force(&mut self, force: bool) {
        insert!(self, "force", force);
    }
    /// Remove the specified link associated with the container.
    pub fn link(&mut self, link: bool) {
        insert!(self, "link", link);
    }
}
/// Options for Container::logs method
pub struct ContainerLogsOpts {
    opts: HashMap<&'static str, Value>,
}
impl Query for ContainerLogsOpts {
    fn to_query(self) -> Vec<(&'static str, String)> {
        query!(self)
    }
}
impl ContainerLogsOpts {
    pub fn new() -> Self {
        ContainerLogsOpts {
            opts: HashMap::new(),
        }
    }
    /// Keep connection after returning logs.
    pub fn follow(&mut self, follow: bool) {
        insert!(self, "follow", follow);
    }
    /// Return logs from stdout
    pub fn stdout(&mut self, stdout: bool) {
        insert!(self, "stdout", stdout);
    }
    /// Return logs from stderr
    pub fn stderr(&mut self, stderr: bool) {
        insert!(self, "stderr", stderr);
    }
    /// Only return logs since this time, as a UNIX timestamp
    pub fn since(&mut self, since: u32) {
        insert!(self, "since", since);
    }
    /// Only return logs before this time, as a UNIX timestamp
    pub fn until(&mut self, until: u32) {
        insert!(self, "until", until);
    }
    /// Add timestamps to every log file
    pub fn timestamps(&mut self, timestamps: bool) {
        insert!(self, "timestamps", timestamps);
    }
    /// Only return this number of log lines from the end of the logs. Specify as an integer or all to output all log lines
    pub fn tail(&mut self, tail: String) {
        insert!(self, "tail", tail);
    }
}

pub struct ContainerBuilderOpts {
    opts: HashMap<&'static str, Value>,
}
impl ContainerBuilderOpts {
    pub fn new() -> Self {
        ContainerBuilderOpts {
            opts: HashMap::new(),
        }
    }
    /// Get opts
    pub fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
    /// The hostname to use for the container, as a valid RFC 1123 hostname.
    pub fn hostname<S: Into<String> + Serialize>(&mut self, hostname: S) {
        insert!(self, "Hostname", hostname);
    }
    /// The domain name to use for the container.
    pub fn domain_name<S: Into<String> + Serialize>(&mut self, domain_name: S) {
        insert!(self, "DomainName", domain_name);
    }
    /// The user that commands are run as inside the container.
    pub fn user<S: Into<String> + Serialize>(&mut self, user: S) {
        insert!(self, "User", user);
    }
    /// Whether to attach to stdin.
    pub fn attach_stdin(&mut self, attach: bool) {
        insert!(self, "AttachStdin", attach);
    }
    /// Whether to attach to stdout.
    pub fn attach_stdout(&mut self, attach: bool) {
        insert!(self, "AttachStdout", attach);
    }
    /// Whether to attach to stderr.
    pub fn attach_stderr(&mut self, attach: bool) {
        insert!(self, "AttachStderr", attach);
    }
    /// Attach standard streams to a TTY, including stdin if it is not closed.
    pub fn tty(&mut self, tty: bool) {
        insert!(self, "Tty", tty);
    }
    /// Open stdin.
    pub fn open_stdin(&mut self, open: bool) {
        insert!(self, "OpenStdin", open);
    }
    /// Close stdin after one attached client disconnects
    pub fn stdin_once(&mut self, stdin_once: bool) {
        insert!(self, "StdinOnce", stdin_once);
    }
    /// A list of environment variables to set inside the container in the form ["VAR=value", ...].
    /// A variable without = is removed from the environment, rather than to have an empty value.
    pub fn Env(&mut self, env: &[String]) {
        insert!(self, "Env", env);
    }
    /// Command to run specified as a string or an array of strings.
    pub fn Cmd(&mut self, cmd: &[String]) {
        insert!(self, "Cmd", cmd);
    }
    /// Command is already escaped (Windows only)
    pub fn args_escaped(&mut self, escaped: bool) {
        insert!(self, "ArgsEscaped", escaped);
    }
    /// The name of the image to use when creating the container
    pub fn image<S: Into<String> + Serialize>(&mut self, image: S) {
        insert!(self, "Image", image);
    }
    /// The working directory for commands to run in.
    pub fn working_dir<S: Into<String> + Serialize>(&mut self, dir: S) {
        insert!(self, "WorkingDir", dir);
    }
    /// The entry point for the container as a string or an array of strings.
    /// If the array consists of exactly one empty string ([""]) then the entry point is reset to system default
    /// (i.e., the entry point used by docker when there is no ENTRYPOINT instruction in the Dockerfile).
    pub fn entrypoint(&mut self, entrypoint: &[String]) {
        insert!(self, "Entrypoint", entrypoint);
    }
    /// Disable networking for the container.
    pub fn network_disabled(&mut self, disabled: bool) {
        insert!(self, "NetworkDisabled", disabled);
    }
    /// MAC address of the container.
    pub fn mac_address<S: Into<String> + Serialize>(&mut self, addr: S) {
        insert!(self, "MacAddress", addr);
    }
    /// ONBUILD metadata that were defined in the image's Dockerfile.
    pub fn on_build(&mut self, md: &[String]) {
        insert!(self, "OnBuild", md);
    }
    /// Signal to stop a container as a string or unsigned integer.
    pub fn stop_signal<S: Into<String> + Serialize>(&mut self, signal: S) {
        insert!(self, "StopSignal", signal);
    }
    /// Timeout to stop a container in seconds.
    pub fn stop_timeout(&mut self, timeout: i64) {
        insert!(self, "StopTimeout", timeout);
    }
    /// Shell for when RUN, CMD, and ENTRYPOINT uses a shell.
    pub fn shell(&mut self, s: &[String]) {
        insert!(self, "Shell", s);
    }
    pub fn exposed_ports(&mut self, _: Value) {}
    pub fn health_check(&mut self, _: Value) {}
    pub fn labels(&mut self, _: Value) {}
    pub fn host_config(&mut self, _: Value) {}
    pub fn network_config(&mut self, _: Value) {}
}

/// Options for creating image
pub struct CreateImageOpts {
    opts: HashMap<&'static str, Value>,
    auth: AuthOpts,
}
impl Query for CreateImageOpts {
    fn to_query(self) -> Vec<(&'static str, String)> {
        query!(self)
    }
}
impl CreateImageOpts {
    pub fn new() -> Self {
        CreateImageOpts {
            opts: HashMap::new(),
            auth: AuthOpts::new(),
        }
    }
    pub fn from_image(&mut self, from_image: &str) {
        insert!(self, "fromImage", from_image);
    }
    pub fn from_src(&mut self, from_src: &str) {
        insert!(self, "fromSrc", from_src);
    }
    pub fn repo(&mut self, repo: &str) {
        insert!(self, "repo", repo);
    }
    pub fn tag(&mut self, tag: &str) {
        insert!(self, "tag", tag);
    }
    pub fn platform(&mut self, platform: &str) {
        insert!(self, "platform", platform);
    }
    pub fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
    pub fn auth(&mut self) -> &mut AuthOpts {
        &mut self.auth
    }
    pub(crate) fn auth_ref(&self) -> &AuthOpts {
        &self.auth
    }
}

/// Options for authentication
pub struct AuthOpts {
    opts: HashMap<&'static str, Value>,
}

impl AuthOpts {
    pub fn new() -> Self {
        AuthOpts {
            opts: HashMap::new(),
        }
    }
    pub fn username(&mut self, username: &str) {
        insert!(self, "username", username);
    }
    pub fn password(&mut self, password: &str) {
        insert!(self, "password", password);
    }
    pub fn email(&mut self, email: &str) {
        insert!(self, "email", email);
    }
    pub fn server_address(&mut self, server_address: &str) {
        insert!(self, "serveraddress", server_address);
    }
    pub fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
    pub fn serialize(&self) -> Result<String, Error> {
        Ok(base64::encode(&serde_json::to_string(&self.opts)?))
    }
}
