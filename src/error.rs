#[derive(thiserror::Error, Debug)]
pub enum IrodzukiError {
    #[error("invalid color: {0}")]
    InvalidColor(String),

    #[error("scheme parsing failed: {0}")]
    Parse(String),

    #[error("missing base16 slot: {0}")]
    MissingSlot(String),
}

pub type Result<T> = std::result::Result<T, IrodzukiError>;
