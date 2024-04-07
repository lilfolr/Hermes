use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInfoRequest {
    pub request_id: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInfoResponse {
    pub up_time: i8,
    pub request_id: Option<String>
}
