use failure::Error;
use std::collections::HashMap;
use std::env;
use wharf::{opts::ContainerBuilderOpts, Docker};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "wharf=trace");
    pretty_env_logger::init();
    // create docker api instance
    let d = Docker::new("http://0.0.0.0:2376/")?;
    // get handle to containers
    let c = d.containers();

    let mut labels = HashMap::new();
    labels.insert("id", "8927323891274986127");

    // Configure a container
    let mut opts = ContainerBuilderOpts::new();
    opts.image("ubuntu:latest")
        .tty(true)
        .shell(&["/bin/bash".into()])
        .cmd(&["/bin/echo", "test"])
        .labels(&labels)
        .env(&["HTTP_PROXY=http://proxy.domain.com"]);

    // Create a container
    c.create("container_name", &opts).await?;

    Ok(())
}
