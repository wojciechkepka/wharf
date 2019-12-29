use failure::Error;
use tokio::io::AsyncReadExt;
use wharf::opts::*;
use wharf::Docker;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    let d = Docker::new("http://0.0.0.0:2376")?;

    // Initialize a container
    let x = d.container("zen_dubinsky");
    // Initialize options for attaching
    let mut opts = AttachOpts::new();
    // stream the stdout
    opts.stream(true).stdout(true);

    match x.attach(&opts).await {
        Ok(mut upgraded) => {
            // Read the upgraded stream byte by byte
            loop {
                let mut buf = [0; 1];
                upgraded.read(&mut buf).await?;
                print!("{}", buf[0] as char);
            }
        }
        Err(e) => eprintln!("Failed to attach to a container - {:?}", e),
    }

    Ok(())
}
