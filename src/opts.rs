//! Configuration builders for all methods.
//!
//! You can easily chain options by doing:
//! ```ignore
//! opts.path("/example/path").no_overwrite("true").copy_uid_gid("false");
//! ```
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
pub trait DockerOpts {
    fn opts(&self) -> &HashMap<&'static str, Value>;

    fn to_query(&self) -> Result<String, Error> {
        let q: Vec<String> = self
            .opts()
            .iter()
            .map(|(k, v)| {
                format!(
                    "{}={}",
                    k,
                    serde_json::to_string(&v).unwrap().trim_matches('"')
                )
            })
            .collect();
        Ok(format!("{}", q.join("&")))
    }
}
impl DockerOpts for UploadArchiveOpts {
    fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
}
impl DockerOpts for AuthOpts {
    fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
}
impl DockerOpts for ContainerBuilderOpts {
    fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
}
impl DockerOpts for ContainerLogsOpts {
    fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
}
impl DockerOpts for CreateImageOpts {
    fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
}
impl DockerOpts for ExecOpts {
    fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
}
impl DockerOpts for ListContainersOpts {
    fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
}
impl DockerOpts for RmContainerOpts {
    fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
}
impl DockerOpts for ImageBuilderOpts {
    fn opts(&self) -> &HashMap<&'static str, Value> {
        &self.opts
    }
}

/// Options for uploading an archive to a container
#[derive(Default)]
pub struct UploadArchiveOpts {
    opts: HashMap<&'static str, Value>,
}
impl UploadArchiveOpts {
    pub fn new() -> Self {
        UploadArchiveOpts::default()
    }
    /// Path to a directory in the container to extract the archive’s contents into.
    pub fn path(&mut self, path: &str) -> &mut Self {
        // It's a valid utf-8 string so its ok to unwrap here
        insert!(self, "path", path);
        self
    }
    /// If “1”, “true”, or “True” then it will be an error if unpacking the given content would cause an existing directory to be replaced with a non-directory and vice versa.
    pub fn no_overwrite(&mut self, no_overwrite: &str) -> &mut Self {
        insert!(self, "noOverwriteDirNonDir", no_overwrite);
        self
    }
    /// If “1”, “true”, then it will copy UID/GID maps to the dest file or dir
    pub fn copy_uid_gid(&mut self, copy_uid_gid: &str) -> &mut Self {
        insert!(self, "copyUIDGID", copy_uid_gid);
        self
    }
}
/// Options for listing containers
#[derive(Default)]
pub struct ListContainersOpts {
    opts: HashMap<&'static str, Value>,
}
impl ListContainersOpts {
    pub fn new() -> Self {
        ListContainersOpts::default()
    }
    /// Return all containers. By default, only running containers are shown
    pub fn all(&mut self, all: bool) -> &mut Self {
        insert!(self, "all", all);
        self
    }
    /// Return this number of most recently created containers, including non-running ones.
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        insert!(self, "limit", limit);
        self
    }
    /// Return the size of container as fields SizeRw and SizeRootFs.
    pub fn size(&mut self, size: bool) -> &mut Self {
        insert!(self, "size", size);
        self
    }
    /// Filters to process on the container list, encoded as JSON (a map[string][]string). For example, {"status": ["paused"]} will only return paused containers.
    /// for more information head to [docker reference](https://docs.docker.com/engine/api/v1.40/#operation/ContainerList)
    pub fn filters(&mut self, filters: &str) -> &mut Self {
        insert!(self, "filters", filters);
        self
    }
}
/// Options for removing a container
#[derive(Default)]
pub struct RmContainerOpts {
    opts: HashMap<&'static str, Value>,
}
impl RmContainerOpts {
    pub fn new() -> Self {
        RmContainerOpts::default()
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
/// Options for container logs
#[derive(Default)]
pub struct ContainerLogsOpts {
    opts: HashMap<&'static str, Value>,
}
impl ContainerLogsOpts {
    pub fn new() -> Self {
        ContainerLogsOpts::default()
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

/// Options for building a container
#[derive(Default)]
pub struct ContainerBuilderOpts {
    opts: HashMap<&'static str, Value>,
}
impl ContainerBuilderOpts {
    pub fn new() -> Self {
        ContainerBuilderOpts::default()
    }
    /// Get opts
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
    pub fn env(&mut self, env: &[&str]) -> &mut Self {
        insert!(self, "Env", env);
        self
    }
    /// Command to run specified as a string or an array of strings.
    pub fn cmd(&mut self, cmd: &[&str]) -> &mut Self {
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
    pub fn entrypoint(&mut self, entrypoint: &[&str]) -> &mut Self {
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
    pub fn on_build(&mut self, md: &[&str]) -> &mut Self {
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
    pub fn shell(&mut self, s: &[&str]) -> &mut Self {
        insert!(self, "Shell", s);
        self
    }
    /// A list of string in the form:
    /// "port/<tcp|udp|sctp>"
    pub fn exposed_ports<S: AsRef<str>>(&mut self, ports: &[S]) -> &mut Self {
        let exposed_ports: HashMap<&str, Value> = ports
            .iter()
            .map(|port| (port.as_ref(), Value::default()))
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
    /// Memory limit in bytes
    pub fn memory(&mut self, limit: i64) -> &mut Self {
        insert!(self, "HostConfig.Memory", limit);
        self
    }
    /// Network mode to use for this container.
    /// Supported standard values are: bridge, host, none, and container:<name|id>.
    /// Any other value is taken as a custom network's name to which this container should connect to.
    pub fn network_mode(&mut self, mode: &str) -> &mut Self {
        insert!(self, "HostConfig.NetworkMode", mode);
        self
    }
}

/// Options for building an image
#[derive(Default)]
pub struct ImageBuilderOpts {
    opts: HashMap<&'static str, Value>,
}
impl ImageBuilderOpts {
    /// Path within the build context to the Dockerfile.  
    /// This is ignored if remote is specified and points to an external Dockerfile.
    pub fn dockerfile(&mut self, path: String) -> &mut Self {
        insert!(self, "dockerfile", path);
        self
    }
    /// A name and optional tag to apply to the image in the name:tag format.  
    /// If you omit the tag the default latest value is assumed.
    pub fn name(&mut self, name: String) -> &mut Self {
        insert!(self, "t", name);
        self
    }
    /// A Git repository URI or HTTP/HTTPS context URI.  
    /// If the URI points to a single text file, the file’s contents are placed into a file called Dockerfile and the image is built from that file.  
    /// If the URI points to a tarball, the file is downloaded by the daemon and the contents therein used as the context for the build.  
    /// If the URI points to a tarball and the dockerfile parameter is also specified, there must be a file with the corresponding path inside the tarball.
    pub fn remote(&mut self, repo: &str) -> &mut Self {
        insert!(self, "remote", repo);
        self
    }
    /// Extra hosts to add to /etc/hosts
    pub fn extra_hosts(&mut self, hosts: String) -> &mut Self {
        insert!(self, "extrahosts", hosts);
        self
    }
    /// Suppress verbose build output.
    pub fn quiet(&mut self, q: bool) -> &mut Self {
        insert!(self, "q", q);
        self
    }
    /// Do not use the cache when building the image.
    pub fn no_cache(&mut self, no_cache: bool) -> &mut Self {
        insert!(self, "no_cache", no_cache);
        self
    }
    /// Remove intermediate containers after a successful build.
    pub fn rm(&mut self, rm: bool) -> &mut Self {
        insert!(self, "rm", rm);
        self
    }
    /// Always remove intermediate containers, even upon failure.
    pub fn forcerm(&mut self, force: bool) -> &mut Self {
        insert!(self, "forcerm", force);
        self
    }
    /// Set memory limit for build.
    pub fn memory(&mut self, limit: u64) -> &mut Self {
        insert!(self, "memory", limit);
        self
    }
    /// Total memory (memory + swap). Set as -1 to disable swap.
    pub fn mem_swap(&mut self, swap: u64) -> &mut Self {
        insert!(self, "memswap", swap);
        self
    }
    /// CPU shares (relative weight).
    pub fn cpu_shares(&mut self, shares: u64) -> &mut Self {
        insert!(self, "cpushares", shares);
        self
    }
    /// CPUs in which to allow execution (e.g., 0-3, 0,1).
    pub fn cpusetcpus(&mut self, setcpus: String) -> &mut Self {
        insert!(self, "cpusetcpus", setcpus);
        self
    }
    /// The length of a CPU period in microseconds.
    pub fn cpu_period(&mut self, period: u64) -> &mut Self {
        insert!(self, "cpuperiod", period);
        self
    }
    /// Microseconds of CPU time that the container can get in a CPU period.
    pub fn cpu_quota(&mut self, quota: u64) -> &mut Self {
        insert!(self, "cpuquota", quota);
        self
    }
    /// JSON map of string pairs for build-time variables. Users pass these values at build-time.  
    /// Docker uses the buildargs as the environment context for commands run via the Dockerfile RUN instruction, or for variable expansion in other Dockerfile instructions. This is not meant for passing secret values.
    pub fn build_args(&mut self, args: HashMap<&str, &str>) -> &mut Self {
        insert!(self, "build_args", args);
        self
    }
    /// Size of /dev/shm in bytes. The size must be greater than 0. If omitted the system uses 64MB.
    pub fn shmsize(&mut self, size: u64) -> &mut Self {
        insert!(self, "shmsize", size);
        self
    }
    /// Arbitrary key/value labels to set on the image
    pub fn labels(&mut self, labels: HashMap<&str, &str>) -> &mut Self {
        insert!(self, "labels", labels);
        self
    }
    /// Sets the networking mode for the run commands during build. Supported standard values are: bridge, host, none, and container:<name|id>.  
    /// Any other value is taken as a custom network's name to which this container should connect to.
    pub fn network_mode(&mut self, mode: &str) -> &mut Self {
        insert!(self, "networkmode", mode);
        self
    }
    /// Platform in the format os[/arch[/variant]]
    pub fn platform(&mut self, p: &str) -> &mut Self {
        insert!(self, "platform", p);
        self
    }
    /// Target build stage
    pub fn target(&mut self, t: &str) -> &mut Self {
        insert!(self, "target", t);
        self
    }
}

/// Options for creating image
#[derive(Default)]
pub struct CreateImageOpts {
    opts: HashMap<&'static str, Value>,
    auth: AuthOpts,
}
impl CreateImageOpts {
    pub fn new() -> Self {
        CreateImageOpts::default()
    }
    /// Name of the image to pull. The name may include a tag or digest. This parameter may only be used when pulling an image. The pull is cancelled if the HTTP connection is closed.
    pub fn from_image(&mut self, from_image: &str) -> &mut Self {
        insert!(self, "fromImage", from_image);
        self
    }
    /// Source to import. The value may be a URL from which the image can be retrieved or - to read the image from the request body. This parameter may only be used when importing an image.
    pub fn from_src(&mut self, from_src: &str) -> &mut Self {
        insert!(self, "fromSrc", from_src);
        self
    }
    /// Repository name given to an image when it is imported. The repo may include a tag. This parameter may only be used when importing an image.
    pub fn repo(&mut self, repo: &str) -> &mut Self {
        insert!(self, "repo", repo);
        self
    }
    /// Tag or digest. If empty when pulling an image, this causes all tags for the given image to be pulled.
    pub fn tag(&mut self, tag: &str) -> &mut Self {
        insert!(self, "tag", tag);
        self
    }
    /// Platform in the format os[/arch[/variant]]
    pub fn platform(&mut self, platform: &str) -> &mut Self {
        insert!(self, "platform", platform);
        self
    }
    pub fn username(&mut self, username: &str) -> &mut Self {
        self.auth.username(username);
        self
    }
    pub fn password(&mut self, password: &str) -> &mut Self {
        self.auth.password(password);
        self
    }
    pub fn email(&mut self, email: &str) -> &mut Self {
        self.auth.email(email);
        self
    }
    /// Url to docker registry
    /// f.e. hub.docker.com
    pub fn server_address(&mut self, server_address: &str) -> &mut Self {
        self.auth.server_address(server_address);
        self
    }
    pub(crate) fn auth_ref(&self) -> &AuthOpts {
        &self.auth
    }
    pub(crate) fn set_auth(&mut self, auth: &AuthOpts) -> &mut Self {
        self.auth = auth.clone();
        self
    }
}

/// Options for authentication
#[derive(Clone, Default)]
pub struct AuthOpts {
    opts: HashMap<&'static str, Value>,
}

impl AuthOpts {
    pub fn new() -> Self {
        AuthOpts::default()
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
    pub fn serialize(&self) -> Result<String, Error> {
        Ok(base64::encode(&serde_json::to_string(&self.opts)?))
    }
}

/// Options for executing commands
#[derive(Clone, Default)]
pub struct ExecOpts {
    opts: HashMap<&'static str, Value>,
}

impl ExecOpts {
    pub fn new() -> Self {
        ExecOpts::default()
    }
    /// Attach to stdin of the exec command.
    pub fn attach_stdin(&mut self, attach: bool) -> &mut Self {
        insert!(self, "AttachStdin", attach);
        self
    }
    /// Attach to stdout of the exec command.
    pub fn attach_stdout(&mut self, attach: bool) -> &mut Self {
        insert!(self, "AttachStdout", attach);
        self
    }
    /// Attach to stderr of the exec command.
    pub fn attach_stderr(&mut self, attach: bool) -> &mut Self {
        insert!(self, "AttachStderr", attach);
        self
    }
    /// Detach from the command.
    pub fn detach(&mut self, detach: bool) -> &mut Self {
        insert!(self, "detach", detach);
        self
    }
    /// Allocate a pseudo-TTY.
    pub fn tty(&mut self, tty: bool) -> &mut Self {
        insert!(self, "Tty", tty);
        self
    }
    /// A list of environment variables in the form ["VAR=value", ...].
    pub fn env(&mut self, env: &[String]) -> &mut Self {
        insert!(self, "Env", env);
        self
    }
    /// Command to run, as a string or array of strings.
    pub fn cmd(&mut self, cmd: &[String]) -> &mut Self {
        insert!(self, "Cmd", cmd);
        self
    }
    /// Runs the exec process with extended privileges.
    pub fn privileged(&mut self, allow: bool) -> &mut Self {
        insert!(self, "Privileged", allow);
        self
    }
    /// The user, and optionally, group to run the exec process inside the container.  
    /// Format is one of: user, user:group, uid, or uid:gid.
    pub fn user(&mut self, user: &str) -> &mut Self {
        insert!(self, "User", user);
        self
    }
    /// The working directory for the exec process inside the container.
    pub fn working_dir(&mut self, dir: &str) -> &mut Self {
        insert!(self, "WorkingDir", dir);
        self
    }
    pub(crate) fn _tty(&self) -> bool {
        if let Some(tty) = self.opts.get("Tty") {
            serde_json::from_value(tty.clone()).unwrap()
        } else {
            false
        }
    }
    pub(crate) fn _detach(&self) -> bool {
        if let Some(detach) = self.opts.get("detach") {
            serde_json::from_value(detach.clone()).unwrap()
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::str;

    #[test]
    fn upload_archive_opts_work() {
        let _ = ();
        let mut query = HashMap::new();
        query.insert("path", serde_json::to_value("/example/path").unwrap());
        query.insert(
            "noOverwriteDirNonDir",
            serde_json::to_value("true").unwrap(),
        );
        query.insert("copyUIDGID", serde_json::to_value("true").unwrap());

        let mut opts = UploadArchiveOpts::new();
        opts.path("/example/path")
            .no_overwrite("true")
            .copy_uid_gid("true");

        opts.opts
            .iter()
            .map(|(k, v)| {
                let val = query.get(k);
                assert!(val.is_some());
                assert_eq!(val.unwrap(), v);
            })
            .collect()
    }
    #[test]
    fn list_container_opts_work() {
        let mut query = HashMap::new();
        query.insert("all", serde_json::to_value(&true).unwrap());
        query.insert("size", serde_json::to_value(&true).unwrap());
        query.insert("limit", serde_json::to_value(&10000).unwrap());
        query.insert("filters", "".into());

        let mut opts = ListContainersOpts::new();
        opts.all(true).size(true).limit(10000).filters("");

        opts.opts
            .iter()
            .map(|(k, v)| {
                let val = query.get(k);
                assert!(val.is_some());
                assert_eq!(val.unwrap(), v);
            })
            .collect()
    }
    #[test]
    fn rm_container_opts_work() {
        let mut query = HashMap::new();
        query.insert("volumes", serde_json::to_value(&true).unwrap());
        query.insert("force", serde_json::to_value(&false).unwrap());
        query.insert("link", serde_json::to_value(&true).unwrap());

        let mut opts = RmContainerOpts::new();
        opts.volumes(true).force(false).link(true);

        opts.opts
            .iter()
            .map(|(k, v)| {
                let val = query.get(k);
                assert!(val.is_some());
                assert_eq!(val.unwrap(), v);
            })
            .collect()
    }
    #[test]
    fn container_builder_opts_work() {
        let mut labels = HashMap::new();
        labels.insert("test", "label");

        let body = json!({
            "Hostname": "test_hostname",
            "DomainName": "test_domain_name",
            "User": "test_user",
            "AttachStdin": false,
            "AttachStdout": true,
            "AttachStderr": true,
            "Tty": false,
            "OpenStdin": true,
            "StdinOnce": false,
            "Env": ["ENV=Vars", "FLAG=test"],
            "Cmd": ["/bin/bash"],
            "ArgsEscaped": false,
            "Image": "alpine",
            "WorkingDir": "/home/test",
            "Entrypoint": [""],
            "NetworkDisabled": false,
            "MacAddress": "39:74:C9:17:87:9F",
            "OnBuild": [""],
            "StopSignal": "SIGSTOP",
            "Shell": [""],
            "Labels": {
                "test": "label"
            },
            "HostConfig.PortBindings": {
                "22/tcp": null,
                "443/tcp": null
            },
            "HostConfig.Binds": {
                "/home/host/path:/home/container/path": null
            },
            "HostConfig.Memory": 1000000,
            "HostConfig.NetworkMode": "bridge",
        });

        let mut opts = ContainerBuilderOpts::new();
        opts.hostname("test_hostname")
            .domain_name("test_domain_name")
            .user("test_user")
            .attach_stdin(false)
            .attach_stdout(true)
            .attach_stderr(true)
            .tty(false)
            .open_stdin(true)
            .stdin_once(false)
            .env(&["ENV=Vars", "FLAG=test"])
            .cmd(&["/bin/bash"])
            .args_escaped(false)
            .image("alpine")
            .working_dir("/home/test")
            .entrypoint(&[""])
            .network_disabled(false)
            .mac_address("39:74:C9:17:87:9F")
            .on_build(&[""])
            .stop_signal("SIGSTOP")
            .shell(&[""])
            .labels(&labels)
            .exposed_ports(&["22/tcp", "443/tcp"])
            .volumes(&["/home/host/path:/home/container/path"])
            .memory(1000000)
            .network_mode("bridge");

        opts.opts
            .iter()
            .map(|(k, v)| {
                let val = body.get(k);
                assert!(val.is_some());
                assert_eq!(val.unwrap(), v);
            })
            .collect()
    }
    #[test]
    fn auth_opts_work() {
        let opts_j = json!({
            "username": "user",
            "password": "pass",
            "email": "email@random.co",
            "serveraddress": "http://0.0.0.0:666"
        });

        let mut opts = AuthOpts::new();
        opts.username("user")
            .password("pass")
            .email("email@random.co")
            .server_address("http://0.0.0.0:666");

        let _ = opts
            .opts
            .iter()
            .map(|(k, v)| {
                let val = opts_j.get(k);
                assert!(val.is_some());
                assert_eq!(val.unwrap(), v);
            })
            .collect::<()>();

        let serialized = opts.serialize().unwrap();
        let decoded = base64::decode(&serialized).unwrap();
        let deserialized: HashMap<&str, Value> =
            serde_json::from_str(str::from_utf8(&decoded).unwrap()).unwrap();

        opts.opts
            .iter()
            .map(|(k, v)| {
                let val = deserialized.get(k);
                assert!(val.is_some());
                assert_eq!(val.unwrap(), v);
            })
            .collect()
    }
    #[test]
    fn create_image_opts_work() {
        let mut query: HashMap<&str, Value> = HashMap::new();
        query.insert("fromImage", "alpine".into());
        query.insert("fromSrc", "-".into());
        query.insert("repo", "repo".into());
        query.insert("tag", "tag".into());

        let mut opts = CreateImageOpts::new();
        opts.from_image("alpine")
            .from_src("-")
            .repo("repo")
            .tag("tag");

        opts.opts
            .iter()
            .map(|(k, v)| {
                let val = query.get(k);
                assert!(val.is_some());
                assert_eq!(val.unwrap(), v);
            })
            .collect()
    }
    #[test]
    fn exec_opts_work() {
        let opts_j = json!({
            "AttachStdin": false,
            "AttachStdout": true,
            "AttachStderr": true,
            "Tty": true,
            "Env": ["TEST=var"],
            "Cmd": ["/bin/echo", "this definitely works"],
            "Privileged": false,
            "User": "test_user",
            "WorkingDir": "/tmp/dir"
        });

        let mut opts = ExecOpts::new();
        opts.attach_stdin(false)
            .attach_stdout(true)
            .attach_stderr(true)
            .tty(true)
            .env(&["TEST=var".into()])
            .cmd(&["/bin/echo".into(), "this definitely works".into()])
            .privileged(false)
            .user("test_user")
            .working_dir("/tmp/dir");

        opts.opts
            .iter()
            .map(|(k, v)| {
                let val = opts_j.get(k);
                assert!(val.is_some());
                assert_eq!(val.unwrap(), v);
            })
            .collect()
    }
}
