use anyhow::Result;
use log::{error, info, warn};
use shared_model::{ControlRequest, ControlResponse};
use thiserror::Error;

use super::device_info::get_device_info;

#[derive(Error, Debug)]
pub enum CommandProcessingError {
    #[error("The input supplied to this command is invalid")]
    InvalidInput(String),

    #[error("An unknown error occured execiting the command")]
    ExecutionError(String),
}

pub async fn command_router(input_message: &str) -> Result<String> {
    let raw_command_request = serde_json::from_str(input_message).map_err(|e| {
        warn!("Invalid input {} - {}", input_message, e);
        CommandProcessingError::InvalidInput(format!("Unknown input: {}", input_message))
    })?;
    let response: Result<ControlResponse, CommandProcessingError> = match raw_command_request {
        ControlRequest::DeviceInfo(request) => get_device_info(request)
            .await
            .map_err(|e| {
                error!("Error executing command - {}", e);
                CommandProcessingError::ExecutionError(format!("Error execiting command"))
            })
            .map(|r| ControlResponse::DeviceInfo(r)),
    };

    info!("Response: {response:?}");
    let string_response = serde_json::to_string(&response?)?;
    Ok(string_response)
}
