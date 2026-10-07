pub mod command;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_not_empty() {
        assert_eq!(VERSION, "0.1.0");
    }
}
