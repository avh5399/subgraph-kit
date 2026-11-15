//! Authentication configuration.
//!
//! Use [`AuthConfigBuilder`] to set the issuer and audience, then call
//! [`AuthConfigBuilder::build`] to get a validated [`AuthConfig`]. Building
//! fails with an [`AuthConfigError`] when a required value is missing.

/// A validated authentication configuration.
///
/// An `AuthConfig` is created by [`AuthConfigBuilder::build`], so the issuer and
/// the audience are always set. Read them with the getters.
#[derive(Debug, PartialEq)]
pub struct AuthConfig {
    issuer: String,
    audience: String,
}

impl AuthConfig {
    /// Returns the OIDC issuer URL.
    pub fn issuer(&self) -> &str {
        &self.issuer
    }

    /// Returns the expected token audience.
    pub fn audience(&self) -> &str {
        &self.audience
    }
}

/// Builds an [`AuthConfig`] step by step.
///
/// Call the setters you need, then call [`build`](AuthConfigBuilder::build).
/// Required settings that were never set are reported when building.
///
/// # Examples
///
/// ```
/// use subgraph_kit::auth::AuthConfigBuilder;
///
/// let builder = AuthConfigBuilder::new()
///     .issuer("https://issuer.example.com/")
///     .audience("https://api.example.com");
/// ```
#[derive(Debug, Default)]
pub struct AuthConfigBuilder {
    issuer: Option<String>,
    audience: Option<String>,
}

impl AuthConfigBuilder {
    /// Creates a builder with nothing set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the OIDC issuer URL. Required.
    pub fn issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }

    /// Sets the expected token audience. Required.
    pub fn audience(mut self, audience: impl Into<String>) -> Self {
        self.audience = Some(audience.into());
        self
    }

    /// Validates the settings and returns the finished [`AuthConfig`].
    ///
    /// This consumes the builder.
    ///
    /// # Errors
    ///
    /// - [`AuthConfigError::MissingIssuer`] if no issuer was set.
    /// - [`AuthConfigError::MissingAudience`] if no audience was set. The issuer is
    ///   checked first, so it is reported when both are missing.
    ///
    /// # Examples
    ///
    /// ```
    /// use subgraph_kit::auth::AuthConfigBuilder;
    ///
    /// let config = AuthConfigBuilder::new()
    ///     .issuer("https://issuer.example.com/")
    ///     .audience("https://api.example.com")
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(config.issuer(), "https://issuer.example.com/");
    /// ```
    pub fn build(self) -> Result<AuthConfig, AuthConfigError> {
        let issuer = self.issuer.ok_or(AuthConfigError::MissingIssuer)?;
        let audience = self.audience.ok_or(AuthConfigError::MissingAudience)?;

        Ok(AuthConfig { issuer, audience })
    }
}

/// The ways building an [`AuthConfig`] can fail.
///
/// Each variant names one required setting that was not provided.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum AuthConfigError {
    /// Returned by [`AuthConfigBuilder::build`] when no issuer was set.
    #[error("missing issuer in configuration")]
    MissingIssuer,

    /// Returned by [`AuthConfigBuilder::build`] when no audience was set.
    #[error("missing audience in configuration")]
    MissingAudience,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_issuer_should_display_a_clear_message() {
        let error = AuthConfigError::MissingIssuer;
        assert_eq!(error.to_string(), "missing issuer in configuration");
    }

    #[test]
    fn missing_audience_should_display_a_clear_message() {
        let error = AuthConfigError::MissingAudience;
        assert_eq!(error.to_string(), "missing audience in configuration");
    }

    #[test]
    fn builder_should_store_issuer_when_set() {
        let builder = AuthConfigBuilder::new().issuer("https://example.com");
        assert_eq!(builder.issuer, Some("https://example.com".to_string()));
    }

    #[test]
    fn builder_should_store_audience_when_set() {
        let builder = AuthConfigBuilder::new().audience("my-audience");
        assert_eq!(builder.audience, Some("my-audience".to_string()));
    }

    #[test]
    fn build_should_return_config_when_issuer_and_audience_are_set() {
        let config = AuthConfigBuilder::new()
            .issuer("https://example.com")
            .audience("my-audience")
            .build()
            .unwrap();
        assert_eq!(config.issuer(), "https://example.com");
        assert_eq!(config.audience(), "my-audience");
    }

    #[test]
    fn build_should_fail_with_missing_issuer_when_issuer_is_not_set() {
        let error = AuthConfigBuilder::new()
            .audience("my-audience")
            .build()
            .unwrap_err();
        assert_eq!(error, AuthConfigError::MissingIssuer);
    }

    #[test]
    fn build_should_fail_with_missing_audience_when_audience_is_not_set() {
        let error = AuthConfigBuilder::new()
            .issuer("https://example.com")
            .build()
            .unwrap_err();
        assert_eq!(error, AuthConfigError::MissingAudience);
    }

    #[test]
    fn build_should_report_missing_issuer_first_when_both_are_missing() {
        let error = AuthConfigBuilder::new().build().unwrap_err();
        assert_eq!(error, AuthConfigError::MissingIssuer);
    }
}
