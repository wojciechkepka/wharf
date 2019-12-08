use serde_json::Value;
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
    pub fn path<T: Into<String>>(&mut self, path: T) {
        self.path = path.into();
    }
    /// If “1”, “true”, or “True” then it will be an error if unpacking the given content would cause an existing directory to be replaced with a non-directory and vice versa.
    pub fn no_overwrite<T: Into<String>>(&mut self, no_overwrite: T) {
        self.no_overwrite = no_overwrite.into();
    }
    /// If “1”, “true”, then it will copy UID/GID maps to the dest file or dir
    pub fn copy_uid_gid<T: Into<String>>(&mut self, copy_uid_gid: T) {
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

#[derive(Default)]
pub struct ContainerBuilderOpts {
    Hostname: String,
    Domainname: String,
    User: String,
    AttachStdin: bool,
    AttachStdout: bool,
    AttachStderr: bool,
    ExposedPorts: Value,
    Tty: bool,
    OpenStdin: bool,
    StdinOnce: bool,
    Env: Vec<String>,
    Cmd: Vec<String>,
    Healthcheck: Value,
    ArgsEscaped: bool,
    Image: String,
    Volumes: Value,
    WorkingDir: String,
    Entrypoint: Vec<String>,
    NetworkDisabled: bool,
    MacAddress: String,
    OnBuild: Vec<String>,
    Lables: Value,
    StopSignal: String,
    StopTimeout: i64,
    Shell: Vec<String>,
    HostConfig: Value,
    NetworkConfig: Value,
}
impl Query for ContainerBuilderOpts {
    fn to_query(self) -> Vec<(&'static str, String)> {
        vec![
            ("Hostname", self.Hostname),
            ("Domainname", self.Domainname),
            ("User", self.User),
            ("AttachStdin", self.AttachStdin.to_string()),
            ("AttachStdout", self.AttachStdout.to_string()),
            ("AttachStderr", self.AttachStderr.to_string()),
            ("ExposedPorts", self.ExposedPorts.to_string()),
            ("Tty", self.Tty.to_string()),
            ("OpenStdin", self.OpenStdin.to_string()),
            ("StdinOnce", self.StdinOnce.to_string()),
            ("Env", format!("{:?}", self.Env)),
            ("Cmd", format!("{:?}", self.Cmd)),
            ("Healthcheck", self.Healthcheck.to_string()),
            ("ArgsEscaped", self.ArgsEscaped.to_string()),
            ("Image", self.Image),
            ("Volumes", self.Volumes.to_string()),
            ("WorkingDir", self.WorkingDir),
            ("Entrypoint", format!("{:?}", self.Entrypoint)),
            ("NetworkDisabled", self.NetworkDisabled.to_string()),
            ("MacAddress", self.MacAddress),
            ("OnBuild", format!("{:?}", self.OnBuild)),
            ("Lables", self.Lables.to_string()),
            ("StopSignal", self.StopSignal),
            ("StopTimeout", self.StopTimeout.to_string()),
            ("Shell", format!("{:?}", self.Shell)),
            ("HostConfig", self.HostConfig.to_string()),
            ("NetworkConfig", self.NetworkConfig.to_string()),
        ]
    }
}
impl ContainerBuilderOpts {
    pub fn new() -> Self {
        ContainerBuilderOpts::default()
    }
    /// The hostname to use for the container, as a valid RFC 1123 hostname.
    pub fn hostname<S: Into<String>>(&mut self, hostname: S) {
        self.Hostname = hostname.into();
    }
    /// The domain name to use for the container.
    pub fn domain_name<S: Into<String>>(&mut self, domain_name: S) {
        self.Domainname = domain_name.into();
    }
    /// The user that commands are run as inside the container.
    pub fn User<S: Into<String>>(&mut self, user: S) {
        self.User = user.into();
    }
    /// Whether to attach to stdin.
    pub fn attach_stdin(&mut self, attach: bool) {
        self.AttachStdin = attach;
    }
    /// Whether to attach to stdout.
    pub fn attach_stdout(&mut self, attach: bool) {
        self.AttachStdout = attach;
    }
    /// Whether to attach to stderr.
    pub fn attach_stderr(&mut self, attach: bool) {
        self.AttachStderr = attach;
    }
    /// Attach standard streams to a TTY, including stdin if it is not closed.
    pub fn tty(&mut self, tty: bool) {
        self.Tty = tty;
    }
    /// Open stdin.
    pub fn open_stdin(&mut self, open: bool) {
        self.OpenStdin = open;
    }
    /// Close stdin after one attached client disconnects
    pub fn stdin_once(&mut self, stdin_once: bool) {
        self.StdinOnce = stdin_once;
    }
    /// A list of environment variables to set inside the container in the form ["VAR=value", ...].
    /// A variable without = is removed from the environment, rather than to have an empty value.
    pub fn Env(&mut self, env: &[String]) {
        self.Env = env.to_vec();
    }
    /// Command to run specified as a string or an array of strings.
    pub fn Cmd(&mut self, cmd: &[String]) {
        self.Cmd = cmd.to_vec();
    }
    /// Command is already escaped (Windows only)
    pub fn args_escaped(&mut self, escaped: bool) {
        self.ArgsEscaped = escaped;
    }
    /// The name of the image to use when creating the container
    pub fn image<S: Into<String>>(&mut self, image: S) {
        self.Image = image.into();
    }
    /// The working directory for commands to run in.
    pub fn working_dir<S: Into<String>>(&mut self, dir: S) {
        self.WorkingDir = dir.into();
    }
    /// The entry point for the container as a string or an array of strings.
    /// If the array consists of exactly one empty string ([""]) then the entry point is reset to system default
    /// (i.e., the entry point used by docker when there is no ENTRYPOINT instruction in the Dockerfile).
    pub fn entrypoint(&mut self, entrypoint: &[String]) {
        self.Entrypoint = entrypoint.to_vec();
    }
    /// Disable networking for the container.
    pub fn network_disabled(&mut self, disabled: bool) {
        self.NetworkDisabled = disabled;
    }
    /// MAC address of the container.
    pub fn mac_address<S: Into<String>>(&mut self, addr: S) {
        self.MacAddress = addr.into();
    }
    /// ONBUILD metadata that were defined in the image's Dockerfile.
    pub fn on_build(&mut self, md: &[String]) {
        self.OnBuild = md.to_vec();
    }
    /// Signal to stop a container as a string or unsigned integer.
    pub fn stop_signal<S: Into<String>>(&mut self, signal: S) {
        self.StopSignal = signal.into();
    }
    /// Timeout to stop a container in seconds.
    pub fn stop_timeout(&mut self, timeout: i64) {
        self.StopTimeout = timeout;
    }
    /// Shell for when RUN, CMD, and ENTRYPOINT uses a shell.
    pub fn shell(&mut self, s: &[String]) {
        self.Shell = s.to_vec();
    }
    pub fn exposed_ports(&mut self, _: Value) {}
    pub fn health_check(&mut self, _: Value) {}
    pub fn labels(&mut self, _: Value) {}
    pub fn host_config(&mut self, _: Value) {}
    pub fn network_config(&mut self, _: Value) {}
}
