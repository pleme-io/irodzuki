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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_color_display_carries_payload() {
        let e = IrodzukiError::InvalidColor("ZZZZZZ".into());
        assert_eq!(e.to_string(), "invalid color: ZZZZZZ");
    }

    #[test]
    fn parse_display_carries_payload() {
        let e = IrodzukiError::Parse("bad yaml".into());
        assert_eq!(e.to_string(), "scheme parsing failed: bad yaml");
    }

    #[test]
    fn missing_slot_display_carries_payload() {
        let e = IrodzukiError::MissingSlot("base10".into());
        assert_eq!(e.to_string(), "missing base16 slot: base10");
    }

    #[test]
    fn error_kinds_discriminate() {
        // Each variant must be matchable without collapsing to a
        // shared arm. Guards future refactors against a lazy
        // Box<dyn Error> collapse.
        let invalid = IrodzukiError::InvalidColor("x".into());
        let parse = IrodzukiError::Parse("x".into());
        let missing = IrodzukiError::MissingSlot("x".into());
        for e in [invalid, parse, missing] {
            match e {
                IrodzukiError::InvalidColor(_)
                | IrodzukiError::Parse(_)
                | IrodzukiError::MissingSlot(_) => {}
            }
        }
    }
}
