use anyhow::Result;
use shared_model::ControlRequest;

use super::device_info::get_device_info;

pub fn command_router(input_message: &str) -> Result<()> {
    let raw_command_request = serde_json::from_str(input_message);
    if raw_command_request.is_err() {
        println!("Invalid input: {}", input_message);
    }
    match raw_command_request.unwrap() {
        ControlRequest::DeviceInfo(request) => {
            get_device_info(request).unwrap();
        }
    }

    return Ok(());
}
