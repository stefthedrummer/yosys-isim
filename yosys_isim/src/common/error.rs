use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimError {
    #[error("IOError")]
    IOError(#[from] std::io::Error),
    #[error("SerdeError")]
    SerdeError(#[from] serde_json::Error),
    #[error("JsonError [{msg:?}]")]
    JsonError { msg: String },
    #[error("SimError [{msg:?}]")]
    SimError { msg: String },
}
