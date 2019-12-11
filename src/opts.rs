use failure::Error;
use log::*;
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
pub trait Query<'o> {
    fn to_query(&self) -> Vec<(&'o str, String)>;
}
/// Options for Container::upload_archive method
pub struct UploadArchiveOpts {
    opts: HashMap<&'static str, Value>,
}
impl<'o> Query<'o> for UploadArchiveOpts {
    fn to_query(&self) -> Vec<(&'o str, String)> {
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
    pub fn path<T: Into<String> + Serialize>(&mut self, path: T) -> &mut Self {
        // It's a valid utf-8 string so its ok to unwrap here
        insert!(self, "path", path);
        self
    }
    /// If “1”, “true”, or “True” then it will be an error if unpacking the given content would cause an existing directory to be replaced with a non-directory and vice versa.
    pub fn no_overwrite<T: Into<String> + Serialize>(&mut self, no_overwrite: T) -> &mut Self {
        insert!(self, "noOverwriteDirNonDir", no_overwrite);
        self
    }
    /// If “1”, “true”, then it will copy UID/GID maps to the dest file or dir
    pub fn copy_uid_gid<T: Into<String> + Serialize>(&mut self, copy_uid_gid: T) -> &mut Self {
        insert!(self, "copyUIDGID", copy_uid_gid);
        self
    }
}
/// Options for listing containers
pub struct ListContainersOpts {
    opts: HashMap<&'static str, Value>,
}
impl<'o> Query<'o> for ListContainersOpts {
    fn to_query(&self) -> Vec<(&'o str, String)> {
        query!(self)
    }
}

impl ListContainersOpts {
    pub fn new() -> Self {
        ListContainersOpts {
            opts: HashMap::new(),
        }
    }
    pub fn all(&mut self, all: bool) -> &mut Self {
        insert!(self, "all", all);
        self
    }
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        insert!(self, "limit", limit);
        self
    }
    pub fn size(&mut self, size: bool) -> &mut Self {
        insert!(self, "size", size);
        self
    }
    pub fn filters(&mut self, filters: bool) -> &mut Self {
        insert!(self, "filters", filters);
        self
    }
}
/// Options for Container::remove method
pub struct RmContainerOpts {
    opts: HashMap<&'static str, Value>,
}
impl<'o> Query<'o> for RmContainerOpts {
    fn to_query(&self) -> Vec<(&'o str, String)> {
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
    pub fn volumes(&mut self, v: bool) -> &mut Self {
        insert!(self, "volumes", v);
        self
    }
    /// If the container is running, kill it before removing it.
    pub fn force(&mut self, force: bool) -> &mut Self {
        insert!(self, "force", force);
        self
    }
    /// Remove the specified link associated with the container.
    pub fn link(&mut self, link: bool) -> &mut Self {
        insert!(self, "link", link);
        self
    }
}
/// Options for Container::logs method
pub struct ContainerLogsOpts {
    opts: HashMap<&'static str, Value>,
}
impl<'o> Query<'o> for ContainerLogsOpts {
    fn to_query(&self) -> Vec<(&'o str, String)> {
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
    pub fn follow(&mut self, follow: bool) -> &mut Self {
        insert!(self, "follow", follow);
        self
    }
    /// Return logs from stdout
    pub fn stdout(&mut self, stdout: bool) -> &mut Self {
        insert!(self, "stdout", stdout);
        self
    }
    /// Return logs from stderr
    pub fn stderr(&mut self, stderr: bool) -> &mut Self {
        insert!(self, "stderr", stderr);
        self
    }
    /// Only return logs since this time, as a UNIX timestamp
    pub fn since(&mut self, since: u32) -> &mut Self {
        insert!(self, "since", since);
        self
    }
    /// Only return logs before this time, as a UNIX timestamp
    pub fn until(&mut self, until: u32) -> &mut Self {
        insert!(self, "until", until);
        self
    }
    /// Add timestamps to every log file
    pub fn timestamps(&mut self, timestamps: bool) -> &mut Self {
        insert!(self, "timestamps", timestamps);
        self
    }
    /// Only return this number of log lines from the end of the logs. Specify as an integer or all to output all log lines
    pub fn tail(&mut self, tail: String) -> &mut Self {
        insert!(self, "tail", tail);
        self
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
    pub fn hostname<S: Into<String> + Serialize>(&mut self, hostname: S) -> &mut Self {
        insert!(self, "Hostname", hostname);
        self
    }
    /// The domain name to use for the container.
    pub fn domain_name<S: Into<String> + Serialize>(&mut self, domain_name: S) -> &mut Self {
        insert!(self, "DomainName", domain_name);
        self
    }
    /// The user that commands are run as inside the container.
    pub fn user<S: Into<String> + Serialize>(&mut self, user: S) -> &mut Self {
        insert!(self, "User", user);
        self
    }
    /// Whether to attach to stdin.
    pub fn attach_stdin(&mut self, attach: bool) -> &mut Self {
        insert!(self, "AttachStdin", attach);
        self
    }
    /// Whether to attach to stdout.
    pub fn attach_stdout(&mut self, attach: bool) -> &mut Self {
        insert!(self, "AttachStdout", attach);
        self
    }
    /// Whether to attach to stderr.
    pub fn attach_stderr(&mut self, attach: bool) -> &mut Self {
        insert!(self, "AttachStderr", attach);
        self
    }
    /// Attach standard streams to a TTY, including stdin if it is not closed.
    pub fn tty(&mut self, tty: bool) -> &mut Self {
        insert!(self, "Tty", tty);
        self
    }
    /// Open stdin.
    pub fn open_stdin(&mut self, open: bool) -> &mut Self {
        insert!(self, "OpenStdin", open);
        self
    }
    /// Close stdin after one attached client disconnects
    pub fn stdin_once(&mut self, stdin_once: bool) -> &mut Self {
        insert!(self, "StdinOnce", stdin_once);
        self
    }
    /// A list of environment variables to set inside the container in the form ["VAR=value", ...].
    /// A variable without = is removed from the environment, rather than to have an empty value.
    pub fn env(&mut self, env: &[String]) -> &mut Self {
        insert!(self, "Env", env);
        self
    }
    /// Command to run specified as a string or an array of strings.
    pub fn cmd(&mut self, cmd: &[String]) -> &mut Self {
        insert!(self, "Cmd", cmd);
        self
    }
    /// Command is already escaped (Windows only)
    pub fn args_escaped(&mut self, escaped: bool) -> &mut Self {
        insert!(self, "ArgsEscaped", escaped);
        self
    }
    /// The name of the image to use when creating the container
    pub fn image<S: Into<String> + Serialize>(&mut self, image: S) -> &mut Self {
        insert!(self, "Image", image);
        self
    }
    /// The working directory for commands to run in.
    pub fn working_dir<S: Into<String> + Serialize>(&mut self, dir: S) -> &mut Self {
        insert!(self, "WorkingDir", dir);
        self
    }
    /// The entry point for the container as a string or an array of strings.
    /// If the array consists of exactly one empty string ([""]) then the entry point is reset to system default
    /// (i.e., the entry point used by docker when there is no ENTRYPOINT instruction in the Dockerfile).
    pub fn entrypoint(&mut self, entrypoint: &[String]) -> &mut Self {
        insert!(self, "Entrypoint", entrypoint);
        self
    }
    /// Disable networking for the container.
    pub fn network_disabled(&mut self, disabled: bool) -> &mut Self {
        insert!(self, "NetworkDisabled", disabled);
        self
    }
    /// MAC address of the container.
    pub fn mac_address<S: Into<String> + Serialize>(&mut self, addr: S) -> &mut Self {
        insert!(self, "MacAddress", addr);
        self
    }
    /// ONBUILD metadata that were defined in the image's Dockerfile.
    pub fn on_build(&mut self, md: &[String]) -> &mut Self {
        insert!(self, "OnBuild", md);
        self
    }
    /// Signal to stop a container as a string or unsigned integer.
    pub fn stop_signal<S: Into<String> + Serialize>(&mut self, signal: S) -> &mut Self {
        insert!(self, "StopSignal", signal);
        self
    }
    /// Timeout to stop a container in seconds.
    pub fn stop_timeout(&mut self, timeout: i64) -> &mut Self {
        insert!(self, "StopTimeout", timeout);
        self
    }
    /// Shell for when RUN, CMD, and ENTRYPOINT uses a shell.
    pub fn shell(&mut self, s: &[String]) -> &mut Self {
        insert!(self, "Shell", s);
        self
    }
    /// A list of string in the form:
    /// "port/<tcp|udp|sctp>"
    pub fn exposed_ports(&mut self, ports: &[String]) -> &mut Self {
        let exposed_ports: HashMap<&str, Value> = ports
            .iter()
            .map(|port| (&port[..], Value::default()))
            .collect();
        debug!("{:?}", exposed_ports);
        //TODO
        //figure out what's the difference
        //insert!(self, "ExposedPorts", exposed_ports);
        insert!(self, "HostConfig.PortBindings", exposed_ports);
        self
    }
    /// A list of mounts in the container in the form:
    /// "/host/path:/container/path"
    pub fn volumes<S: AsRef<str>>(&mut self, mounts: &[S]) -> &mut Self {
        let volumes: HashMap<&str, Value> = mounts
            .iter()
            .map(|m| (m.as_ref(), Value::default()))
            .collect();
        debug!("{:?}", volumes);
        insert!(self, "HostConfig.Binds", volumes);
        self
    }
    /// User-defined key/value metadata.
    pub fn labels(&mut self, labels: &HashMap<&str, &str>) -> &mut Self {
        insert!(self, "Labels", labels);
        self
    }
}

/// Options for creating image
pub struct CreateImageOpts {
    opts: HashMap<&'static str, Value>,
    auth: AuthOpts,
}
impl<'o> Query<'o> for CreateImageOpts {
    fn to_query(&self) -> Vec<(&'o str, String)> {
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
    pub fn from_image(&mut self, from_image: &str) -> &mut Self {
        insert!(self, "fromImage", from_image);
        self
    }
    pub fn from_src(&mut self, from_src: &str) -> &mut Self {
        insert!(self, "fromSrc", from_src);
        self
    }
    pub fn repo(&mut self, repo: &str) -> &mut Self {
        insert!(self, "repo", repo);
        self
    }
    pub fn tag(&mut self, tag: &str) -> &mut Self {
        insert!(self, "tag", tag);
        self
    }
    pub fn platform(&mut self, platform: &str) -> &mut Self {
        insert!(self, "platform", platform);
        self
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
    pub fn username(&mut self, username: &str) -> &mut Self {
        insert!(self, "username", username);
        self
    }
    pub fn password(&mut self, password: &str) -> &mut Self {
        insert!(self, "password", password);
        self
    }
    pub fn email(&mut self, email: &str) -> &mut Self {
        insert!(self, "email", email);
        self
    }
    pub fn server_address(&mut self, server_address: &str) -> &mut Self {
        insert!(self, "serveraddress", server_address);
        self
    }
    pub fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
    pub fn serialize(&self) -> Result<String, Error> {
        Ok(base64::encode(&serde_json::to_string(&self.opts)?))
    }
}
