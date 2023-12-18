use cosmwasm_std::StdError;

pub enum CrowdfundingError {
    Generic(String),
    Unauthorized,
    ProjectNotFound,
    InvalidFundsAmount,
    SerializationError(serde_json::Error),
    DeserializationError(serde_json::Error),
}

impl From<CrowdfundingError> for StdError {
    fn from(val: CrowdfundingError) -> Self {
        match val {
            CrowdfundingError::Generic(msg) => StdError::generic_err(msg),
            CrowdfundingError::Unauthorized => StdError::generic_err("Unauthorized"),
            CrowdfundingError::ProjectNotFound => StdError::generic_err("Project not found"),
            CrowdfundingError::InvalidFundsAmount => StdError::generic_err("Invalid funds amount"),
            CrowdfundingError::SerializationError(err) => {
                StdError::generic_err(format!("Serialization error: {}", err))
            }
            CrowdfundingError::DeserializationError(err) => {
                StdError::generic_err(format!("Deserialization error: {}", err))
            }
        }
    }
}
