use anyhow::Result;
use log::info;
use shared_model::device_info;

use crate::kubernetes::{KubeImpl, KubeInterface};

pub async fn get_device_info(
    request: device_info::DeviceInfoRequest,
) -> Result<device_info::DeviceInfoResponse> {
    info!("Start device info");
    let kube = KubeImpl::new().await.unwrap();

    let pods = kube.list_pods_all_namespaces().await;
    info!("Got pods");
    let response = device_info::DeviceInfoResponse {
        up_time: 2,
        request_id: request.request_id,
        pods: pods.ok(),
    };

    Ok(response)
}
