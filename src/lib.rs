use thiserror::Error;

pub mod filter;
pub mod sort;

#[cfg(test)]
pub mod test;

type JsonValue = serde_json::Value;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Value error")]
    ValueError,
    #[error("Not able to Serialize")]
    SerializeError(#[from] serde_json::Error),
}
