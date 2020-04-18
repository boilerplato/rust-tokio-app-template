pub use self::error::{Error, ErrorExt, ResultExt};

pub mod constants;
mod error;
pub mod prelude;

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_foo() {
        assert_eq!(1, 1);
    }
}
