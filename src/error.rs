use thiserror::Error;

/// Error types for the Nitrolite library
#[derive(Error, Debug)]
pub enum NitroliteError {
    /// Error occurred during signature operation
    #[error("Signature error: {0}")]
    SignatureError(String),
    
    /// Error occurred during crypto operation
    #[error("Crypto error: {0}")]
    CryptoError(String),
    
    /// Invalid input data
    #[error("Invalid data: {0}")]
    InvalidData(String),
}