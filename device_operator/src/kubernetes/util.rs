use anyhow::Result;
use k8s_openapi::{serde::de::DeserializeOwned, ListableResource};
use kube::{api::ListParams, Api};
use log::debug;
use std::fmt::Debug;

const PAGE_SIZE: u32 = 20;

pub async fn fetch_page<T>(api: &Api<T>, continue_token: Option<String>) -> Result<Vec<T>>
where
    T: ListableResource + Clone + DeserializeOwned + Debug,
{
    let mut lp = ListParams::default().limit(PAGE_SIZE);
    if let Some(token) = continue_token {
        lp = lp.continue_token(&token);
    }

    let resources = api.list(&lp).await?;
    let mut return_vec = Vec::new();
    for r in resources {
        debug!("Found Resource: {r:?}");
        return_vec.push(r);
    }

    Ok(return_vec)
}
