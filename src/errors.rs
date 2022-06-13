use aws_sigv4::signing_params::BuildError;
use std::fmt::{self};
use thiserror::Error;
use url::ParseError;

#[derive(Error)]
pub enum ESDumpError {
    #[error("Error while signing")]
    ErrorRequestSign(#[from] http::Error),
    #[error("Error while signed request building")]
    ErrorRequestSignBuild(#[from] BuildError),
    #[error("Error while signed request building")]
    ErrorSigning(#[from] aws_sigv4::http_request::Error),
    #[error("Error while creating request")]
    ErrorHttp(#[from] reqwest::Error),
    #[error("Error while creating request")]
    ErrorSerde(#[from] serde_json::Error),
    #[error("Error while creating request")]
    ErrorIO(#[from] std::io::Error),
    #[error("Url parsing Error")]
    ParseError(#[from] ParseError),
    #[error("`{0}`")]
    CustomError(String),
}

impl fmt::Debug for ESDumpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ESDumpError::ErrorRequestSign(err) => write!(f, "{}", err),
            ESDumpError::ErrorRequestSignBuild(err) => write!(f, "{}", err),
            ESDumpError::ErrorSigning(err) => write!(f, "{}", err),
            ESDumpError::ErrorHttp(err) => write!(f, "{}", err),
            ESDumpError::ErrorSerde(err) => write!(f, "{}", err),
            ESDumpError::ErrorIO(err) => write!(f, "{}", err),
            ESDumpError::CustomError(err) => write!(f, "{}", err),
            ESDumpError::ParseError(err) => write!(f, "{}", err),
        }
    }
}

pub type ESDumpResult<T> = Result<T, ESDumpError>;
