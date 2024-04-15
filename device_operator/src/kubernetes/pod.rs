use anyhow::Result;
use k8s_openapi::api::core::v1::Pod as K_Pod;
use kube::{api::ListParams, Api, Client, ResourceExt};
use log::info;
use shared_model::device_info::Pod;


const PAGE_SIZE :u32 = 20;

pub async fn get_pods() -> Result<Vec<Pod>> {

    let client = Client::try_default().await?;
    let api = Api::<K_Pod>::default_namespaced(client);

    let results = fetch_page(&api, None).await?;
    let pod_results : Vec<Pod> = results.iter().filter_map(|p| Pod::try_from(p.clone()).ok()).collect();
    
    Ok(pod_results)
}

async fn fetch_page(api: &Api<K_Pod>, continue_token: Option<String>) -> Result<Vec<K_Pod>> {
    let mut lp = ListParams::default().limit(PAGE_SIZE);
    if let Some(token) = continue_token {
        lp = lp.continue_token(&token);
    }

    let pods = api.list(&lp).await?;
    // let continue_token = pods.metadata.continue_.clone();
    let mut pods_vec = Vec::new();
    for p in pods {
        let pod_name = p.name_any();
        pods_vec.push(p);
        info!("Found Pod: {}", pod_name);
    }

    Ok(pods_vec)
}
