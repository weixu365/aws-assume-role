use aws_sdk_sts::error::AssumeRoleError;
use aws_smithy_http::result::SdkError;
use std::{fmt, time::SystemTimeError};
use thiserror::Error;

#[derive(Error)]
pub enum AppError {
    #[error("Utf8 Error")]
    NulError(#[from] std::ffi::NulError),

    #[error("Assume role error: {0}")]
    AssumeRoleError(#[from] SdkError<AssumeRoleError>),

    #[error("Ini Error")]
    IniErr(#[from] ini::Error),

    #[error("Std Error")]
    StdErr(#[from] std::io::Error),

    #[error("System Time Error")]
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
