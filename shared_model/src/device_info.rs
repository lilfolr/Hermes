use std::u32;

use k8s_openapi::{api::core::v1::Pod as K_Pod, Metadata};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pod {
    namespace: String,
    name: String,
    status: String,
    running_containers: u32,
    total_containers: u32,
}

const UNKNOWN_STRING_FIELD: &str = "Unknown";
const UNKNOWN_INT_FIELD: usize = 0;

impl TryFrom<K_Pod> for Pod {
    type Error = &'static str;

    fn try_from(value: K_Pod) -> Result<Self, Self::Error> {
        let pod_spec = &value.spec;
        let pod_status = &value.status;
        let pod_metadata = value.metadata();

        let namespace: String = pod_metadata
            .namespace
            .clone()
            .unwrap_or(UNKNOWN_STRING_FIELD.to_string());
        let name: String = pod_metadata
            .name
            .clone()
            .unwrap_or(UNKNOWN_STRING_FIELD.to_string());
        let status: String = pod_status
            .clone()
            .and_then(|s| s.phase)
            .unwrap_or(UNKNOWN_STRING_FIELD.to_string());
        let total_containers: u32 = u32::try_from(
            pod_spec
                .clone()
                .and_then(|p| Some(p.containers.len()))
                .unwrap_or(UNKNOWN_INT_FIELD),
        )
        .unwrap_or(0);
        let running_containers: u32 = u32::try_from(
            pod_status
                .clone()
                .and_then(|p| {
                    p.container_statuses
                        .and_then(|c| Some(c.iter().filter(|c| c.ready).count()))
                })
                .unwrap_or(UNKNOWN_INT_FIELD),
        )
        .unwrap_or(0);

        Ok(Self {
            namespace,
            name,
            status,
            total_containers,
            running_containers,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInfoRequest {
    pub request_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInfoResponse {
    pub up_time: i8,
    pub request_id: Option<String>,
    pub pods: Option<Vec<Pod>>,
}
