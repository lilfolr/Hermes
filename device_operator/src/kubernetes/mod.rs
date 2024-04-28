use anyhow::{Context, Result};
use kube::Client;
use shared_model::device_info::Pod;

mod pod;
mod util;

pub trait KubeInterface: Send + Sync {
    fn get_kube_client(&self) -> Client;
    async fn list_pods_all_namespaces(&self) -> Result<Vec<Pod>>;
}

#[derive(Clone)]
pub struct KubeImpl {
    client: kube::Client,
}

impl KubeImpl {
    pub async fn new() -> Result<Self> {
        Ok(KubeImpl {
            client: Client::try_default()
                .await
                .context("Unable to setup k8s client")?,
        })
    }
}

impl KubeInterface for KubeImpl {
    fn get_kube_client(&self) -> Client {
        self.client.clone()
    }

    async fn list_pods_all_namespaces(&self) -> Result<Vec<Pod>> {
        pod::list_pods_all_ns(self.get_kube_client()).await
    }
}
