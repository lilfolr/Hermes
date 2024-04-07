use anyhow::Result;
use shared_model::device_info;

pub fn get_device_info(
    request: device_info::DeviceInfoRequest,
) -> Result<device_info::DeviceInfoResponse> {
    let response = device_info::DeviceInfoResponse {
        up_time: 2,
        request_id: request.request_id,
    };
    return Ok(response);
}
