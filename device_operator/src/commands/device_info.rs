use anyhow::Result;
use shared_model::device_info;

use crate::kubernetes::pod::get_pods;

pub async fn get_device_info(
    request: device_info::DeviceInfoRequest,
) -> Result<device_info::DeviceInfoResponse> {

    let pods = get_pods().await;

    let response = device_info::DeviceInfoResponse {
        up_time: 2,
        request_id: request.request_id,
        pods: pods.ok()
    };
    return Ok(response);
}
