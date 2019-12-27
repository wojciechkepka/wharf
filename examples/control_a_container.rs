use failure::Error;
use std::fs;
use wharf::{opts::UploadArchiveOpts, Docker};

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    // create docker api instance
    let d = Docker::new("http://0.0.0.0:2376/")?;
    // get handle to a specific container
    let mut c = d.container("container_name");
    c.start().await?;

    let mut opts = UploadArchiveOpts::new();
    opts.path("/path/in/container/")
        .no_overwrite(false)
        .copy_uid_gid(true);
    let archive = fs::read("/path/to/some/archive.tar")?;
    c.upload_archive(&archive, &opts).await?;
    c.rename("new_name").await?;

    println!("{:?}", c.inspect().await?);
    c.stop().await?;

    Ok(())
}
