#[doc(inline)]
pub use self::different_package::DifferentPackage;
#[doc(inline)]
pub use self::test_service::{
    AsyncTestService, TestService, TestServiceAsyncClient, TestServiceClient, TestServiceService,
};
pub mod different_package;
pub mod test_service;
