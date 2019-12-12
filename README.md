# Wharf ⚓🦀
Fully asynchronous docker api library written in Rust.
## Examples
```rust
use failure::Error;
use wharf::Docker;
use wharf::opts::{ContainerBuilderOpts, ListContainersOpts};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // create docker api instance
    let d = Docker::new("http://0.0.0.0:2376")?;
    // get containers api handle from d
    let containers = d.containers();
    // Create instance of query options
    let mut opts = ListContainersOpts::new();
    opts.all(true);
    // iterate over containers
    for container in containers.list(opts).await? {
        // access container metadata
        println!("{:?}", container.data().unwrap());
        // manipulate container
        container.stop().await?;
        container.start().await?;
        container.rename("alpine1").await?;
    }
    // Create a container
    let mut container_opts = ContainerBuilderOpts::new();
    container_opts
	.image("ubuntu")
	.cmd(&["/bin/echo".into(), "hello".into()])
	.env(&["HTTPS_PROXY=proxy.domain.com:1337"]);

    containers.create("jimmy-falcon", &container_opts).await?;


    Ok(())
}
```
## Current TODO
- [ ] fix uploading archives to containers
- [ ] Attaching to containers
- [ ] Getting logs from containers
- [ ] allow connection over unix socket
- [ ] implement all images interface
- [ ] implement all networks interface
- [ ] figure out a way to write tests
## Currently working api
- Containers
  - [x] listing
  - [x] starting
  - [x] stopping
  - [x] restarting
  - [x] inspecting
  - [x] killing
  - [x] unpausing
  - [x] pausing
  - [x] renaming
  - [x] archiving path
  - [x] file info
  - [x] ps
  - [x] create
  - [ ] uploading archives
  - [ ] logs
  - [ ] attaching
- Images
  - [x] pull
  - [x] listing 
- Networks
  - [x] listing
## License
[MIT](https://github.com/wojciechkepka/wharf/blob/master/LICENSE)
