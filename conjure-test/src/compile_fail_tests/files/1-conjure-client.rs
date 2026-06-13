use conjure_macros::conjure_client;

#[conjure_client]
struct CustomService {}

#[conjure_client]
trait CustomService {
    fn example_method() -> Result<(), Error> {}
}

#[allow(dead_code)]
fn main() {}