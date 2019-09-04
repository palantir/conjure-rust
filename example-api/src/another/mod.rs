#[doc(inline)]
pub use self::different_package::DifferentPackage;
#[doc(inline)]
pub use self::test_service::{TestService, TestServiceClient, TestServiceResource};
pub mod different_package;
pub mod test_service;
