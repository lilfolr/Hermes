use device_info::{DeviceInfoRequest, DeviceInfoResponse};
use serde::{Deserialize, Serialize};

pub mod device_info;

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
    DeviceInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "operation")]
pub enum ControlRequest {
    DeviceInfo(DeviceInfoRequest),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "operation")]
pub enum ControlResponse {
    DeviceInfo(DeviceInfoResponse),
}
