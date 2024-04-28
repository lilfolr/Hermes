use anyhow::Result;
use k8s_openapi::api::core::v1::Pod as K_Pod;
use kube::{Api, Client};
use shared_model::device_info::Pod;

use super::util::fetch_page;

pub async fn list_pods_ns(kube_client: Client, namespace: &str) -> Result<Vec<Pod>> {
    let api = Api::<K_Pod>::namespaced(kube_client, namespace);
    let results = fetch_page(&api, None).await?;
    let pod_results: Vec<Pod> = results
        .iter()
        .filter_map(|p| Pod::try_from(p.clone()).ok())
        .collect();

    Ok(pod_results)
}
pub async fn list_pods_all_ns(kube_client: Client) -> Result<Vec<Pod>> {
    let api = Api::<K_Pod>::all(kube_client);
    let results = fetch_page(&api, None).await?;
    let pod_results: Vec<Pod> = results
        .iter()
        .filter_map(|p| Pod::try_from(p.clone()).ok())
        .collect();

    Ok(pod_results)
}
