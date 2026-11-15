//! A Rust crate to scaffold a GraphQL server with JWKS JWT auth.

#![deny(missing_docs)]

pub mod auth;

/// Returns a greeting for the given name.
///
/// # Examples
///
/// ```
/// use subgraph_kit::greeting;
///
/// assert_eq!(greeting("Carol"), "Hello Carol!");
/// ```
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_should_say_hello_when_given_a_name() {
        let result = greeting("Carol");
        assert_eq!(result, "Hello Carol!");
    }
}
