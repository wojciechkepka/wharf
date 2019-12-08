use crate::Docker;
use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImagesJson {
    Id: String,
    ParentId: String,
    RepoTags: Vec<String>,
    RepoDigests: Vec<String>,
    Created: u64,
    Size: isize,
    VirtualSize: isize,
    SharedSize: isize,
    Labels: Value,
    Containers: isize,
}

pub struct Images<'d> {
    docker: &'d Docker,
}
impl<'d> Images<'d> {
    pub fn new(docker: &'d Docker) -> Self {
        Images { docker }
    }
    pub async fn list(&self) -> Result<Vec<ImagesJson>, Error> {
        let res = self
            .docker
            .client
            .get(self.docker.url.join("images/json")?)
            .send()
            .await?;
        let body = res.text().await?;
        Ok(serde_json::from_str(&body)?)
    }
}
