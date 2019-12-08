use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
pub trait Query {
    fn to_query(self) -> Vec<(&'static str, String)>;
}
/// Options for Container::upload_archive method
pub struct UploadArchiveOpts {
    path: String,
    no_overwrite: String,
    copy_uid_gid: String,
}
impl Query for UploadArchiveOpts {
    fn to_query(self) -> Vec<(&'static str, String)> {
        vec![
            ("path", self.path),
            ("noOverwriteDirNonDir", self.no_overwrite),
            ("copyUIDGID", self.copy_uid_gid),
        ]
    }
}
impl UploadArchiveOpts {
    pub fn new() -> Self {
        UploadArchiveOpts {
            path: "".to_string(),
            no_overwrite: "".to_string(),
            copy_uid_gid: "".to_string(),
        }
    }
    /// Path to a directory in the container to extract the archive’s contents into.
    pub fn path<T: Into<String> + Serialize>(&mut self, path: T) {
        self.path = path.into();
    }
    /// If “1”, “true”, or “True” then it will be an error if unpacking the given content would cause an existing directory to be replaced with a non-directory and vice versa.
    pub fn no_overwrite<T: Into<String> + Serialize>(&mut self, no_overwrite: T) {
        self.no_overwrite = no_overwrite.into();
    }
    /// If “1”, “true”, then it will copy UID/GID maps to the dest file or dir
    pub fn copy_uid_gid<T: Into<String> + Serialize>(&mut self, copy_uid_gid: T) {
        self.copy_uid_gid = copy_uid_gid.into();
    }
}
/// Options for Container::remove method
pub struct RmContainerOpts {
    v: bool,
    force: bool,
    link: bool,
}
impl Query for RmContainerOpts {
    fn to_query(self) -> Vec<(&'static str, String)> {
        vec![
            ("v", self.v.to_string()),
            ("force", self.force.to_string()),
            ("link", self.link.to_string()),
        ]
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

#[derive(Serialize, Deserialize)]
pub struct ContainerBuilderOpts {
    pub opts: HashMap<String, Value>,
}
macro_rules! insert {
    ($s:ident, $k:expr, $v:ident) => {
        $s.opts
            .insert($k.to_string(), serde_json::to_value($v).unwrap());
    };
}
impl ContainerBuilderOpts {
    pub fn new() -> Self {
        ContainerBuilderOpts {
            opts: HashMap::new(),
        }
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
    fromImage: String,
    fromSrc: String,
    repo: String,
    tag: String,
    platform: String,
}
impl Query for CreateImageOpts {
    fn to_query(self) -> Vec<(&'static str, String)> {
        vec![]
    }
}

/// Options for authentication
#[derive(Default)]
pub struct AuthOpts {
    username: String,
    password: String,
    email: String,
    server_address: String,
}

impl AuthOpts {
    pub fn new() -> Self {
        AuthOpts::default()
    }
    pub fn username<S: Into<String>>(&mut self, username: S) {
        self.username = username.into();
    }
    pub fn password<S: Into<String>>(&mut self, password: S) {
        self.password = password.into();
    }
    pub fn email<S: Into<String>>(&mut self, email: S) {
        self.email = email.into();
    }
    pub fn server_address<S: Into<String>>(&mut self, server_address: S) {
        self.server_address = server_address.into();
    }

    pub fn serialize(self) -> HashMap<String, String> {
        let mut data = HashMap::new();
        data.insert("username".to_string(), self.username);
        data.insert("password".to_string(), self.password);
        data.insert("email".to_string(), self.email);
        data.insert("serveraddress".to_string(), self.server_address);
        data
    }
}
