//! This crate provides a configuration module and error handling for missing configuration parameters.

/// The ways building a configuration can fail.
///
/// Each variant names one required setting that was not provided.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ConfigError {
    /// Error indicating that the issuer is missing in the configuration when build() is called.
    #[error("missing issuer in configuration")]
    MissingIssuer,

    /// Error indicating that the audience is missing in the configuration when build() is called.
    #[error("missing audience in configuration")]
    MissingAudience,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_issuer_should_display_a_clear_message() {
        let error = ConfigError::MissingIssuer;
        assert_eq!(error.to_string(), "missing issuer in configuration");
    }

    #[test]
    fn missing_audience_should_display_a_clear_message() {
        let error = ConfigError::MissingAudience;
        assert_eq!(error.to_string(), "missing audience in configuration");
    }
}
