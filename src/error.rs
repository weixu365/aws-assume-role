use aws_sdk_sts::operation::assume_role::AssumeRoleError;
use aws_smithy_runtime_api::{client::result::SdkError, http::Response};
use std::{fmt, time::SystemTimeError};
use thiserror::Error;

#[derive(Error)]
pub enum AppError {
    #[error("Utf8 Error: {0}")]
    NulError(#[from] std::ffi::NulError),

    #[error("Assume role error: {0}")]
    AssumeRoleError(#[from] SdkError<AssumeRoleError, Response>),

    #[error("Ini Error: {0}")]
    IniErr(#[from] ini::Error),

    #[error("io error: {0}")]
    StdErr(#[from] std::io::Error),

    #[error("SystemTimeError: {0}")]
    SystemTimeErr(#[from] SystemTimeError),
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NulError(source) => write!(f, "{}", source),
            AppError::AssumeRoleError(source) => match source {
                SdkError::ServiceError(context) => {
                    let meta = context.err().meta();
                    write!(f, "{}", meta)
                }
                err @ _ => write!(f, "{}", err),
            },
            AppError::IniErr(source) => write!(f, "{}", source),
            AppError::StdErr(source) => write!(f, "{}", source),
            AppError::SystemTimeErr(source) => write!(f, "{}", source),
        }
    }
}
