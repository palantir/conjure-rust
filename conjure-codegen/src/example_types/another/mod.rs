#[doc(inline)]
pub use self::different_package::DifferentPackage;
#[doc(inline)]
pub use self::test_service::{
    TestServiceClient, TestServiceAsyncClient, TestService, AsyncTestService,
    TestServiceEndpoints,
};
pub mod different_package;
pub mod test_service;
