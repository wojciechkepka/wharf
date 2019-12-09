# Wharf ⚓🦀
Fully asynchronus docker api library written in Rust.
## Examples
```rust
use failure::Error;
use wharf::Docker;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // create docker api instance
    let d = Docker::new("http://0.0.0.0:2376")?;
    // get containers api handle from d
    let containers = d.containers();
    // iterate over containers
    for container in containers.list().await? {
        // access container metadata
        println!("{:?}", container.data().unwrap());
        // manipulate container
        container.stop().await?;
        container.start().await?;
        container.rename("alpine1").await?;
    }
    Ok(())
}
```
## Current TODO
- [ ] fix uploading archives to containers
- [ ] Creating containers
- [ ] Attaching to containers
- [ ] Getting logs from containers
- [ ] implement all images interface
- [ ] implement all networks interface
## Currently working api
- Containers
  - [x] listing
  - [x] starting
  - [x] stopping
  - [x] restarting
  - [x] killing
  - [x] unpausing
  - [x] pausing
  - [x] renaming
  - [x] archiving path
  - [x] file info
  - [x] ps
  - [ ] create
  - [ ] uploading archives
  - [ ] logs
  - [ ] attaching
- Images
  - [x] listing 
- Networks
  - [x] listing
## License
[MIT](https://github.com/wojciechkepka/wharf/blob/master/LICENSE)
