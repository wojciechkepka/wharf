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
